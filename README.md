# gattrs

This crate provides proc macros for creating a Bluez GATT server, allowing a device to operate as a Bluetooth Low Energy (BLE) peripheral. It is a thin but opinionated wrapper around [zbus](https://docs.rs/zbus/latest/zbus/) crate using an object-oriented-like style to define GATT services and characteristics.

## Installation

Because `gattrs` is a GATT-specific wrapper around `zbus`, both crates must be listed as dependencies.

```toml
[dependencies]
gattrs = { git = "https://github.com/benarmstrongg/gattrs.git", branch = "main" }
zbus = { version = "2.2.0", default-features = false }
```

An async runtime is also required to use this crate. The examples in this repo use [tokio](https://docs.rs/tokio/latest/tokio/).

```toml
[dependencies]
# ...
tokio = { version = "1", features = ["full"] }
```

## Usage

A GATT application must expose at least one service containing at least one characteristic. The proc macros `gatt_characteristic` and `gatt_service` add the methods required for D-Bus interfaces `org.bluez.GattCharacteristic1` and `org.bluez.GattService1`. Services and characteristics require UUIDs to identify themselves to client devices.

See [example directory](https://github.com/benarmstrongg/gattrs/tree/main/example/counter) for full code examples.

### Characteristics

A GATT characteristic is an individual property on a GATT service. In addition to UUID, a characteristic declaration must include a list of `flags`, which can be one of `read`, `write`, and `notify`.

#### Read

Characteristics with the `read` flag must declare a method `read` returning a `GattReadResult`.

```rust
#[gatt_characteristic(
    uuid = "42000000-0000-0000-0000-000000000001",
    flags = ["read"]
)]
struct CountCharacteristic {
    count: u8,
}

impl CountCharacteristic {
    async fn read(&self) -> gattrs::GattReadResult {
        Ok(vec![self.count])
    }
}
```

#### Write

Characteristics with the `write` flag must declare a method `write`, taking raw bytes as input and returing a `GattWriteResult`.

```rust
#[gatt_characteristic(
    uuid = "42000000-0000-0000-0000-000000000001",
    flags = ["write"]
)]
struct CountCharacteristic {
    count: u8,
}

impl CountCharacteristic {
    async fn write(&mut self, val: &[u8]) -> gattrs::GattWriteResult {
        let count = val.first().unwrap_or(&self.count).to_owned();
        self.count = count;
        Ok(())
    }
}
```

#### Notify

Characteristics with the `notify` flag don't need to declare any special methods. Instead, they gain access to the `notify` method, which brodcasts the characteristics `value` property to subscribed clients.

```rust
#[gatt_characteristic(
    uuid = "42000000-0000-0000-0000-000000000001",
    flags = ["notify"]
)]
struct CountCharacteristic {}

impl CountCharacteristic {
    async fn hello(&mut self) -> zbus::fdo::Result<()> {
        self.value = "hello world".as_bytes().to_vec();
        self.notify()
    }
}
```

Characteristics with both `write` and `notify` flags will emit notifications on writes as well.

### Services

A service is an interface exposing one or more characteristics to client devices. The service declaration must include a UUID.

```rust
#[gatt_service(uuid = "42000000-0000-0000-0000-000000000000")]
struct CounterService {}
```

A GATT service is only responsible for exposing characteristics, so only a `get_characteristics` method is required, returning `Vec<impl CharacteristicRegister>`. `CharacteristicRegister` trait is automatically applied to all structs with the `gatt_characteristic` proc macro.

```rust
impl CounterService {
    fn get_characteristics(&self) -> Vec<impl CharacteristicRegister> {
        vec![CountCharacteristic::default()]
    }
}
```

### Application

A GATT application is a collection of GATT services registered by the D-Bus interface `org.bluez.GattManager1`.

The `GattApplication` struct provides 2 ways for creating a GATT application.

```rust
// Using the default system message bus
GattApplication::new().await

// Using a session bus
let session_bus = zbus::Connection::session().await.unwrap();
GattApplication::on_bus(session_bus)
```

The `GattApplication` struct's builder methods can then be used to register services/advertisement and set the application path.

```rust
GattApplication::new()
    // Set path
    .path("/MyApp")
    .await
    // Register services
    .service(MyService1::default())
    .await
    .service(MyService2::new("hello world"))
    .await
    // Register advertisement
    .advertise()
    // Serve application
    .serve();
```

### Advertisements

In order to expose your GATT application to BLE clients, an advertisement matching D-Bus interface `org.bluez.LEAdvertisement1` must be registered at the same path as your GATT application. The `LEAdvertisement` contains a list of GATT service UUIDs to be exposed to clients. Registering an advertisement can be done automatically using the `.advertise()` method on `GattApplication` struct.
