use zbus::dbus_interface;

pub struct LEAdvertisement {
    pub ad_type: String,
    pub service_uuids: Vec<String>,
}

#[dbus_interface(name = "org.bluez.LEAdvertisement1")]
impl LEAdvertisement {
    #[dbus_interface(property, name = "Type")]
    fn ad_type(&self) -> &str {
        &self.ad_type
    }

    #[dbus_interface(property, name = "ServiceUUIDs")]
    fn service_uuids(&self) -> Vec<String> {
        self.service_uuids.clone()
    }
}
