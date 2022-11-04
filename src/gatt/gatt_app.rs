use super::super::adv::{LEAdvertisement, LEAdvertisingManagerProxy};
use super::{GattManagerProxy, ServiceRegister};
use std::collections::HashMap;
use zbus::fdo::ObjectManager;
use zbus::zvariant::ObjectPath;
use zbus::Connection;

pub struct GattApplication {
    service_uuids: Vec<String>,
    should_advertise: bool,
    path: ObjectPath<'static>,
    bus: Connection,
}

impl GattApplication {
    pub async fn new() -> Self {
        let bus = Connection::system().await.unwrap();
        Self {
            service_uuids: vec![],
            should_advertise: false,
            path: ObjectPath::from_str_unchecked("/"),
            bus,
        }
    }

    pub fn on_bus(bus: Connection) -> Self {
        Self {
            service_uuids: vec![],
            should_advertise: false,
            path: ObjectPath::from_str_unchecked("/"),
            bus,
        }
    }

    pub async fn service(mut self, service: impl ServiceRegister) -> Self {
        self.service_uuids.push(service.get_uuid());
        let service_path = service.get_path(self.path.clone());
        let _res = service.register(self.bus.clone(), self.path.clone()).await;
        println!("GATT service {} registered", &service_path);
        self
    }

    pub fn advertise(mut self) -> Self {
        self.should_advertise = true;
        self
    }

    pub fn path(mut self, path: &'static str) -> Self {
        self.path = ObjectPath::from_str_unchecked(path);
        self
    }

    pub async fn serve(self) -> zbus::Result<()> {
        self.register_object_manager().await?;
        self.register_gatt_app().await?;
        self.register_advertisement().await?;
        Ok(())
    }

    async fn register_object_manager(&self) -> zbus::Result<()> {
        let object_manager = ObjectManager {};
        self.bus
            .object_server()
            .at(self.path.clone(), object_manager)
            .await?;
        // todo! handle error
        println!("Object manager registered");
        Ok(())
    }

    async fn register_gatt_app(&self) -> zbus::fdo::Result<()> {
        let gatt_manager = GattManagerProxy::new(&self.bus).await?;
        let res = gatt_manager
            .register_application(self.path.clone(), HashMap::new())
            .await;
        // todo! handle error
        println!("GATT application registered at {}", &self.path);
        res
    }

    async fn register_advertisement(&self) -> zbus::fdo::Result<()> {
        if self.should_advertise == true {
            let ad_manager = LEAdvertisingManagerProxy::new(&self.bus).await?;
            let ad = LEAdvertisement {
                service_uuids: self.service_uuids.clone(),
                ad_type: "peripheral".into(),
            };
            self.bus.object_server().at(self.path.clone(), ad).await?;
            // todo! handle error
            let res = ad_manager
                .register_advertisement(self.path.clone(), HashMap::new())
                .await;
            println!("Advertisement registered at {}", &self.path);
            return res;
        }
        Ok(())
    }
}
