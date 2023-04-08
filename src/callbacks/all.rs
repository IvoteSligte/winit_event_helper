use winit::event::Event;

#[cfg(feature = "unique_devices")]
use winit::event::DeviceId;

#[cfg(feature = "unique_windows")]
use winit::event::WindowId;

#[cfg(any(feature = "unique_windows", feature = "unique_devices"))]
use ahash::AHashMap;

#[cfg(any(feature = "unique_windows", feature = "unique_devices"))]
use crate::default_ahashmap::DefaultAHashMap;

use crate::{definitions::CallbackCallable, EventHelper};

use super::{
    device::{DeviceCallbackData, DeviceCallbacks},
    general::{GeneralCallbackData, GeneralCallbacks},
    window::{WindowCallbackData, WindowCallbacks},
};

#[derive(Clone, Default)]
/// Struct that holds all the callbacks and accompanying callback data as well as a user-supplied `user_data` struct.
///
/// This struct is passed to callback functions.
pub struct CallbackData {
    pub general: GeneralCallbackData,
    #[cfg(not(feature = "unique_windows"))]
    pub window: WindowCallbackData,
    #[cfg(feature = "unique_windows")]
    pub windows: DefaultAHashMap<WindowId, WindowCallbackData>,
    #[cfg(not(feature = "unique_devices"))]
    pub device: DeviceCallbackData,
    #[cfg(feature = "unique_devices")]
    pub devices: DefaultAHashMap<DeviceId, DeviceCallbackData>,
}

impl CallbackData {
    /// Calls the callbacks associated with this struct and child structs.
    ///
    /// This is called once internally after every step, but the user can call it manually.
    pub fn call_callbacks<D>(self, event_helper: &mut EventHelper<D>, callbacks: &Callbacks<D>) {
        self.general
            .call_callbacks(event_helper, &callbacks.general);

        self.window.call_callbacks(event_helper, &callbacks.window);

        #[cfg(feature = "unique_windows")]
        self.windows
            .iter()
            .filter_map(|(window_id, window_callback_data)| {
                callbacks
                    .windows
                    .map
                    .get(window_id)
                    .zip(Some(window_callback_data))
            })
            .for_each(|(window_callbacks, window_callback_data)| {
                window_callback_data.call_callbacks(event_helper, &window_callbacks);
            });

        self.device.call_callbacks(event_helper, &callbacks.device);

        #[cfg(feature = "unique_devices")]
        event_helper
            .callback_data
            .devices
            .iter()
            .filter_map(|(device_id, device_callback_data)| {
                callbacks
                    .devices
                    .map
                    .get(device_id)
                    .zip(Some(device_callback_data))
            })
            .for_each(|(device_callbacks, device_callback_data)| {
                device_callback_data.call_callbacks(event_helper, &device_callbacks);
            });
    }

    pub fn clear(&mut self) {
        #[cfg(not(feature = "unique_windows"))]
        self.window.clear();

        #[cfg(feature = "unique_windows")]
        self.windows
            .values_mut()
            .for_each(WindowCallbackData::clear);

        #[cfg(not(feature = "unique_devices"))]
        self.device.clear();

        #[cfg(feature = "unique_devices")]
        self.devices
            .values_mut()
            .for_each(DeviceCallbackData::clear);
    }

    #[allow(unused_variables)]
    pub fn update<'a, E>(&mut self, event: &Event<'a, E>) {
        match event {
            Event::WindowEvent { event, window_id } => {
                #[cfg(not(feature = "unique_windows"))]
                {
                    self.window.update(event);
                }
                #[cfg(feature = "unique_windows")]
                {
                    let window = self.windows.entry(*window_id).or_default();
                    window.update(event);
                }
            }
            Event::DeviceEvent { event, device_id } => {
                #[cfg(not(feature = "unique_devices"))]
                {
                    self.device.update(event);
                }
                #[cfg(feature = "unique_devices")]
                {
                    let device = self.devices.entry(*device_id).or_default();
                    device.update(event);
                }
            }
            _ => self.general.update(event),
        }
    }
}

#[derive(Clone)]
/// A collection of callbacks. This is the only `callbacks` type struct you should use directly.
pub struct Callbacks<D> {
    pub general: GeneralCallbacks<D>,
    #[cfg(not(feature = "unique_windows"))]
    pub window: WindowCallbacks<D>,
    #[cfg(feature = "unique_windows")]
    pub windows: DefaultAHashMap<WindowId, WindowCallbacks<D>>,
    #[cfg(not(feature = "unique_devices"))]
    pub device: DeviceCallbacks<D>,
    #[cfg(feature = "unique_devices")]
    pub devices: DefaultAHashMap<DeviceId, DeviceCallbacks<D>>,
}

impl<D> Default for Callbacks<D> {
    fn default() -> Self {
        Self {
            general: Default::default(),
            window: Default::default(),
            #[cfg(feature = "unique_windows")]
            windows: Default::default(),
            device: Default::default(),
            #[cfg(feature = "unique_devices")]
            devices: Default::default(),
        }
    }
}

impl<D> Callbacks<D> {
    pub fn empty() -> Self {
        Self::default()
    }
}
