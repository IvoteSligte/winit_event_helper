use std::{
    ops::{Deref, DerefMut},
    time::{Duration, Instant},
};

use ahash::{AHashMap, AHashSet};
use winit::event::{DeviceId, ElementState, MouseButton, VirtualKeyCode};

use crate::{
    default_ahashmap::DefaultAHashMap,
    definitions::{CallbackCallable, GenericInput, KeyCode, Modifiers},
    EventHelper,
};

use super::callbacks::InputCallbacks;

pub struct InputDataWithId(DefaultAHashMap<DeviceId, InputData>);

impl Default for InputDataWithId {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Deref for InputDataWithId {
    type Target = DefaultAHashMap<DeviceId, InputData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InputDataWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<D: Clone> CallbackCallable<D> for InputDataWithId {
    type CallbackStruct = DefaultAHashMap<DeviceId, InputCallbacks<D>>;

    fn call_callbacks(&self, event_helper: &mut EventHelper<D>, callbacks: &Self::CallbackStruct) {
        self.iter().for_each(|(device_id, input_data)| {
            input_data.call_callbacks(event_helper, &callbacks[device_id])
        });
    }
}

impl InputDataWithId {
    pub fn clear(&mut self) {
        self.values_mut().for_each(InputData::clear);
    }
}

#[derive(Clone)]
/// A collection of data used for input callbacks.
///
/// [InputCallbacks] holds the callbacks themselves.
pub struct InputData {
    pressed: AHashMap<GenericInput, Instant>,
    just_pressed: AHashSet<GenericInput>,
    just_released: AHashSet<GenericInput>,
    modifiers: Modifiers,
}

impl<D> CallbackCallable<D> for InputData {
    type CallbackStruct = InputCallbacks<D>;

    fn call_callbacks(&self, event_helper: &mut EventHelper<D>, callbacks: &Self::CallbackStruct) {
        callbacks
            .pressed
            .iter()
            .filter(|((inputs, modifiers), _)| self.pressed_combination(inputs.clone(), *modifiers))
            .for_each(|(_, func)| func(event_helper));

        callbacks
            .just_pressed
            .iter()
            .filter(|((inputs, modifiers), _)| {
                self.just_pressed_combination(inputs.clone(), *modifiers)
            })
            .for_each(|(_, func)| func(event_helper));

        callbacks
            .just_released
            .iter()
            .filter(|((inputs, modifiers), _)| {
                self.just_released_combination(inputs.clone(), *modifiers)
            })
            .for_each(|(_, func)| func(event_helper));
    }
}

impl Default for InputData {
    fn default() -> Self {
        Self {
            pressed: AHashMap::new(),
            just_pressed: AHashSet::new(),
            just_released: AHashSet::new(),
            modifiers: Modifiers::empty(),
        }
    }
}

impl InputData {
    pub fn just_pressed<I: Into<GenericInput>>(&self, input: I) -> bool {
        self.just_pressed.contains(&input.into())
    }

    pub fn just_pressed_any<I: Into<GenericInput>>(
        &self,
        inputs: impl IntoIterator<Item = I>,
    ) -> bool {
        inputs.into_iter().any(|input| self.just_pressed(input))
    }

    pub fn just_pressed_all<I: Into<GenericInput>>(
        &self,
        inputs: impl IntoIterator<Item = I>,
    ) -> bool {
        inputs.into_iter().all(|input| self.just_pressed(input))
    }

    pub fn just_pressed_iter(&self) -> impl ExactSizeIterator<Item = GenericInput> {
        self.just_pressed.clone().into_iter()
    }

    pub fn just_released<I: Into<GenericInput>>(&self, input: I) -> bool {
        self.just_released.contains(&input.into())
    }

    pub fn just_released_any<I: Into<GenericInput>>(
        &self,
        inputs: impl IntoIterator<Item = I>,
    ) -> bool {
        inputs.into_iter().any(|input| self.just_released(input))
    }

    pub fn just_released_all<I: Into<GenericInput>>(
        &self,
        inputs: impl IntoIterator<Item = I>,
    ) -> bool {
        inputs.into_iter().all(|input| self.just_released(input))
    }

    pub fn just_released_iter(&self) -> impl ExactSizeIterator<Item = GenericInput> {
        self.just_released.clone().into_iter()
    }

    /// Registers the given input as pressed
    pub fn press<I: Into<GenericInput>>(&mut self, input: I) {
        let value = input.into();
        if self.pressed.insert(value, Instant::now()).is_none() {
            self.just_pressed.insert(value);
        }
    }

    pub fn pressed<I: Into<GenericInput>>(&self, input: I) -> bool {
        self.pressed.contains_key(&input.into())
    }

    pub fn pressed_all<I: Into<GenericInput>>(&self, inputs: impl IntoIterator<Item = I>) -> bool {
        inputs.into_iter().all(|input| self.pressed(input))
    }

    pub fn pressed_any<I: Into<GenericInput>>(&self, inputs: impl IntoIterator<Item = I>) -> bool {
        inputs.into_iter().any(|input| self.pressed(input))
    }

    pub fn pressed_for<I: Into<GenericInput>>(&self, input: I) -> Option<Duration> {
        self.pressed.get(&input.into()).map(|i| i.elapsed())
    }

    pub fn pressed_iter(&self) -> impl ExactSizeIterator<Item = GenericInput> {
        self.pressed.clone().into_keys()
    }

    pub fn update_modifiers(&mut self, modifiers: Modifiers) {
        self.modifiers = modifiers;
    }

    pub fn just_pressed_combination<I: Into<GenericInput>>(
        &self,
        inputs: impl IntoIterator<Item = I>,
        modifiers: Modifiers,
    ) -> bool {
        self.just_pressed_all(inputs) && self.modifiers.contains(modifiers)
    }

    pub fn pressed_combination<I: Into<GenericInput>>(
        &self,
        inputs: impl IntoIterator<Item = I>,
        modifiers: Modifiers,
    ) -> bool {
        self.pressed_all(inputs) && self.modifiers.contains(modifiers)
    }

    pub fn just_released_combination<I: Into<GenericInput>>(
        &self,
        inputs: impl IntoIterator<Item = I>,
        modifiers: Modifiers,
    ) -> bool {
        self.just_released_all(inputs) && self.modifiers.contains(modifiers)
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

    /// Registers the given input as released
    pub fn release<I: Into<GenericInput>>(&mut self, input: I) {
        let value = input.into();
        self.pressed.remove(&value);
        self.just_pressed.remove(&value);
        self.just_released.insert(value);
    }

    /// Clears the `just_pressed` and `just_released` fields
    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// Resets all fields
    pub fn reset(&mut self) {
        self.pressed.clear();
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn update<I: Into<GenericInput>>(&mut self, value: I, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.press(value);
            }
            ElementState::Released => {
                self.release(value);
            }
        }
    }

    pub fn key_just_pressed_iter(&self) -> impl Iterator<Item = VirtualKeyCode> {
        filter_keyboard_keys(self.just_pressed_iter())
    }

    pub fn button_just_pressed_iter(&self) -> impl Iterator<Item = MouseButton> {
        filter_mouse_buttons(self.just_pressed_iter())
    }

    pub fn key_just_released_iter(&self) -> impl Iterator<Item = VirtualKeyCode> {
        filter_keyboard_keys(self.just_released_iter())
    }

    pub fn button_just_released_iter(&self) -> impl Iterator<Item = MouseButton> {
        filter_mouse_buttons(self.just_released_iter())
    }

    pub fn key_pressed_iter(&self) -> impl Iterator<Item = VirtualKeyCode> {
        filter_keyboard_keys(self.pressed_iter())
    }

    pub fn button_pressed_iter(&self) -> impl Iterator<Item = MouseButton> {
        filter_mouse_buttons(self.pressed_iter())
    }
}

pub fn filter_keyboard_keys<I>(iter: I) -> impl Iterator<Item = KeyCode>
where
    I: Iterator<Item = GenericInput>,
{
    iter.filter_map(|input| input.try_into().ok())
}

pub fn filter_mouse_buttons<I>(iter: I) -> impl Iterator<Item = MouseButton>
where
    I: Iterator<Item = GenericInput>,
{
    iter.filter_map(|input| input.try_into().ok())
}
