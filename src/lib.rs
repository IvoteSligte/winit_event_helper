//! winit_event_helper is a crate intended to make winit event handling 
//! easier using [callback functions](https://en.wikipedia.org/wiki/Callback_(computer_programming))
//! without taking over the main loop.
//! 
//! ## Usage
//! 
//! winit_event_helper comes with the EventHelper struct, which handles all the callbacks
//! and various miscellaneous things (see [EventHelper]). The user can also create their own struct
//! and implement its functions using the [add_callback] and [insert_callback] macros.
//! 
//! ## Example (using the EventHelper struct)
//! 
//! ```rust
//! use winit_event_helper::EventHelper;
//! use winit::event::{ElementState, VirtualKeyCode, MouseButton};
//! use winit::event_loop::{EventLoop, ControlFlow};
//! 
//! struct Data {
//!     counter: usize
//! }
//! 
//! fn main() {
//!     let mut event_loop = EventLoop::new();
//!     let mut eh = EventHelper::new( Data { counter: 0 } );
//!     
//!     // is called whenever the given mouse button is in the given state
//!     eh.mouse_input(MouseButton::Left, ElementState::Pressed, |data| data.counter += 1);
//!     
//!     // is called whenever a keyboard button is pressed
//!     eh.keyboard_any(|_, (keycode, state)| {
//!         if (state == ElementState::Pressed) {
//!             println!("{:?}", keycode);
//!         }
//!     });
//!     
//!     event_loop.run(move |event, _, control_flow| {
//!         // feed the events to the EventHelper struct
//!         // returns true when it receives MainEventsCleared
//!         if !eh.update(&event) {
//!             return;
//!         }
//! 
//!         // returns true if the given key is being held
//!         if eh.key_held(VirtualKeyCode::Escape) {
//!             *control_flow = ControlFlow::Exit;
//!         }
//!     })
//! }
//! ```

use std::ops::{Deref, DerefMut};
use ahash::{AHashMap, AHashSet};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode, WindowEvent,
    },
};

/// Callback function with no inputs
pub type CB<D> = fn(&mut EventHelper<D>);
/// Callback function with one input
pub type CBI<D, I> = fn(&mut EventHelper<D>, I);

/// Executes the internal function if `$self.$field` is Some(_)
#[macro_export]
macro_rules! option_exec {
    ($self:ident.$($field:ident).+) => {
        if let Some(callback) = &mut $self.$( $field ).+ {
            callback($self);
        }
    };
    ($self:ident.$($field:ident).+, input=$input:expr) => {
        if let Some(callback) = &mut $self.$( $field ).+ {
            callback($self, $input);
        }
    };
    ($self:ident, $getter:expr) => {
        if let Some(callback) = $getter {
            callback($self);
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

/// A struct holding all the callback functions and user function data
/// 
/// The EventHelper struct has several helper functions:
/// - `key_held` and `mouse_input_held` return true if the given key/button is being held
/// - `keys_held` and `mouse_inputs_held` return all keys/buttons currently being held
/// - `call_after` calls the given function the next time `update` is called
pub struct EventHelper<D> {
    // misc
    pub data: D,
    call_after: Vec<CB<D>>,
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
            call_after: Vec::new(),
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
        for cb in self.call_after.clone().iter().rev() {
            cb(self);
        }
        self.call_after.clear();
        
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

    /// Returns true if a given key is pressed
    pub fn key_held(&self, key: VirtualKeyCode) -> bool {
        self.keys_held.contains(&key)
    }

    /// Returns all keys currently pressed
    pub fn keys_held(&self) -> impl Iterator<Item = &VirtualKeyCode> + '_ {
        self.keys_held.iter()
    }

    /// Returns true if a given mouse button is pressed
    pub fn mouse_input_held(&self, button: MouseButton) -> bool {
        self.mouse_inputs_held.contains(&button)
    }

    /// Returns all mouse buttons currently pressed
    pub fn mouse_inputs_held(&self) -> impl Iterator<Item = &MouseButton> + '_ {
        self.mouse_inputs_held.iter()
    }

    /// Calls the given function before the next event is handled
    pub fn call_after(&mut self, callback: CB<D>) {
        self.call_after.push(callback);
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

impl<D> Deref for EventHelper<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<D> DerefMut for EventHelper<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
