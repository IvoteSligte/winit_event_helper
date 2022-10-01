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
//! ```
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

/// mod that contains the [EventHelper] struct and several macros
pub mod winit_event_helper;

pub use winit_event_helper::EventHelper;
