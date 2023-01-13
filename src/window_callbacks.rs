use std::path::PathBuf;

use ahash::AHashMap;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        AxisId, Ime, KeyboardInput, ModifiersState, MouseButton, MouseScrollDelta, Touch,
        TouchPhase, VirtualKeyCode, WindowEvent,
    },
    window::Theme,
};

use crate::{
    callback_data::CallbackData,
    definitions::{Quit, CB, CBI},
    input::{element_state_callbacks, State},
};

#[derive(Clone)]
/// A struct containing all window callbacks.
pub struct WindowCallbacks<D> {
    pub(crate) axis_motion: CBI<D, (AxisId, f64)>,
    pub(crate) close_requested: CB<D>,
    pub(crate) cursor_entered: CB<D>,
    pub(crate) cursor_left: CB<D>,
    pub(crate) cursor_moved: CBI<D, PhysicalPosition<f64>>,
    pub(crate) destroyed: CB<D>,
    pub(crate) dropped_file: CBI<D, PathBuf>,
    pub(crate) focused: CBI<D, bool>,
    pub(crate) hovered_file: CBI<D, PathBuf>,
    pub(crate) hovered_file_canceled: CB<D>,
    pub(crate) ime: CBI<D, Ime>,
    pub(crate) keyboard_input_any: CBI<D, (VirtualKeyCode, State)>,
    pub(crate) keyboard_input: AHashMap<(VirtualKeyCode, State), CB<D>>,
    pub(crate) modifiers_changed: CBI<D, ModifiersState>,
    pub(crate) mouse_input_any: CBI<D, (MouseButton, State)>,
    pub(crate) mouse_input: AHashMap<(MouseButton, State), CB<D>>,
    pub(crate) mouse_wheel: CBI<D, (MouseScrollDelta, TouchPhase)>,
    pub(crate) moved: CBI<D, PhysicalPosition<i32>>,
    pub(crate) occluded: CBI<D, bool>,
    pub(crate) received_character: CBI<D, char>,
    pub(crate) resized: CBI<D, PhysicalSize<u32>>,
    pub(crate) scale_factor_changed: CBI<D, f64>,
    pub(crate) theme_changed: CBI<D, Theme>,
    pub(crate) touch: CBI<D, Touch>,
    pub(crate) touchpad_pressure: CBI<D, (f32, i64)>,
}

impl<D> Default for WindowCallbacks<D> {
    fn default() -> Self {
        Self {
            axis_motion: |_, _| {},
            close_requested: |_| {},
            cursor_entered: |_| {},
            cursor_left: |_| {},
            cursor_moved: |_, _| {},
            destroyed: |_| {},
            dropped_file: |_, _| {},
            focused: |_, _| {},
            hovered_file: |_, _| {},
            hovered_file_canceled: |_| {},
            ime: |_, _| {},
            keyboard_input_any: |_, _| {},
            keyboard_input: AHashMap::new(),
            modifiers_changed: |_, _| {},
            mouse_input_any: |_, _| {},
            mouse_input: AHashMap::new(),
            mouse_wheel: |_, _| {},
            moved: |_, _| {},
            occluded: |_, _| {},
            received_character: |_, _| {},
            resized: |_, _| {},
            scale_factor_changed: |_, _| {},
            theme_changed: |_, _| {},
            touch: |_, _| {},
            touchpad_pressure: |_, _| {},
        }
    }
}

impl<D> WindowCallbacks<D> {
    pub fn axis_motion(&mut self, callback: CBI<D, (u32, f64)>) {
        self.axis_motion = callback;
    }

    pub fn close_requested(&mut self, callback: CB<D>) {
        self.close_requested = callback;
    }

    pub fn cursor_entered(&mut self, callback: CB<D>) {
        self.cursor_entered = callback;
    }

    pub fn cursor_left(&mut self, callback: CB<D>) {
        self.cursor_left = callback;
    }

    pub fn cursor_moved(&mut self, callback: CBI<D, PhysicalPosition<f64>>) {
        self.cursor_moved = callback;
    }

    pub fn destroyed(&mut self, callback: CB<D>) {
        self.destroyed = callback;
    }

    pub fn dropped_file(&mut self, callback: CBI<D, PathBuf>) {
        self.dropped_file = callback;
    }

    pub fn focused(&mut self, callback: CBI<D, bool>) {
        self.focused = callback;
    }

    pub fn hovered_file(&mut self, callback: CBI<D, PathBuf>) {
        self.hovered_file = callback;
    }

    pub fn hovered_file_canceled(&mut self, callback: CB<D>) {
        self.hovered_file_canceled = callback;
    }

    pub fn ime(&mut self, callback: CBI<D, Ime>) {
        self.ime = callback;
    }

    /// Callback is called when an event with the given key and state is received
    pub fn keyboard_input(&mut self, key: VirtualKeyCode, state: State, callback: CB<D>) {
        self.keyboard_input.insert((key, state), callback);
    }

    /// Callback is called for any key/state combination
    pub fn keyboard_input_any(&mut self, callback: CBI<D, (VirtualKeyCode, State)>) {
        self.keyboard_input_any = callback;
    }

    pub fn modifiers_changed(&mut self, callback: CBI<D, ModifiersState>) {
        self.modifiers_changed = callback;
    }

    /// Callback is called when an event with the given button and state is received
    pub fn mouse_input(&mut self, button: MouseButton, state: State, callback: CB<D>) {
        self.mouse_input.insert((button, state), callback);
    }

    /// Callback is called for any button/state combination
    pub fn mouse_input_any(&mut self, callback: CBI<D, (MouseButton, State)>) {
        self.mouse_input_any = callback;
    }

    pub fn mouse_wheel(&mut self, callback: CBI<D, (MouseScrollDelta, TouchPhase)>) {
        self.mouse_wheel = callback;
    }

    pub fn moved(&mut self, callback: CBI<D, PhysicalPosition<i32>>) {
        self.moved = callback;
    }

    pub fn occluded(&mut self, callback: CBI<D, bool>) {
        self.occluded = callback;
    }

    pub fn received_character(&mut self, callback: CBI<D, char>) {
        self.received_character = callback;
    }

    pub fn resized(&mut self, callback: CBI<D, PhysicalSize<u32>>) {
        self.resized = callback;
    }

    pub fn scale_factor_changed(&mut self, callback: CBI<D, f64>) {
        self.scale_factor_changed = callback;
    }

    pub fn theme_changed(&mut self, callback: CBI<D, Theme>) {
        self.theme_changed = callback;
    }

    pub fn touch(&mut self, callback: CBI<D, Touch>) {
        self.touch = callback;
    }

    pub fn touchpad_pressure(&mut self, callback: CBI<D, (f32, i64)>) {
        self.touchpad_pressure = callback;
    }

    #[inline]
    pub fn update(&self, callback_data: &mut CallbackData<D>, event: &WindowEvent) {
        match event {
            &WindowEvent::CloseRequested => (self.close_requested)(callback_data),
            &WindowEvent::CursorEntered { .. } => (self.cursor_entered)(callback_data),
            &WindowEvent::CursorLeft { .. } => (self.cursor_left)(callback_data),
            &WindowEvent::CursorMoved { position, .. } => {
                (self.cursor_moved)(callback_data, position);
            }
            &WindowEvent::Focused(focused) => {
                callback_data.focused = focused;
                (self.focused)(callback_data, focused);
            }
            &WindowEvent::MouseInput { button, state, .. } => {
                #[cfg(not(feature = "save_device_inputs"))]
                callback_data.update_buttons(button, state);

                element_state_callbacks(state, callback_data.pressed(button), |state| {
                    (self.mouse_input_any)(callback_data, (button, state));
                    if let Some(func) = self.mouse_input.get(&(button, state)) {
                        func(callback_data);
                    }
                });
            }
            &WindowEvent::Moved(position) => {
                (self.moved)(callback_data, position);
            }
            &WindowEvent::Resized(size) => {
                (self.resized)(callback_data, size);
            }
            &WindowEvent::Destroyed => {
                callback_data.quit.insert(Quit::WINDOW_DESTROYED);
                (self.destroyed)(callback_data);
            }
            WindowEvent::DroppedFile(path) => {
                (self.dropped_file)(callback_data, path.clone());
            }
            WindowEvent::HoveredFile(path) => {
                (self.hovered_file)(callback_data, path.clone());
            }
            &WindowEvent::HoveredFileCancelled => {
                (self.hovered_file_canceled)(callback_data);
            }
            &WindowEvent::ReceivedCharacter(codepoint) => {
                #[cfg(not(feature = "save_device_inputs"))]
                callback_data.text.push(codepoint);
                (self.received_character)(callback_data, codepoint);
            }
            &WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    },
                ..
            } => {
                if let Some(key) = virtual_keycode {
                    #[cfg(not(feature = "save_device_inputs"))]
                    callback_data.update_keys(key, state);

                    element_state_callbacks(state, callback_data.pressed(key), |state| {
                        (self.keyboard_input_any)(callback_data, (key, state));
                        if let Some(func) = self.keyboard_input.get(&(key, state)) {
                            func(callback_data);
                        }
                    });
                }
            }
            &WindowEvent::ModifiersChanged(modifiers) => {
                (self.modifiers_changed)(callback_data, modifiers);
            }
            WindowEvent::Ime(ime) => {
                (self.ime)(callback_data, ime.clone());
            }
            &WindowEvent::TouchpadPressure {
                pressure, stage, ..
            } => {
                (self.touchpad_pressure)(callback_data, (pressure, stage));
            }
            &WindowEvent::AxisMotion { axis, value, .. } => {
                (self.axis_motion)(callback_data, (axis, value));
            }
            &WindowEvent::Touch(touch) => {
                (self.touch)(callback_data, touch);
            }
            &WindowEvent::ThemeChanged(theme) => {
                (self.theme_changed)(callback_data, theme);
            }
            &WindowEvent::Occluded(occluded) => {
                (self.occluded)(callback_data, occluded);
            }
            &WindowEvent::MouseWheel { delta, phase, .. } => {
                #[cfg(not(feature = "save_device_inputs"))]
                {
                    callback_data.mouse_wheel = delta;
                }
                (self.mouse_wheel)(callback_data, (delta, phase));
            }
            &WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                (self.scale_factor_changed)(callback_data, scale_factor);
            }
        }
    }
}
