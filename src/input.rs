use std::{
    hash::Hash,
    time::{Duration, Instant},
};

use ahash::{AHashMap, AHashSet};
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
/// An extension of [ElementState]
pub enum State {
    Pressed,
    Held,
    Released,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
/// A generic input type.
pub enum GeneralInput {
    MouseButton(MouseButton),
    KeyCode(VirtualKeyCode),
}

impl From<MouseButton> for GeneralInput {
    fn from(value: MouseButton) -> Self {
        Self::MouseButton(value)
    }
}

impl From<VirtualKeyCode> for GeneralInput {
    fn from(value: VirtualKeyCode) -> Self {
        Self::KeyCode(value)
    }
}

pub fn filter_keyboard_keys<'a, I>(iter: I) -> impl Iterator<Item = &'a VirtualKeyCode>
where
    I: Iterator<Item = &'a GeneralInput>,
{
    iter.filter_map(|i| match i {
        GeneralInput::KeyCode(value) => Some(value),
        _ => None,
    })
}

pub fn filter_mouse_buttons<'a, I>(iter: I) -> impl Iterator<Item = &'a MouseButton>
where
    I: Iterator<Item = &'a GeneralInput>,
{
    iter.filter_map(|i| match i {
        GeneralInput::MouseButton(value) => Some(value),
        _ => None,
    })
}

pub fn into_input_iter<V: Into<GeneralInput>>(
    values: impl IntoIterator<Item = V>,
) -> impl Iterator<Item = GeneralInput> {
    values.into_iter().map(|v| v.into())
}

#[derive(Clone)]
/// An input handling struct for generic input handling with steps.
pub struct Input<T: 'static + Hash + PartialEq + Eq + Clone + Copy> {
    pressed: AHashMap<T, Instant>,
    just_pressed: AHashSet<T>,
    just_released: AHashSet<T>,
}

impl<T: 'static + Hash + PartialEq + Eq + Clone + Copy> Default for Input<T> {
    fn default() -> Self {
        Self {
            pressed: AHashMap::new(),
            just_pressed: AHashSet::new(),
            just_released: AHashSet::new(),
        }
    }
}

impl<T: 'static + Hash + PartialEq + Eq + Clone + Copy> Input<T> {
    pub fn just_pressed(&self, input: T) -> bool {
        self.just_pressed.contains(&input)
    }

    pub fn just_pressed_any(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().any(|input| self.just_pressed(input))
    }

    pub fn just_pressed_all(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().all(|input| self.just_pressed(input))
    }

    pub fn just_pressed_iter(&self) -> impl ExactSizeIterator<Item = &T> {
        self.just_pressed.iter()
    }

    pub fn just_released(&self, input: T) -> bool {
        self.just_released.contains(&input)
    }

    pub fn just_released_any(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().any(|input| self.just_released(input))
    }

    pub fn just_released_all(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().all(|input| self.just_released(input))
    }

    pub fn just_released_iter(&self) -> impl ExactSizeIterator<Item = &T> {
        self.just_released.iter()
    }

    /// Registers the given input as pressed
    pub fn press(&mut self, input: T) {
        if self.pressed.insert(input, Instant::now()).is_none() {
            self.just_pressed.insert(input);
        }
    }

    pub fn pressed(&self, input: T) -> bool {
        self.pressed.contains_key(&input)
    }

    pub fn pressed_all(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().all(|input| self.pressed(input))
    }

    pub fn pressed_any(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().any(|input| self.pressed(input))
    }

    pub fn pressed_for(&self, input: T) -> Option<Duration> {
        self.pressed.get(&input).map(|i| i.elapsed())
    }

    pub fn pressed_for_secs_f32(&self, input: T) -> Option<f32> {
        self.pressed_for(input).map(|d| d.as_secs_f32())
    }

    pub fn pressed_for_secs_f64(&self, input: T) -> Option<f64> {
        self.pressed_for(input).map(|d| d.as_secs_f64())
    }

    pub fn pressed_iter(&self) -> impl ExactSizeIterator<Item = &T> {
        self.pressed.keys()
    }

    /// Registers the given input as released
    pub fn release(&mut self, input: T) {
        self.pressed.remove(&input);
        self.just_pressed.remove(&input);
        self.just_released.insert(input);
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
}

pub(crate) fn element_state_callbacks<F>(state: ElementState, is_pressed: bool, mut callbacks: F)
where
    F: FnMut(State),
{
    match state {
        ElementState::Pressed => {
            if is_pressed {
                callbacks(State::Pressed);
            }
            callbacks(State::Held);
        }
        ElementState::Released => callbacks(State::Released),
    }
}
