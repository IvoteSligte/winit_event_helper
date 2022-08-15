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

/// Executes the internal function if `$self.$field` is Some(_)
#[macro_export]
macro_rules! option_exec {
    ($self:ident.$($field:ident).+) => {
        if let Some(callback) = &mut $self.$( $field ).+ {
            callback(&mut $self.data);
        }
    };
    ($self:ident.$($field:ident).+, input=$input:expr) => {
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

/// Implements a function that changes the `$self.$field` option to Some(callback)
#[macro_export]
macro_rules! add_callback {
    ($self:ident.$field:ident $(,)?) => {
        add_callback!($self.$field, name=$field);
    };
    ($self:ident.$field:ident, input=$type:ty $(,)?) => {
        add_callback!($self.$field, input=$type, name=$field);
    };
    ($self:ident.$field:ident, name=$name:ident $(,)?) => {
        pub fn $name(&mut $self, callback: CB<D>) {
            $self.$field = Some(callback);
        }
    };
    ($self:ident.$field:ident, input=$type:ty, name=$name:ident $(,)?) => {
        pub fn $name(&mut $self, callback: CBI<D, $type>) {
            $self.$field = Some(callback);
        }
    };
}

#[macro_export]
/// Implements a function that inserts a callback into `$self.$field`
/// (assumes `$self.$field` implements the hashmap `insert` function)
macro_rules! insert_callback {
    ($self:ident.$field:ident, keys=($( $key:ident: $keytype:ty ),+) $(,)?) => {
        insert_callback!($self.$field, keys=$( $key: $keytype ),+, name=$field);
    };
    ($self:ident.$field:ident, keys=($( $key:ident: $keytype:ty ),+), name=$name:ident $(,)?) => {
        pub fn $name(&mut $self, $( $key: $keytype ),+, callback: CB<D>) {
            $self.$field.insert(($( $key ),+), callback);
        }
    };
}

pub struct EventHelper<D> {
    pub data: D,
    // stored
    keys_held: AHashSet<VirtualKeyCode>,
    mouse_inputs_held: AHashSet<MouseButton>,
    // global
    suspended: Option<CB<D>>,
    resumed: Option<CB<D>>,
    // window
    close_requested: Option<CB<D>>,
    cursor_entered: Option<CB<D>>,
    cursor_left: Option<CB<D>>,
    cursor_moved: Option<CBI<D, PhysicalPosition<f64>>>,
    mouse_inputs: AHashMap<(MouseButton, ElementState), CB<D>>,
    mouse_input_any: Option<CBI<D, (MouseButton, ElementState)>>,
    resized: Option<CBI<D, PhysicalSize<u32>>>,
    moved: Option<CBI<D, PhysicalPosition<i32>>>,
    focused: Option<CBI<D, bool>>,
    // device
    keys: AHashMap<(VirtualKeyCode, ElementState), CB<D>>,
    key_any: Option<CBI<D, (VirtualKeyCode, ElementState)>>,
    raw_mouse_delta: Option<CBI<D, (f64, f64)>>,
    raw_mouse_scroll: Option<CBI<D, MouseScrollDelta>>,
}

impl<D> EventHelper<D> {
    pub fn new(data: D) -> EventHelper<D> {
        EventHelper {
            data,
            keys_held: AHashSet::new(),
            mouse_inputs_held: AHashSet::new(),
            suspended: None,
            resumed: None,
            close_requested: None,
            cursor_entered: None,
            cursor_left: None,
            cursor_moved: None,
            mouse_inputs: AHashMap::new(),
            mouse_input_any: None,
            resized: None,
            moved: None,
            focused: None,
            keys: AHashMap::new(),
            key_any: None,
            raw_mouse_delta: None,
            raw_mouse_scroll: None,
        }
    }

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

    #[inline]
    fn update_window_event(&mut self, event: &WindowEvent) {
        match event {
            &WindowEvent::MouseInput { button, state, .. } => {
                match state {
                    ElementState::Pressed => self.mouse_inputs_held.insert(button),
                    ElementState::Released => self.mouse_inputs_held.remove(&button),
                };
                option_exec!(self.mouse_input_any, input = (button, state));
                option_exec!(self, { self.mouse_inputs.get_mut(&(button, state)) });
            }
            &WindowEvent::CursorMoved { position, .. } => {
                option_exec!(self.cursor_moved, input = position);
            }
            &WindowEvent::Resized(size) => {
                option_exec!(self.resized, input = size);
            }
            &WindowEvent::Moved(position) => {
                option_exec!(self.moved, input = position);
            }
            &WindowEvent::Focused(focus) => option_exec!(self.focused, input = focus),
            &WindowEvent::CursorEntered { .. } => option_exec!(self.cursor_entered),
            &WindowEvent::CursorLeft { .. } => option_exec!(self.cursor_left),
            &WindowEvent::CloseRequested => option_exec!(self.close_requested),
            _ => (),
        }
    }

    #[inline]
    fn update_device_event(&mut self, event: &DeviceEvent) {
        match event {
            &DeviceEvent::Key(KeyboardInput {
                virtual_keycode,
                state,
                ..
            }) => {
                if let Some(key) = virtual_keycode {
                    option_exec!(self.key_any, input = (key, state));
                    match state {
                        ElementState::Pressed => self.keys_held.insert(key),
                        ElementState::Released => self.keys_held.remove(&key),
                    };
                    option_exec!(self, { self.keys.get_mut(&(key, state)) });
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

    /// Returns true when a given key is pressed
    pub fn key_held(&self, key: VirtualKeyCode) -> bool {
        self.keys_held.contains(&key)
    }

    /// Retrieves all keys currently pressed
    pub fn keys_held(&self) -> impl Iterator<Item = &VirtualKeyCode> + '_ {
        self.keys_held.iter()
    }

    /// Returns true when a given mouse button is pressed
    pub fn mouse_input_held(&self, button: MouseButton) -> bool {
        self.mouse_inputs_held.contains(&button)
    }

    /// Retrieves all mouse buttons currently pressed
    pub fn mouse_inputs_held(&self) -> impl Iterator<Item = &MouseButton> + '_ {
        self.mouse_inputs_held.iter()
    }

    insert_callback!(
        self.keys,
        keys = (key: VirtualKeyCode, state: ElementState),
        name = keyboard,
    );
    insert_callback!(
        self.mouse_inputs,
        keys = (button: MouseButton, state: ElementState),
        name = mouse_input,
    );
    add_callback!(
        self.key_any,
        input = (VirtualKeyCode, ElementState),
        name = keyboard_any,
    );
    add_callback!(self.mouse_input_any, input = (MouseButton, ElementState));
    add_callback!(self.suspended);
    add_callback!(self.resumed);
    add_callback!(self.cursor_entered);
    add_callback!(self.cursor_left);
    add_callback!(self.close_requested);
    add_callback!(self.cursor_moved, input=PhysicalPosition<f64>);
    add_callback!(self.resized, input=PhysicalSize<u32>);
    add_callback!(self.moved, input=PhysicalPosition<i32>);
    add_callback!(self.focused, input = bool);
    add_callback!(self.raw_mouse_delta, input = (f64, f64));
    add_callback!(self.raw_mouse_scroll, input = MouseScrollDelta);
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
