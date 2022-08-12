#![allow(dead_code)]

#[cfg(feature = "std")]
use std::ops::{Deref, DerefMut};

use ahash::{AHashMap, AHashSet};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode, WindowEvent,
    },
    event_loop::ControlFlow,
};

pub type Callback<D> = fn(&mut D, &mut ControlFlow);
pub type Callback2<D, I> = fn(&mut D, &mut ControlFlow, I);

pub type MouseMotionCallback<D> = Callback2<D, PhysicalPosition<f64>>;
pub type MouseDeltaCallback<D> = Callback2<D, (f64, f64)>;
pub type MouseScrollCallback<D> = Callback2<D, MouseScrollDelta>;
pub type ResizedCallback<D> = Callback2<D, PhysicalSize<u32>>;
pub type MovedCallback<D> = Callback2<D, PhysicalPosition<i32>>;
pub type FocusedCallback<D> = Callback2<D, bool>;

/// Executes the internal function if $self.$field is Some(_)
macro_rules! option_exec {
    ($self:ident, $field:ident, $control_flow:ident) => {
        if let Some(callback) = &mut $self.$field {
            callback(&mut $self.data, $control_flow);
        }
    };
    ($self:ident, $field:ident, $control_flow:ident, input=$input:ident) => {
        if let Some(callback) = &mut $self.$field {
            callback(&mut $self.data, $control_flow, $input);
        }
    };
    ($self:ident, $control_flow:ident, $getter:expr) => {
        if let Some(callback) = $getter {
            callback(&mut $self.data, $control_flow);
        }
    };
}

macro_rules! add_callback {
    ($self:ident, $field:ident) => {
        pub fn $field(&mut $self, callback: Callback<D>) {
            $self.$field = Some(callback);
        }
    };
    ($self:ident, $field:ident, $type:ty) => {
        pub fn $field(&mut $self, callback: $type) {
            $self.$field = Some(callback);
        }
    };
}

pub struct EventHelper<D: 'static> {
    pub data: D,
    keys_held: AHashSet<VirtualKeyCode>,
    suspended: Option<Callback<D>>,
    resumed: Option<Callback<D>>,
    close_requested: Option<Callback<D>>,
    unfocused: Option<Callback<D>>,
    mouse_entered: Option<Callback<D>>,
    mouse_left: Option<Callback<D>>,
    keyboard: AHashMap<(VirtualKeyCode, ElementState), Callback<D>>,
    mouse_click: AHashMap<(MouseButton, ElementState), Callback<D>>,
    mouse_motion: Option<MouseMotionCallback<D>>,
    resized: Option<ResizedCallback<D>>,
    moved: Option<MovedCallback<D>>,
    focused: Option<FocusedCallback<D>>,
    raw_mouse_delta: Option<MouseDeltaCallback<D>>,
    raw_mouse_scroll: Option<MouseScrollCallback<D>>,
}

impl<D> EventHelper<D> {
    pub fn new(data: D) -> EventHelper<D> {
        EventHelper {
            data,
            keys_held: AHashSet::new(),
            keyboard: AHashMap::new(),
            mouse_click: AHashMap::new(),
            mouse_motion: None,
            suspended: None,
            resumed: None,
            resized: None,
            moved: None,
            focused: None,
            unfocused: None,
            mouse_entered: None,
            mouse_left: None,
            close_requested: None,
            raw_mouse_delta: None,
            raw_mouse_scroll: None,
        }
    }

    // TODO: benchmark inlining
    // currently it is set to be inlined because there is only a single call site,
    // but it has not been benchmarked
    #[inline]
    pub fn update<'a, E>(&mut self, event: &Event<'a, E>, cf: &mut ControlFlow) -> bool {
        match event {
            Event::WindowEvent { event, .. } => {
                self.update_window_event(event, cf);
            }
            Event::DeviceEvent { event, .. } => {
                self.update_device_event(event, cf);
            }
            Event::Suspended => option_exec!(self, suspended, cf),
            Event::Resumed => option_exec!(self, resumed, cf),
            Event::MainEventsCleared => return true,
            _ => (),
        }
        false
    }

    // TODO: benchmark inlining
    #[inline]
    fn update_window_event(&mut self, event: &WindowEvent, cf: &mut ControlFlow) {
        match event {
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
                    match state {
                        ElementState::Pressed => self.keys_held.insert(key),
                        ElementState::Released => self.keys_held.remove(&key),
                    };
                    option_exec!(self, cf, { self.keyboard.get_mut(&(key, state)) });
                }
            }
            &WindowEvent::MouseInput { button, state, .. } => {
                option_exec!(self, cf, { self.mouse_click.get_mut(&(button, state)) });
            }
            &WindowEvent::CursorMoved { position, .. } => {
                option_exec!(self, mouse_motion, cf, input = position);
            }
            &WindowEvent::Resized(size) => option_exec!(self, resized, cf, input = size),
            &WindowEvent::Moved(position) => option_exec!(self, moved, cf, input = position),
            &WindowEvent::Focused(focus) => option_exec!(self, focused, cf, input = focus),
            &WindowEvent::CursorEntered { .. } => option_exec!(self, mouse_entered, cf),
            &WindowEvent::CursorLeft { .. } => option_exec!(self, mouse_left, cf),
            &WindowEvent::CloseRequested => option_exec!(self, close_requested, cf),
            _ => (),
        }
    }

    // TODO: benchmark inlining
    #[inline]
    fn update_device_event(&mut self, event: &DeviceEvent, cf: &mut ControlFlow) {
        match event {
            &DeviceEvent::MouseMotion { delta } => {
                option_exec!(self, raw_mouse_delta, cf, input = delta)
            }
            &DeviceEvent::MouseWheel { delta } => {
                option_exec!(self, raw_mouse_scroll, cf, input = delta)
            }
            _ => (),
        }
    }

    pub fn key_held(&self, key: VirtualKeyCode) -> bool {
        self.keys_held.contains(&key)
    }

    pub fn keys_held(&self) -> impl Iterator<Item = &VirtualKeyCode> + '_ {
        self.keys_held.iter()
    }

    /// Adds a keyboard callback and returns true if a callback was already linked
    pub fn keyboard(
        &mut self,
        key: VirtualKeyCode,
        state: ElementState,
        callback: Callback<D>,
    ) -> bool {
        self.keyboard.insert((key, state), callback).is_some()
    }

    /// Adds a mouse callback and returns true if a callback was already linked
    pub fn mouse_click(
        &mut self,
        button: MouseButton,
        state: ElementState,
        callback: Callback<D>,
    ) -> bool {
        self.mouse_click.insert((button, state), callback).is_some()
    }

    add_callback!(self, suspended);
    add_callback!(self, resumed);
    add_callback!(self, mouse_entered);
    add_callback!(self, mouse_left);
    add_callback!(self, close_requested);
    add_callback!(self, mouse_motion, MouseMotionCallback<D>);
    add_callback!(self, resized, ResizedCallback<D>);
    add_callback!(self, moved, MovedCallback<D>);
    add_callback!(self, focused, FocusedCallback<D>);
    add_callback!(self, raw_mouse_delta, MouseDeltaCallback<D>);
    add_callback!(self, raw_mouse_scroll, MouseScrollCallback<D>);
}

#[cfg(feature = "std")]
impl<D> Deref for EventHelper<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(feature = "std")]
impl<D> DerefMut for EventHelper<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
