#[cfg(feature = "std")]
use std::ops::{Deref, DerefMut};

use ahash::{AHashMap, AHashSet};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode, WindowEvent,
    },
};

pub type CB<D> = fn(&mut D);
pub type CBI<D, I> = fn(&mut D, I);

/// Executes the internal function if $self.$field is Some(_)
macro_rules! option_exec {
    ($self:ident.$($field:ident).+) => {
        if let Some(callback) = &mut $self.$( $field ).+ {
            callback(&mut $self.data);
        }
    };
    ($self:ident.$($field:ident).+, input=$input:ident) => {
        if let Some(callback) = &mut $self.$( $field ).+ {
            callback(&mut $self.data, $input);
        }
    };
    ($self:ident, $getter:expr) => {
        if let Some(callback) = $getter {
            callback(&mut $self.data);
        }
    };
}

macro_rules! add_callback {
    ($self:ident.$field:ident) => {
        pub fn $field(&mut $self, callback: CB<D>) {
            $self.$field = Some(callback);
        }
    };
    ($self:ident.$field:ident, $type:ty) => {
        pub fn $field(&mut $self, callback: CBI<D, $type>) {
            $self.$field = Some(callback);
        }
    };
}

pub struct EventHelper<D> {
    pub data: D,
    // stored
    keys_held: AHashSet<VirtualKeyCode>,
    // global
    suspended: Option<CB<D>>,
    resumed: Option<CB<D>>,
    // window
    close_requested: Option<CB<D>>,
    cursor_entered: Option<CB<D>>,
    cursor_left: Option<CB<D>>,
    cursor_moved: Option<CBI<D, PhysicalPosition<f64>>>,
    mouse_input: AHashMap<(MouseButton, ElementState), CB<D>>,
    mouse_input_any: Option<CBI<D, (MouseButton, ElementState)>>,
    resized: Option<CBI<D, PhysicalSize<u32>>>,
    moved: Option<CBI<D, PhysicalPosition<i32>>>,
    focused: Option<CBI<D, bool>>,
    // device
    key: AHashMap<(VirtualKeyCode, ElementState), CB<D>>,
    key_any: Option<CBI<D, (VirtualKeyCode, ElementState)>>,
    raw_mouse_delta: Option<CBI<D, (f64, f64)>>,
    raw_mouse_scroll: Option<CBI<D, MouseScrollDelta>>,
}

impl<D> EventHelper<D> {
    pub fn new(data: D) -> EventHelper<D> {
        EventHelper {
            data,
            keys_held: AHashSet::new(),
            suspended: None,
            resumed: None,
            close_requested: None,
            cursor_entered: None,
            cursor_left: None,
            cursor_moved: None,
            mouse_input: AHashMap::new(),
            mouse_input_any: None,
            resized: None,
            moved: None,
            focused: None,
            key: AHashMap::new(),
            key_any: None,
            raw_mouse_delta: None,
            raw_mouse_scroll: None,
        }
    }

    // TODO: benchmark inlining
    // currently it is set to be inlined because there is only a single call site,
    // but it has not been benchmarked
    #[inline]
    pub fn update<'a, E>(&mut self, event: &Event<'a, E>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => {
                self.update_window_event(event);
            }
            Event::DeviceEvent { event, .. } => {
                self.update_device_event(event);
            }
            Event::Suspended => option_exec!(self.suspended),
            Event::Resumed => option_exec!(self.resumed),
            Event::MainEventsCleared => return true,
            _ => (),
        }
        false
    }

    // TODO: benchmark inlining
    #[inline]
    fn update_window_event(&mut self, event: &WindowEvent) {
        match event {
            &WindowEvent::MouseInput { button, state, .. } => {
                option_exec!(self, { self.mouse_input.get_mut(&(button, state)) });
            }
            &WindowEvent::CursorMoved { position, .. } => {
                option_exec!(self.cursor_moved, input = position);
            }
            &WindowEvent::Resized(size) => option_exec!(self.resized, input = size),
            &WindowEvent::Moved(position) => option_exec!(self.moved, input = position),
            &WindowEvent::Focused(focus) => option_exec!(self.focused, input = focus),
            &WindowEvent::CursorEntered { .. } => option_exec!(self.cursor_entered),
            &WindowEvent::CursorLeft { .. } => option_exec!(self.cursor_left),
            &WindowEvent::CloseRequested => option_exec!(self.close_requested),
            _ => (),
        }
    }

    // TODO: benchmark inlining
    #[inline]
    fn update_device_event(&mut self, event: &DeviceEvent) {
        match event {
            &DeviceEvent::Key(KeyboardInput {
                virtual_keycode,
                state,
                ..
            }) => {
                if let Some(key) = virtual_keycode {
                    match state {
                        ElementState::Pressed => self.keys_held.insert(key),
                        ElementState::Released => self.keys_held.remove(&key),
                    };
                    option_exec!(self, { self.key.get_mut(&(key, state)) });
                }
            }
            &DeviceEvent::MouseMotion { delta } => {
                option_exec!(self.raw_mouse_delta, input = delta)
            }
            &DeviceEvent::MouseWheel { delta } => {
                option_exec!(self.raw_mouse_scroll, input = delta)
            }
            _ => (),
        }
    }

    /// Returns true when a given key is being held
    pub fn key_held(&self, key: VirtualKeyCode) -> bool {
        self.keys_held.contains(&key)
    }

    /// Retreives all keys currently being held
    pub fn keys_held(&self) -> impl Iterator<Item = &VirtualKeyCode> + '_ {
        self.keys_held.iter()
    }

    /// Adds a keyboard callback and returns true if a callback was already linked
    pub fn keyboard(&mut self, key: VirtualKeyCode, state: ElementState, callback: CB<D>) -> bool {
        self.key.insert((key, state), callback).is_some()
    }

    pub fn keyboard_any(&mut self, callback: CBI<D, (VirtualKeyCode, ElementState)>) {
        self.key_any = Some(callback);
    }

    /// Adds a mouse callback and returns true if a callback was already linked
    pub fn mouse(
        &mut self,
        button: MouseButton,
        state: ElementState,
        callback: CB<D>,
    ) -> bool {
        self.mouse_input.insert((button, state), callback).is_some()
    }

    pub fn mouse_any(&mut self, callback: CBI<D, (MouseButton, ElementState)>) {
        self.mouse_input_any = Some(callback);
    }

    add_callback!(self.suspended);
    add_callback!(self.resumed);
    add_callback!(self.cursor_entered);
    add_callback!(self.cursor_left);
    add_callback!(self.close_requested);
    add_callback!(self.cursor_moved, PhysicalPosition<f64>);
    add_callback!(self.resized, PhysicalSize<u32>);
    add_callback!(self.moved, PhysicalPosition<i32>);
    add_callback!(self.focused, bool);
    add_callback!(self.raw_mouse_delta, (f64, f64));
    add_callback!(self.raw_mouse_scroll, MouseScrollDelta);
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
