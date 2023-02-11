# 0.5.0

## Non-breaking

- Added combined support for `WindowEvent::CursorEntered` and `WindowEvent::CursorLeft` through the `cursor_entered` field.
- Added support for `WindowEvent::CursorMoved`.
- Added support for the `winit` `0.28.0` events `WindowEvent::TouchpadMagnify`, `WindowEvent::SmartMagnify` and `WindowEvent::TouchpadRotate`
- Updated the example

## Breaking

- Updated `winit` to version `0.28.1`
- Renamed `Callbacks::new()` to `Callbacks::empty()`

# 0.4.1

## Non-breaking

- Renamed `time_since_previous_update` to `time_since_previous_step` and changed its functionality. 
  It now returns the time since the previous time `EventHelper::update` returned *true*.

# 0.4.0

### Revision

This updates changes how the majority of the library works,
which most likely breaks all code currently written using previous versions of the library.
You are free to continue using older versions, but they will not be updated to support future winit releases.

### Changes (incomplete)

- Renamed *many* functions
- Callback functions now take the `CallbackData` struct instead of the `EventHelper` struct
- Moved window events to `WindowCallbacks`
- Moved device events to `DeviceCallbacks`
- Moved general (global) events to `GeneralCallbacks`
- text() now returns a `String` instead of a `Vec<char>`
- Callback structs are now generated using a dedicated macro
- Added `Input` struct which handles keyboard and mouse inputs (inspired by the `bevy_input` crate)
- Added support for multiple devices (feature `multiple_devices`)
- Added support for multiple windows (feature `multiple_windows`)
- Removed the `Quit` struct in favor of handling Event::LoopDestroyed separately from
  WindowEvent::Destroyed and WindowEvent::CloseRequested
- Combined `WindowEvent::Destroyed` and `WindowEvent::CloseRequested` into `QuitWindow`

# 0.3.2

### Non-breaking

- Made callbacks with trigger `ElementState2::Held` also trigger for `ElementState2::Pressed`

# 0.3.1

### Breaking

- Added a new struct, `ElementState2`, as a complement to `winit::Event::ElementState`.
    It adds a `Pressed` state that is triggered on press.
- Replaced all instances of `ElementState` with `ElementState2`

# 0.3.0

### Breaking

- Renamed `WindowEvent` functions and fields to window_#name#
- Renamed `DeviceEvent` functions and fields to device_#name#
- Renamed `raw_mouse_delta` to `device_mouse_motion`
- Renamed `raw_mouse_scroll` to `device_mouse_wheel`
- `key_held` and `button_held` now return how long the element has been held for (or None)
- Removed macros

### Non-breaking

- Added a `Callbacks` struct (implements Default)
- Switched from `Option`s to empty closures for default values
- Added all winit `Event`s except `Event::UserEvent`
- Implemented several more helper functions
- Implemented the compile feature `save_device_inputs`
which determines whether device or window events are used for inputs.
- Added a `Quit` struct with bitflags from the `bitflags` crate
- Added documentation for `save_device_inputs` with the `document-features` crate
