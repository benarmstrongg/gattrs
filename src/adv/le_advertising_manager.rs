use std::collections::HashMap;
use zbus::dbus_proxy;
use zbus::fdo::Result;
use zbus::zvariant::{ObjectPath, Value};

#[dbus_proxy(
    name = "org.bluez.LEAdvertisingManager1",
    default_service = "org.bluez",
    default_path = "/org/bluez/hci0"
)]
pub trait LEAdvertisingManager {
    async fn register_advertisement(
        &self,
        path: ObjectPath<'_>,
        options: HashMap<&str, Value<'_>>,
    ) -> Result<()>;

    async fn unregister_advertisement(&self, path: ObjectPath<'_>) -> Result<()>;
}
