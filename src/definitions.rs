use std::ops::AddAssign;

use bitflags::bitflags;
use winit::{
    dpi::PhysicalPosition,
    event::{Force, ModifiersState, MouseScrollDelta, Touch, TouchPhase, VirtualKeyCode},
};

pub use winit::event::{AxisId, ButtonId, MouseButton, ScanCode};

use crate::EventHelper;

/// A callback function with no inputs
pub type CB<D> = fn(&mut EventHelper<D>);
/// A callback function with one input
pub type CBI<D, I> = fn(&mut EventHelper<D>, I);

bitflags! {
    pub struct QuitWindow: u8 {
        const DESTROYED       = 0b0000_0001;
        const CLOSE_REQUESTED = 0b0000_0010;
    }
}

bitflags! {
    /// Bitflags for quit requests
    pub struct Quit: u8 {
        const USER_REQUESTED = 0b0000_0001;
        const LOOP_DESTROYED = 0b0000_0010;
    }
}

impl Default for Quit {
    fn default() -> Self {
        Quit::empty()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct LineDelta {
    right: f32,
    down: f32,
}

impl AddAssign for LineDelta {
    fn add_assign(&mut self, rhs: Self) {
        self.right += rhs.right;
        self.down += rhs.down;
    }
}

impl TryFrom<MouseScrollDelta> for LineDelta {
    type Error = ();

    fn try_from(value: MouseScrollDelta) -> Result<Self, Self::Error> {
        if let MouseScrollDelta::LineDelta(right, down) = value {
            Ok(Self { right, down })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PixelDelta {
    right: f64,
    down: f64,
}

impl AddAssign for PixelDelta {
    fn add_assign(&mut self, rhs: Self) {
        self.right += rhs.right;
        self.down += rhs.down;
    }
}

impl From<PhysicalPosition<f64>> for PixelDelta {
    fn from(value: PhysicalPosition<f64>) -> Self {
        Self {
            right: value.x,
            down: value.y,
        }
    }
}

impl TryFrom<MouseScrollDelta> for PixelDelta {
    type Error = ();

    fn try_from(value: MouseScrollDelta) -> Result<Self, Self::Error> {
        if let MouseScrollDelta::PixelDelta(position) = value {
            Ok(position.into())
        } else {
            Err(())
        }
    }
}

pub type Modifiers = ModifiersState;
pub type KeyCode = VirtualKeyCode;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CursorState {
    Entered,
    Left,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
/// A generic input type combining inputs that can be pressed.
pub enum GenericInput {
    MouseButton(MouseButton),
    KeyCode(KeyCode),
    ScanCode(ScanCode),
}

impl From<MouseButton> for GenericInput {
    fn from(value: MouseButton) -> Self {
        Self::MouseButton(value)
    }
}

impl From<KeyCode> for GenericInput {
    fn from(value: KeyCode) -> Self {
        Self::KeyCode(value)
    }
}

impl From<ScanCode> for GenericInput {
    fn from(value: ScanCode) -> Self {
        Self::ScanCode(value)
    }
}

impl TryFrom<GenericInput> for KeyCode {
    type Error = ();

    fn try_from(value: GenericInput) -> Result<Self, Self::Error> {
        match value {
            GenericInput::KeyCode(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryFrom<GenericInput> for MouseButton {
    type Error = ();

    fn try_from(value: GenericInput) -> Result<Self, Self::Error> {
        match value {
            GenericInput::MouseButton(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryFrom<GenericInput> for ScanCode {
    type Error = ();

    fn try_from(value: GenericInput) -> Result<Self, Self::Error> {
        match value {
            GenericInput::ScanCode(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl IntoIterator for GenericInput {
    type Item = Self;

    type IntoIter = std::iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Adapted from [winit::event::Touch]
pub struct IdLessTouch {
    pub phase: TouchPhase,
    pub location: PhysicalPosition<f64>,
    /// Describes how hard the screen was pressed. May be `None` if the platform
    /// does not support pressure sensitivity.
    ///
    /// ## Platform-specific
    ///
    /// - Only available on **iOS** 9.0+ and **Windows** 8+.
    pub force: Option<Force>,
    /// Unique identifier of a finger.
    pub id: u64,
}

impl From<Touch> for IdLessTouch {
    fn from(
        Touch {
            phase,
            location,
            force,
            id,
            ..
        }: Touch,
    ) -> Self {
        Self {
            phase,
            location,
            force,
            id,
        }
    }
}

pub trait CallbackCallable<D> {
    type CallbackStruct;

    #[allow(unused_variables)]
    fn call_callbacks(&self, event_helper: &mut EventHelper<D>, callbacks: &Self::CallbackStruct) {}
}
