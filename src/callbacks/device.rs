use winit::event::{AxisId, DeviceEvent, KeyboardInput, MouseButton};

use crate::{
    create_callbacks,
    definitions::{LineDelta, PixelDelta},
    input::data::InputData,
};

create_callbacks! {
    /// A collection of data used for [DeviceEvent] callbacks.
    ///
    /// [DeviceCallbacks] holds the callbacks themselves.
    pub struct DeviceCallbackData: DeviceCallbacks<D> {
        clr boo pub added: bool,
        clr boo pub removed: bool,
        clr opt pub mouse_motion: (f64, f64),
        clr vec pub text: char,
        clr vec pub mouse_wheel: (LineDelta, PixelDelta),
        clr vec pub motion: (AxisId, f64),
        clr cus pub inputs: InputData,
    }
}

impl DeviceCallbackData {
    pub fn update(&mut self, event: &DeviceEvent) {
        match event {
            &DeviceEvent::Key(KeyboardInput {
                virtual_keycode,
                scancode,
                state,
                ..
            }) => {
                self.inputs.update(scancode, state);
                if let Some(key) = virtual_keycode {
                    self.inputs.update(key, state);
                }
            }
            &DeviceEvent::Button { button, state } => {
                self.inputs.update(mouse_button_from_u32(button), state);
            }
            &DeviceEvent::Text { codepoint } => {
                self.text.push(codepoint);
            }
            &DeviceEvent::MouseWheel { delta } => {
                self.mouse_wheel.push((
                    delta.try_into().unwrap_or_default(),
                    delta.try_into().unwrap_or_default(),
                ));
            }
            &DeviceEvent::Motion { axis, value } => {
                self.motion.push((axis, value));
            }
            DeviceEvent::Added => self.added = true,
            DeviceEvent::Removed => self.removed = true,
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                let (x, y) = self.mouse_motion.get_or_insert(Default::default());
                *x += dx;
                *y += dy;
            }
        }
    }
}

fn mouse_button_from_u32(button: u32) -> MouseButton {
    match button {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        _ => MouseButton::Other(button as u16),
    }
}
