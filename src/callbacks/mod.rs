//! Contains collections of data used for callbacks and the collections callbacks themselves.

pub mod all;
pub mod device;
pub mod general;
pub mod window;

pub use all::CallbackData;
pub use all::Callbacks;

pub use general::GeneralCallbackData;
pub use general::GeneralCallbacks;

pub use device::DeviceCallbackData;
pub use device::DeviceCallbacks;

pub use window::WindowCallbackData;
pub use window::WindowCallbacks;
