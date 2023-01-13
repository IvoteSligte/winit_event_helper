use winit::event::Event;

use crate::{
    callback_data::CallbackData, device_callbacks::DeviceCallbacks,
    general_callbacks::GeneralCallbacks, window_callbacks::WindowCallbacks,
};

/// A struct containing all callback functions, each corresponding to a winit event
#[derive(Clone)]
pub struct Callbacks<D> {
    general: GeneralCallbacks<D>,
    device: DeviceCallbacks<D>,
    window: WindowCallbacks<D>,
}

impl<D> Default for Callbacks<D> {
    fn default() -> Self {
        Self {
            general: Default::default(),
            device: Default::default(),
            window: Default::default(),
        }
    }
}

impl<D> Callbacks<D> {
    pub fn update<'a, E>(&self, data: &mut CallbackData<D>, event: &Event<'a, E>) -> bool {
        match event {
            Event::DeviceEvent { event, .. } => {
                self.device.update(data, event);
            }
            Event::WindowEvent { event, .. } => {
                self.window.update(data, event);
            }
            Event::MainEventsCleared => {
                return true;
            }
            _ => self.general.update(data, event),
        }
        false
    }
}
