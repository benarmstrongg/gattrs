use gattrs::{
    gatt::{CharacteristicRegister, GattApplication},
    gatt_characteristic, gatt_service,
};

#[gatt_service(uuid = "42000000-0000-0000-0000-000000000000")]
struct CounterService {}

impl CounterService {
    fn get_characteristics(&self) -> Vec<impl CharacteristicRegister> {
        vec![CountCharacteristic::default()]
    }
}

#[gatt_characteristic(
    uuid = "42000000-0000-0000-0000-000000000001",
    flags = ["read", "write", "notify"]
)]
struct CountCharacteristic {
    count: u8,
}

impl CountCharacteristic {
    async fn read(&self) -> gattrs::GattReadResult {
        Ok(vec![self.count])
    }

    async fn write(&mut self, val: &[u8]) -> gattrs::GattWriteResult {
        let count = val.first().unwrap_or(&self.count).to_owned();
        self.count = count;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> gattrs::zbus::Result<()> {
    GattApplication::new()
        .await
        .service(CounterService::default())
        .await
        .advertise()
        .serve()
        .await?;

    Ok(())
}
