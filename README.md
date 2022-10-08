# winit_event_helper

[![Latest Version](https://img.shields.io/crates/v/winit_event_helper.svg)](https://crates.io/crates/winit_event_helper)
[![API](https://docs.rs/winit_event_helper/badge.svg)](https://docs.rs/winit_event_helper)

winit_event_helper is a crate for high level winit event handling
using [callback functions](https://en.wikipedia.org/wiki/Callback_(computer_programming))
without taking over the main loop.

## Usage

winit_event_helper comes with the `EventHelper` struct, which handles all the callbacks
and various miscellaneous things.

Pass your events to `EventHelper::update` and run your application calculations when it returns true.
You can also add callbacks for specific winit events with the EventHelper helper functions
or the `Callbacks` struct.

## Example

```rust
use winit_event_helper::EventHelper;
use winit::event::{ElementState, VirtualKeyCode, MouseButton};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;

struct Data {
    counter: usize
}

fn main() {
    let mut event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut eh = EventHelper::new( Data { counter: 0 } );
    
    // is called whenever the given mouse button is in the given state and the window is focused
    eh.window_mouse_input(MouseButton::Left, ElementState::Pressed, |data| data.counter += 1);
    
    // is called whenever a keyboard key is pressed and the window is focused
    eh.window_keyboard_input_any(|_, (keycode, state)| {
        if (state == ElementState::Pressed) {
            println!("{:?}", keycode);
        }
    });
    
    event_loop.run(move |event, _, control_flow| {
        // feed the events to the EventHelper struct
        // returns true when it receives [Event::MainEventsCleared]
        if !eh.update(&event) {
            return;
        }

        // returns true if the given key is being held
        if eh.key_held(VirtualKeyCode::Escape) {
            *control_flow = ControlFlow::Exit;
        }

        // do stuff
    })
}
```