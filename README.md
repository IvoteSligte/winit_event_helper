# winit_event_helper

[![Latest Version](https://img.shields.io/crates/v/winit_event_helper.svg)](https://crates.io/crates/winit_event_helper)
[![API](https://docs.rs/winit_event_helper/badge.svg)](https://docs.rs/winit_event_helper)

`winit_event_helper` is a crate for flattened winit event handling
using [callback functions](https://en.wikipedia.org/wiki/Callback_(computer_programming))
without taking over the main loop.

## Usage
`winit_event_helper` comes with the `EventHelper` struct, which handles all the callbacks
and various miscellaneous things.

Pass your events to `EventHelper::update` and run your application logic when it returns `true`.

You can add callbacks for specific `winit` events with the `Callbacks` struct.

## Example
```rust
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_event_helper::*;

struct Data {
    counter: usize,
}

fn main() {
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let mut eh = EventHelper::new(Data { counter: 0 });
    let mut callbacks = Callbacks::<Data>::empty();

    // is called whenever one of the given inputs was just pressed
    callbacks
        .window
        .inputs
        .just_pressed_all([GenericInput::from(MouseButton::Left), KeyCode::Space.into()], |eh| {
            eh.counter += 1
        });
    
    event_loop.run(move |event, _, control_flow| {
        // feed the events to the [EventHelper] struct
        // returns true when it receives [Event::MainEventsCleared]
        if !eh.update(&callbacks, &event) {
            return;
        }

        // exits the application when the key combination CTRL + ESC has been released
        if eh.data.window.inputs.just_released_combination([KeyCode::Escape], Modifiers::CTRL) {
            *control_flow = ControlFlow::Exit;
        }

        println!("{}", eh.counter);

        // do stuff
    })
}
```