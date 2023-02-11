//! winit_event_helper is a crate for simplified winit event handling
//! using [callback functions](https://en.wikipedia.org/wiki/Callback_(computer_programming))
//! without taking over the main loop.
//!
//! ## Usage
//!
//! winit_event_helper comes with the [EventHelper] struct, which handles all the callbacks
//! and various miscellaneous things.
//!
//! Pass your events to [EventHelper::update] and run your application logic when it returns `true`.
//! 
//! You can also add callbacks for specific winit events with the EventHelper helper functions
//! or the [Callbacks] struct.
//!
//! ## Example
//!
//! ```rust
//! use winit::event_loop::{ControlFlow, EventLoop};
//! use winit::window::WindowBuilder;
//! use winit_event_helper::*;
//! 
//! struct Data {
//!     counter: usize,
//! }
//! 
//! fn main() {
//!     let event_loop = EventLoop::new();
//!     let _window = WindowBuilder::new().build(&event_loop).unwrap();
//! 
//!     let mut eh = EventHelper::new(Data { counter: 0 });
//!     let mut callbacks = Callbacks::<Data>::empty();
//! 
//!     // is called whenever one of the given inputs was just pressed
//!     callbacks
//!         .window
//!         .inputs
//!         .just_pressed_all([GenericInput::from(MouseButton::Left), KeyCode::Space.into()], |eh| {
//!             eh.counter += 1
//!         });
//!     
//!     event_loop.run(move |event, _, control_flow| {
//!         // feed the events to the [EventHelper] struct
//!         // returns true when it receives [Event::MainEventsCleared]
//!         if !eh.update(&callbacks, &event) {
//!             return;
//!         }
//! 
//!         // exits the application when the key combination CTRL + ESC has been released
//!         if eh.data.window.inputs.just_released_combination([KeyCode::Escape], Modifiers::CTRL) {
//!             *control_flow = ControlFlow::Exit;
//!         }
//! 
//!         println!("{}", eh.counter);
//! 
//!         // do stuff
//!     })
//! }
//! ```
//!
//! ## Functions and Callbacks
//!
//! `winit_event_helper` has functions for adding a callback for every winit event type except
//! [winit::event::UserEvent](https://docs.rs/winit/latest/winit/event/enum.Event.html#variant.UserEvent).
//! Callbacks are called after a step as long as a winit event of the desired type is received.
//!
//! For a complete overview of functions, see [callbacks].
//!
//! ## Keyboard and Mouse Inputs
//!
//! Keyboard and mouse inputs are combined and moved into the [InputData](input::InputData) struct.
//!
//! This can be accessed as field `inputs` on the
//! [WindowCallbackData](crate::callbacks::WindowCallbackData) and [DeviceCallbackData](crate::callbacks::DeviceCallbackData) structs (see example).
//!
//! Callbacks are collected in [InputCallbacks](input::InputCallbacks).
//!
//! ## Features
#![doc = document_features::document_features!()]

pub mod callbacks;
pub mod default_ahashmap;
pub mod definitions;
pub mod event_helper;
pub mod input;

#[macro_use]
mod macros;

pub use crate::callbacks::all::Callbacks;
pub use crate::definitions::*;
pub use crate::event_helper::EventHelper;
