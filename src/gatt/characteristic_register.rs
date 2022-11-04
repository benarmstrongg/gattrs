use async_trait::async_trait;
use zbus::zvariant::ObjectPath;
use zbus::Connection;

#[async_trait]
pub trait CharacteristicRegister {
    fn get_path(&self, base_path: ObjectPath<'static>) -> ObjectPath<'static>;

    async fn register(
        mut self,
        bus: Connection,
        service_path: ObjectPath<'static>,
    ) -> zbus::Result<bool>;
}
