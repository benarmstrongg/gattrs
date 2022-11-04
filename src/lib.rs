// sub modules exports
pub mod adv;
pub mod gatt;

// types exports
mod types;
pub use types::{GattReadResult, GattWriteResult};

// macros re-export
pub use macros::{gatt_characteristic, gatt_service};

// external crate re-exports
pub extern crate async_trait;
pub extern crate derivative;
pub extern crate zbus;
