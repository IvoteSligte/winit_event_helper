//! winit_event_helper is a crate for high level winit event handling
//! using [callback functions](https://en.wikipedia.org/wiki/Callback_(computer_programming))
//! without taking over the main loop.
//!
//! ## Usage
//!
//! winit_event_helper comes with the [event_helper::EventHelper] struct, which handles all the callbacks
//! and various miscellaneous things.
//!
//! Pass your events to [event_helper::EventHelper::update] and run your application calculations when it returns true.
//! You can also add callbacks for specific winit events with the EventHelper helper functions
//! or the [callbacks::Callbacks] struct.
//!
//! ## Example
//!
//! ```rust
//! use winit_event_helper::prelude::*;
//! use winit::event::{VirtualKeyCode, MouseButton};
//! use winit::event_loop::{EventLoop, ControlFlow};
//! use winit::window::WindowBuilder;
//!
//! struct Data {
//!     counter: usize
//! }
//!
//! fn main() {
//!     let mut event_loop = EventLoop::new();
//!     let _window = WindowBuilder::new().build(&event_loop).unwrap();
//!
//!     let mut eh = EventHelper::new( Data { counter: 0 } );
//!     
//!     // is called whenever the given mouse button is in the given state and the window is focused
//!     eh.callbacks.windows.mouse_input(MouseButton::Left, State::Pressed, |data| data.counter += 1);
//!     
//!     // is called whenever a keyboard key is pressed and the window is focused
//!     eh.callbacks.windows.keyboard_input_any(|_, (keycode, state)| {
//!         if (state == State::Pressed) {
//!             println!("{:?}", keycode);
//!         }
//!     });
//!     
//!     event_loop.run(move |event, _, control_flow| {
//!         // feed the events to the EventHelper struct
//!         // returns true when it receives [Event::MainEventsCleared]
//!         if !eh.update(&event) {
//!             return;
//!         }
//!
//!         // returns true when the given key goes from 'not pressed' to 'pressed'
//!         if eh.key_pressed(VirtualKeyCode::Escape) {
//!             *control_flow = ControlFlow::Exit;
//!         }
//!
//!         // do stuff
//!     })
//! }
//! ```
//!
//! ## Function names
//!
//! `winit_event_helper` has functions for adding a callback for every winit event except [Event::UserEvent].
//!
//! For a complete overview of functions, see 
//! [general_callbacks::GeneralCallbacks], 
//! [device_callbacks::DeviceCallbacks] and 
//! [window_callbacks::WindowCallbacks].
//!
//! ## Features
#![doc = document_features::document_features!()]

pub mod event_helper;

pub mod input;
pub mod definitions;

pub mod callback_data;
pub mod callbacks;

pub mod device_callbacks;
pub mod general_callbacks;
pub mod window_callbacks;

pub mod prelude {
    pub use crate::callbacks::Callbacks;
    pub use crate::event_helper::EventHelper;
    pub use crate::callback_data::CallbackData;
    pub use crate::input::State;
}

// TODO: docs for helper functions and structs
// TODO: update functions such as WindowCallbacks::keyboard_input_any
// TODO: update example
