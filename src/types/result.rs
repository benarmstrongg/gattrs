pub type GattReadResult = zbus::fdo::Result<Vec<u8>>;
pub type GattWriteResult = zbus::fdo::Result<()>;
pub type GattResult<T> = zbus::fdo::Result<T>;
