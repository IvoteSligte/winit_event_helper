use ahash::AHashMap;
use winit::event::{
    AxisId, ButtonId, DeviceEvent, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode,
};

use crate::{
    callback_data::CallbackData,
    definitions::{CB, CBI},
    input::{element_state_callbacks, State},
};

#[derive(Clone)]
/// A struct containing all the device callbacks.
pub struct DeviceCallbacks<D> {
    pub(crate) added: CB<D>,
    pub(crate) button_any: CBI<D, (ButtonId, State)>,
    pub(crate) button: AHashMap<(ButtonId, State), CB<D>>,
    pub(crate) key_any: CBI<D, (VirtualKeyCode, State)>,
    pub(crate) key: AHashMap<(VirtualKeyCode, State), CB<D>>,
    pub(crate) motion: CBI<D, (AxisId, f64)>,
    pub(crate) mouse_motion: CBI<D, (f64, f64)>,
    pub(crate) mouse_wheel: CBI<D, MouseScrollDelta>,
    pub(crate) removed: CB<D>,
    pub(crate) text: CBI<D, char>,
}

impl<D> Default for DeviceCallbacks<D> {
    fn default() -> Self {
        Self {
            added: |_| {},
            button_any: |_, _| {},
            button: AHashMap::new(),
            key_any: |_, _| {},
            key: AHashMap::new(),
            motion: |_, _| {},
            mouse_motion: |_, _| {},
            mouse_wheel: |_, _| {},
            removed: |_| {},
            text: |_, _| {},
        }
    }
}

impl<D> DeviceCallbacks<D> {
    pub fn added(&mut self, callback: CB<D>) {
        self.added = callback;
    }

    /// Callback is called when an event with the given button and state is received
    pub fn button(&mut self, button: u32, state: State, callback: CB<D>) {
        self.button.insert((button, state), callback);
    }

    /// Callback is called for any button/state combination
    pub fn button_any(&mut self, callback: CBI<D, (u32, State)>) {
        self.button_any = callback;
    }

    /// Callback is called when an event with the given key and state is received
    pub fn key(&mut self, key: VirtualKeyCode, state: State, callback: CB<D>) {
        self.key.insert((key, state), callback);
    }

    /// Callback is called for any key/state combination
    pub fn key_any(&mut self, callback: CBI<D, (VirtualKeyCode, State)>) {
        self.key_any = callback;
    }

    pub fn motion(&mut self, callback: CBI<D, (u32, f64)>) {
        self.motion = callback;
    }

    pub fn mouse_motion(&mut self, callback: CBI<D, (f64, f64)>) {
        self.mouse_motion = callback;
    }

    pub fn mouse_wheel(&mut self, callback: CBI<D, MouseScrollDelta>) {
        self.mouse_wheel = callback;
    }

    pub fn removed(&mut self, callback: CB<D>) {
        self.removed = callback;
    }

    pub fn text(&mut self, callback: CBI<D, char>) {
        self.text = callback;
    }

    #[inline]
    pub fn update(&self, data: &mut CallbackData<D>, event: &DeviceEvent) {
        match event {
            &DeviceEvent::Key(KeyboardInput {
                virtual_keycode,
                state,
                ..
            }) => {
                if let Some(key) = virtual_keycode {
                    #[cfg(feature = "save_device_inputs")]
                    data.update_keys(key, state);

                    element_state_callbacks(state, data.pressed(key), |state| {
                        (self.key_any)(data, (key, state));
                        if let Some(func) = self.key.get(&(key, state)) {
                            func(data);
                        }
                    });
                }
            }
            &DeviceEvent::MouseMotion { delta } => {
                (self.mouse_motion)(data, delta);
            }
            &DeviceEvent::Added => {
                (self.added)(data);
            }
            &DeviceEvent::Removed => {
                (self.removed)(data);
            }
            &DeviceEvent::Motion { axis, value } => {
                (self.motion)(data, (axis, value));
            }
            &DeviceEvent::Button { button, state } => {
                let mouse_button = match button {
                    0 => MouseButton::Left,
                    1 => MouseButton::Middle,
                    2 => MouseButton::Right,
                    _ => MouseButton::Other(button as u16),
                };
                #[cfg(feature = "save_device_inputs")]
                data.update_buttons(mouse_button, state);

                element_state_callbacks(state, data.pressed(mouse_button), |state| {
                    (self.button_any)(data, (button, state));
                    if let Some(func) = self.button.get(&(button, state)) {
                        func(data);
                    }
                });
            }
            &DeviceEvent::Text { codepoint } => {
                #[cfg(feature = "save_device_inputs")]
                data.text.push(codepoint);
                (self.text)(data, codepoint);
            }
            &DeviceEvent::MouseWheel { delta } => {
                (self.mouse_wheel)(data, delta);
            }
        }
    }
}
