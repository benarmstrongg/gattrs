use std::collections::HashMap;
use zbus::dbus_proxy;
use zbus::fdo::Result;
use zbus::zvariant::{ObjectPath, Value};

#[dbus_proxy(
    name = "org.bluez.GattManager1",
    default_service = "org.bluez",
    default_path = "/org/bluez/hci0"
)]
pub trait GattManager {
    async fn register_application(
        &self,
        path: ObjectPath<'_>,
        options: HashMap<&str, Value<'_>>,
    ) -> Result<()>;

    async fn unregister_application(&self, path: ObjectPath<'_>) -> zbus::fdo::Result<()>;
}
