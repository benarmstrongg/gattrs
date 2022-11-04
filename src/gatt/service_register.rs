use async_trait::async_trait;
use zbus::zvariant::ObjectPath;
use zbus::{Connection, Result};

#[async_trait]
pub trait ServiceRegister {
    fn get_uuid(&self) -> String;

    fn get_path(&self, base_path: ObjectPath<'static>) -> ObjectPath<'static>;

    async fn register(self, bus: Connection, app_path: ObjectPath<'static>) -> Result<bool>;
}
