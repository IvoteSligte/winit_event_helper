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
