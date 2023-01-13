use std::time::Duration;

use winit::event::{ElementState, ModifiersState, MouseButton, MouseScrollDelta, VirtualKeyCode};

use crate::{
    definitions::Quit,
    input::{filter_keyboard_keys, filter_mouse_buttons, into_input_iter, GeneralInput, Input},
};

pub struct CallbackData<D> {
    pub user_data: D,
    pub inputs: Input<GeneralInput>,
    pub modifiers: ModifiersState,
    pub mouse_wheel: MouseScrollDelta,
    pub focused: bool,
    pub text: Vec<char>,
    pub quit: Quit,
}

impl<D> CallbackData<D> {
    pub fn new(user_data: D) -> Self {
        Self {
            user_data,
            inputs: Input::default(),
            modifiers: ModifiersState::empty(),
            mouse_wheel: MouseScrollDelta::LineDelta(0.0, 0.0),
            focused: true,
            text: vec![],
            quit: Quit::empty(),
        }
    }
}

impl<D> CallbackData<D> {
    pub fn update(&mut self) {
        self.text.clear();
        self.mouse_wheel = MouseScrollDelta::LineDelta(0.0, 0.0);
        self.inputs.clear();
    }

    pub fn update_buttons(&mut self, value: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.inputs.press(value.into());
            }
            ElementState::Released => {
                self.inputs.release(value.into());
            }
        }
    }

    pub fn update_keys(&mut self, value: VirtualKeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.inputs.press(value.into());
            }
            ElementState::Released => {
                self.inputs.press(value.into());
            }
        }
    }

    pub fn just_pressed_iter(&self) -> impl ExactSizeIterator<Item = &GeneralInput> {
        self.inputs.just_pressed_iter()
    }

    pub fn just_released_iter(&self) -> impl ExactSizeIterator<Item = &GeneralInput> {
        self.inputs.just_released_iter()
    }

    pub fn key_just_pressed_iter(&self) -> impl Iterator<Item = &VirtualKeyCode> {
        filter_keyboard_keys(self.just_pressed_iter())
    }

    pub fn button_just_pressed_iter(&self) -> impl Iterator<Item = &MouseButton> {
        filter_mouse_buttons(self.just_pressed_iter())
    }

    pub fn key_just_released_iter(&self) -> impl Iterator<Item = &VirtualKeyCode> {
        filter_keyboard_keys(self.just_released_iter())
    }

    pub fn button_just_released_iter(&self) -> impl Iterator<Item = &MouseButton> {
        filter_mouse_buttons(self.just_released_iter())
    }

    pub fn pressed_iter(&self) -> impl ExactSizeIterator<Item = &GeneralInput> {
        self.inputs.pressed_iter()
    }

    pub fn button_pressed_iter(&self) -> impl Iterator<Item = &MouseButton> {
        filter_mouse_buttons(self.pressed_iter())
    }

    pub fn pressed_for<T: Into<GeneralInput>>(&self, value: T) -> Option<Duration> {
        self.inputs.pressed_for(value.into())
    }

    pub fn pressed<T: Into<GeneralInput>>(&self, value: T) -> bool {
        self.inputs.pressed(value.into())
    }

    pub fn pressed_any<T: Into<GeneralInput>>(&self, values: impl IntoIterator<Item = T>) -> bool {
        self.inputs.pressed_any(into_input_iter(values))
    }

    pub fn pressed_all<T: Into<GeneralInput>>(&self, values: impl IntoIterator<Item = T>) -> bool {
        self.inputs.pressed_all(into_input_iter(values))
    }

    pub fn key_pressed_iter(&self) -> impl Iterator<Item = &VirtualKeyCode> {
        filter_keyboard_keys(self.pressed_iter())
    }

    pub fn just_pressed<T: Into<GeneralInput>>(&self, value: T) -> bool {
        self.inputs.just_pressed(value.into())
    }

    pub fn just_released<T: Into<GeneralInput>>(&self, value: T) -> bool {
        self.inputs.just_released(value.into())
    }

    pub fn just_pressed_any<T: Into<GeneralInput>>(
        &self,
        values: impl IntoIterator<Item = T>,
    ) -> bool {
        self.inputs
            .just_pressed_any(values.into_iter().map(|v| v.into()))
    }

    pub fn just_pressed_all<T: Into<GeneralInput>>(
        &self,
        values: impl IntoIterator<Item = T>,
    ) -> bool {
        self.inputs.just_pressed_all(into_input_iter(values))
    }

    pub fn just_released_any<T: Into<GeneralInput>>(
        &self,
        values: impl IntoIterator<Item = T>,
    ) -> bool {
        self.inputs.just_released_any(into_input_iter(values))
    }

    pub fn just_released_all<T: Into<GeneralInput>>(
        &self,
        values: impl IntoIterator<Item = T>,
    ) -> bool {
        self.inputs.just_released_all(into_input_iter(values))
    }

    pub fn just_pressed_combination<T: Into<GeneralInput>>(
        &self,
        modifiers: ModifiersState,
        values: impl IntoIterator<Item = T>,
    ) -> bool {
        self.just_pressed_all(values) && self.modifiers.contains(modifiers)
    }

    pub fn pressed_combination<T: Into<GeneralInput>>(
        &self,
        modifiers: ModifiersState,
        values: impl IntoIterator<Item = T>,
    ) -> bool {
        self.pressed_all(values) && self.modifiers.contains(modifiers)
    }

    pub fn just_released_combination<T: Into<GeneralInput>>(&self, modifiers: ModifiersState, values: impl IntoIterator<Item = T>) -> bool {
        self.just_released_all(values) && self.modifiers.contains(modifiers)
    }

    /// Returns true if the window is focused
    pub fn focused(&self) -> bool {
        self.focused
    }

    /// Returns true if any alt key is pressed
    pub fn pressed_alt(&self) -> bool {
        self.modifiers.alt()
    }

    /// Returns true if any ctrl key is pressed
    pub fn pressed_ctrl(&self) -> bool {
        self.modifiers.ctrl()
    }

    /// Returns true if the logo key is pressed
    pub fn pressed_logo(&self) -> bool {
        self.modifiers.logo()
    }

    /// Returns true if any shift key is pressed
    pub fn pressed_shift(&self) -> bool {
        self.modifiers.shift()
    }

    /// Makes quit_requested(Quit::USER_REQUESTED) return true during the next step
    pub fn quit(&mut self) {
        self.quit.insert(Quit::USER_REQUESTED);
    }

    /// Returns true if a quit request of the given request type has been received
    pub fn quit_requested(&self, request_type: Quit) -> bool {
        self.quit.intersects(request_type)
    }

    /// Returns the unicode characters received in the last step in the order received
    pub fn text(&self) -> Vec<char> {
        self.text.clone()
    }
}