//! Keyboard and mouse inputs are combined and moved into the [InputData] struct.
//!
//! This can be accessed as field `inputs` on the
//! [WindowCallbackData](crate::callbacks::WindowCallbackData) and [DeviceCallbackData](crate::callbacks::DeviceCallbackData) structs.
//!
//! Callbacks are collected in [InputCallbacks],
//! which can be accessed via the `callbacks` field on the [EventHelper](crate::EventHelper) struct.

pub mod callbacks;
pub mod data;

pub use callbacks::InputCallbacks;
pub use data::InputData;
pub use data::InputDataWithId;
