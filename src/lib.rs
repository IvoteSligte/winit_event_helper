//! winit_event_helper is a crate for high level winit event handling
//! using [callback functions](https://en.wikipedia.org/wiki/Callback_(computer_programming))
//! without taking over the main loop.
//!
//! ## Usage
//!
//! winit_event_helper comes with the [EventHelper] struct, which handles all the callbacks
//! and various miscellaneous things.
//! 
//! Pass your events to [EventHelper::update] and run your application calculations when it returns true.
//! You can also add callbacks for specific winit events with the EventHelper helper functions
//! or the [Callbacks] struct.
//!
//! ## Example
//!
//! ```rust
//! use winit_event_helper::EventHelper;
//! use winit::event::{ElementState, VirtualKeyCode, MouseButton};
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
//!     eh.window_mouse_input(MouseButton::Left, ElementState::Pressed, |data| data.counter += 1);
//!     
//!     // is called whenever a keyboard key is pressed and the window is focused
//!     eh.window_keyboard_input_any(|_, (keycode, state)| {
//!         if (state == ElementState::Pressed) {
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
//!         // returns the time the key has been held for when held, or None
//!         if eh.key_held(VirtualKeyCode::Escape).is_some() {
//!             *control_flow = ControlFlow::Exit;
//!         }
//! 
//!         // do stuff
//!     })
//! }
//! ```
//! 
//! ## Function and field names
//! 
//! `winit_event_helper` has a function for adding a callback for every winit event except [Event::UserEvent].
//! The [Callbacks] struct has fields of identical names.
//! 
//! These names are almost the same as their respective event names, with a few changes:
//! - The names are completely in [snake_case](https://rust-lang.github.io/api-guidelines/naming.html)
//!   and appropriate changes have been made as such.
//! - `::` is replaced with `_`
//! - `WindowEvent` is replaced with `window`
//! - `DeviceEvent` is replaced with `device`
//! 
//! For a complete overview of functions, see [EventHelper].
//! 
//! ## Features
#![doc = document_features::document_features!()]

use ahash::{AHashMap, AHashSet};
use bitflags::bitflags;
use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    time::Instant,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        AxisId, ButtonId, DeviceEvent, ElementState, Event, Ime, KeyboardInput, ModifiersState,
        MouseButton, MouseScrollDelta, StartCause, Touch, TouchPhase, VirtualKeyCode, WindowEvent,
    },
    window::{Theme, WindowId},
};

/// A callback function with no inputs
pub type CB<D> = fn(&mut EventHelper<D>);
/// A callback function with one input
pub type CBI<D, I> = fn(&mut EventHelper<D>, I);

bitflags! {
    /// Bitflags for quit requests
    pub struct Quit: u8 {
        const USER_REQUESTED   = 0b0000_0001;
        const LOOP_DESTROYED   = 0b0000_0010;
        const WINDOW_DESTROYED = 0b0000_0100;
    }
}

/// A struct for all callback functions, each corresponding to a winit event
#[derive(Clone)]
pub struct Callbacks<D> {
    // global
    loop_destroyed: CB<D>,
    new_events: CBI<D, StartCause>,
    suspended: CB<D>,
    redraw_events_cleared: CB<D>,
    redraw_requested: CBI<D, WindowId>,
    resumed: CB<D>,
    // device
    device_added: CB<D>,
    device_button_any: CBI<D, (ButtonId, ElementState)>,
    device_button: AHashMap<(ButtonId, ElementState), CB<D>>,
    device_key_any: CBI<D, (VirtualKeyCode, ElementState)>,
    device_key: AHashMap<(VirtualKeyCode, ElementState), CB<D>>,
    device_motion: CBI<D, (AxisId, f64)>,
    device_mouse_motion: CBI<D, (f64, f64)>,
    device_mouse_wheel: CBI<D, MouseScrollDelta>,
    device_removed: CB<D>,
    device_text: CBI<D, char>,
    // window
    window_axis_motion: CBI<D, (AxisId, f64)>,
    window_close_requested: CB<D>,
    window_cursor_entered: CB<D>,
    window_cursor_left: CB<D>,
    window_cursor_moved: CBI<D, PhysicalPosition<f64>>,
    window_destroyed: CB<D>,
    window_dropped_file: CBI<D, PathBuf>,
    window_focused: CBI<D, bool>,
    window_hovered_file: CBI<D, PathBuf>,
    window_hovered_file_canceled: CB<D>,
    window_ime: CBI<D, Ime>,
    window_keyboard_input_any: CBI<D, (VirtualKeyCode, ElementState)>,
    window_keyboard_input: AHashMap<(VirtualKeyCode, ElementState), CB<D>>,
    window_modifiers_changed: CBI<D, ModifiersState>,
    window_mouse_input_any: CBI<D, (MouseButton, ElementState)>,
    window_mouse_input: AHashMap<(MouseButton, ElementState), CB<D>>,
    window_mouse_wheel: CBI<D, (MouseScrollDelta, TouchPhase)>,
    window_moved: CBI<D, PhysicalPosition<i32>>,
    window_occluded: CBI<D, bool>,
    window_received_character: CBI<D, char>,
    window_resized: CBI<D, PhysicalSize<u32>>,
    window_scale_factor_changed: CBI<D, f64>,
    window_theme_changed: CBI<D, Theme>,
    window_touch: CBI<D, Touch>,
    window_touchpad_pressure: CBI<D, (f32, i64)>,
}

impl<D> Default for Callbacks<D> {
    fn default() -> Callbacks<D> {
        Callbacks {
            // global
            loop_destroyed: |_| {},
            new_events: |_, _| {},
            redraw_events_cleared: |_| {},
            redraw_requested: |_, _| {},
            resumed: |_| {},
            suspended: |_| {},
            // device
            device_added: |_| {},
            device_button: AHashMap::new(),
            device_button_any: |_, _| {},
            device_key_any: |_, _| {},
            device_key: AHashMap::new(),
            device_motion: |_, _| {},
            device_mouse_motion: |_, _| {},
            device_mouse_wheel: |_, _| {},
            device_removed: |_| {},
            device_text: |_, _| {},
            // window
            window_axis_motion: |_, _| {},
            window_close_requested: |_| {},
            window_cursor_entered: |_| {},
            window_cursor_left: |_| {},
            window_cursor_moved: |_, _| {},
            window_destroyed: |_| {},
            window_dropped_file: |_, _| {},
            window_focused: |_, _| {},
            window_hovered_file: |_, _| {},
            window_hovered_file_canceled: |_| {},
            window_ime: |_, _| {},
            window_keyboard_input_any: |_, _| {},
            window_keyboard_input: AHashMap::new(),
            window_modifiers_changed: |_, _| {},
            window_mouse_input_any: |_, _| {},
            window_mouse_input: AHashMap::new(),
            window_mouse_wheel: |_, _| {},
            window_moved: |_, _| {},
            window_occluded: |_, _| {},
            window_received_character: |_, _| {},
            window_resized: |_, _| {},
            window_scale_factor_changed: |_, _| {},
            window_theme_changed: |_, _| {},
            window_touch: |_, _| {},
            window_touchpad_pressure: |_, _| {},
        }
    }
}

/// A struct holding all the callback functions and user function data.
/// Also has several helper functions.
///
/// Create an empty instance using EventHelper::new()
/// or an instance with callbacks pre-set using EventHelper::with_callbacks().
///
/// Add callbacks to the struct using the functions directly corresponding to winit events.
/// Examples:
/// - WindowEvent::KeyboardInput == window_keyboard_input()
/// - DeviceEvent::MouseMotion   == device_mouse_motion()
pub struct EventHelper<D> {
    pub data: D,
    callbacks: Callbacks<D>,
    call_after: Vec<CB<D>>,
    // (held, released)
    buttons: (AHashMap<MouseButton, Instant>, AHashSet<MouseButton>),
    // (held, released)
    keys: (AHashMap<VirtualKeyCode, Instant>, AHashSet<VirtualKeyCode>),
    modifiers: ModifiersState,
    mouse_wheel: MouseScrollDelta,
    focused: bool,
    text: Vec<char>,
    quit: Quit,
    steps: (Instant, Instant),
    time: Instant,
}

impl<D> EventHelper<D> {
    /// Create an EventHelper instance
    /// 
    /// The `data` object holds all the variables you need inside of the callback functions.
    pub fn new(data: D) -> EventHelper<D> {
        EventHelper {
            data,
            callbacks: Callbacks::default(),
            call_after: vec![],
            buttons: (AHashMap::new(), AHashSet::new()),
            keys: (AHashMap::new(), AHashSet::new()),
            modifiers: ModifiersState::empty(),
            mouse_wheel: MouseScrollDelta::LineDelta(0.0, 0.0),
            focused: true,
            text: vec![],
            quit: Quit::empty(),
            steps: (Instant::now(), Instant::now()),
            time: Instant::now(),
        }
    }

    /// Create an EventHelper instance with pre-set callbacks.
    /// 
    /// The `data` object holds all the variables you need inside of the callback functions.
    /// 
    /// The `callbacks` object holds all the callback functions.
    pub fn with_callbacks(data: D, callbacks: Callbacks<D>) -> EventHelper<D> {
        EventHelper {
            callbacks,
            ..EventHelper::new(data)
        }
    }

    #[inline]
    /// Pass all winit events to this function.
    /// When it returns true, a step has passed and application logic can be run.
    pub fn update<'a, E>(&mut self, event: &Event<'a, E>) -> bool {
        self.call_after.clone().iter().for_each(|f| f(self));
        self.call_after.clear();

        match event {
            Event::DeviceEvent { event, .. } => {
                self.update_device_event(event);
            }
            Event::WindowEvent { event, .. } => {
                self.update_window_event(event);
            }
            Event::LoopDestroyed => {
                self.quit.insert(Quit::LOOP_DESTROYED);
                (self.callbacks.loop_destroyed)(self);
            }
            &Event::NewEvents(start_cause) => {
                (self.callbacks.new_events)(self, start_cause);
            }
            Event::Suspended => (self.callbacks.suspended)(self),
            Event::Resumed => (self.callbacks.resumed)(self),
            Event::RedrawEventsCleared => {
                (self.callbacks.redraw_events_cleared)(self);
            }
            &Event::RedrawRequested(window_id) => {
                (self.callbacks.redraw_requested)(self, window_id);
            }
            Event::MainEventsCleared => {
                self.steps = (self.steps.1, Instant::now());
        
                self.call_after(|eh| {
                    eh.text.clear();
                    eh.mouse_wheel = MouseScrollDelta::LineDelta(0.0, 0.0);

                    eh.buttons.1.clear();
                    eh.keys.1.clear();
                });

                return true
            },
            _ => (),
        }
        false
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
                    #[cfg(feature = "save_device_inputs")]
                    self.update_keys_held(key, state);
                    (self.callbacks.device_key_any)(self, (key, state));
                    if let Some(func) = self.callbacks.device_key.get_mut(&(key, state)) {
                        func(self);
                    }
                }
            }
            &DeviceEvent::MouseMotion { delta } => {
                (self.callbacks.device_mouse_motion)(self, delta);
            }
            &DeviceEvent::MouseWheel { delta } => {
                #[cfg(feature = "save_device_inputs")]
                {
                    self.mouse_wheel = delta;
                }
                (self.callbacks.device_mouse_wheel)(self, delta);
            }
            &DeviceEvent::Added => {
                (self.callbacks.device_added)(self);
            }
            &DeviceEvent::Removed => {
                (self.callbacks.device_removed)(self);
            }
            &DeviceEvent::Motion { axis, value } => {
                (self.callbacks.device_motion)(self, (axis, value));
            }
            &DeviceEvent::Button { button, state } => {
                #[cfg(feature = "save_device_inputs")]
                {
                    let button = match button {
                        0 => MouseButton::Left,
                        1 => MouseButton::Middle,
                        2 => MouseButton::Right,
                        _ => MouseButton::Other(button as u16),
                    };
                    self.update_buttons_held(button, state);
                }
                if let Some(func) = self.callbacks.device_button.get_mut(&(button, state)) {
                    func(self);
                }
                (self.callbacks.device_button_any)(self, (button, state));
            }
            &DeviceEvent::Text { codepoint } => {
                #[cfg(feature = "save_device_inputs")]
                self.text.push(codepoint);
                (self.callbacks.device_text)(self, codepoint);
            }
        }
    }

    #[inline]
    fn update_window_event(&mut self, event: &WindowEvent) {
        match event {
            &WindowEvent::CloseRequested => (self.callbacks.window_close_requested)(self),
            &WindowEvent::CursorEntered { .. } => (self.callbacks.window_cursor_entered)(self),
            &WindowEvent::CursorLeft { .. } => (self.callbacks.window_cursor_left)(self),
            &WindowEvent::CursorMoved { position, .. } => {
                (self.callbacks.window_cursor_moved)(self, position);
            }
            &WindowEvent::Focused(focused) => {
                self.focused = focused;
                (self.callbacks.window_focused)(self, focused);
            }
            &WindowEvent::MouseInput { button, state, .. } => {
                #[cfg(not(feature = "save_device_inputs"))]
                self.update_buttons_held(button, state);
                (self.callbacks.window_mouse_input_any)(self, (button, state));
                if let Some(func) = self.callbacks.window_mouse_input.get_mut(&(button, state)) {
                    func(self);
                }
            }
            &WindowEvent::Moved(position) => {
                (self.callbacks.window_moved)(self, position);
            }
            &WindowEvent::Resized(size) => {
                (self.callbacks.window_resized)(self, size);
            }
            &WindowEvent::Destroyed => {
                self.quit.insert(Quit::WINDOW_DESTROYED);
                (self.callbacks.window_destroyed)(self);
            }
            WindowEvent::DroppedFile(path) => {
                (self.callbacks.window_dropped_file)(self, path.clone());
            }
            WindowEvent::HoveredFile(path) => {
                (self.callbacks.window_hovered_file)(self, path.clone());
            }
            &WindowEvent::HoveredFileCancelled => {
                (self.callbacks.window_hovered_file_canceled)(self);
            }
            &WindowEvent::ReceivedCharacter(codepoint) => {
                #[cfg(not(feature = "save_device_inputs"))]
                self.text.push(codepoint);
                (self.callbacks.window_received_character)(self, codepoint);
            }
            &WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    },
                ..
            } => {
                if let Some(key) = virtual_keycode {
                    #[cfg(not(feature = "save_device_inputs"))]
                    self.update_keys_held(key, state);
                    if let Some(func) = self.callbacks.window_keyboard_input.get_mut(&(key, state))
                    {
                        func(self);
                    }
                    (self.callbacks.window_keyboard_input_any)(self, (key, state));
                }
            }
            &WindowEvent::ModifiersChanged(modifiers) => {
                (self.callbacks.window_modifiers_changed)(self, modifiers);
            }
            WindowEvent::Ime(ime) => {
                (self.callbacks.window_ime)(self, ime.clone());
            }
            &WindowEvent::TouchpadPressure {
                pressure, stage, ..
            } => {
                (self.callbacks.window_touchpad_pressure)(self, (pressure, stage));
            }
            &WindowEvent::AxisMotion { axis, value, .. } => {
                (self.callbacks.window_axis_motion)(self, (axis, value));
            }
            &WindowEvent::Touch(touch) => {
                (self.callbacks.window_touch)(self, touch);
            }
            &WindowEvent::ThemeChanged(theme) => {
                (self.callbacks.window_theme_changed)(self, theme);
            }
            &WindowEvent::Occluded(occluded) => {
                (self.callbacks.window_occluded)(self, occluded);
            }
            &WindowEvent::MouseWheel { delta, phase, .. } => {
                #[cfg(not(feature = "save_device_inputs"))]
                {
                    self.mouse_wheel = delta;
                }
                (self.callbacks.window_mouse_wheel)(self, (delta, phase));
            }
            &WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                (self.callbacks.window_scale_factor_changed)(self, scale_factor);
            }
        }
    }

    fn update_buttons_held(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.buttons.0.entry(button).or_insert(Instant::now());
            }
            ElementState::Released => {
                self.buttons.0.remove(&button);
                self.buttons.1.insert(button);
            }
        };
    }

    fn update_keys_held(&mut self, key: VirtualKeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.keys.0.entry(key).or_insert(Instant::now());
            }
            ElementState::Released => {
                self.keys.0.remove(&key);
                self.keys.1.insert(key);
            }
        };
    }

    /// Returns the time since the mouse button was first pressed in seconds if pressed, else returns None
    pub fn button_held(&self, button: MouseButton) -> Option<f64> {
        self.buttons.0
            .get(&button)
            .map(|&t| t.elapsed().as_secs_f64())
    }

    /// Returns all mouse buttons currently pressed
    pub fn buttons_held(&self) -> impl Iterator<Item = &MouseButton> + '_ {
        self.buttons.0.keys()
    }

    /// Returns true when the given button goes from 'not pressed' to 'pressed'
    pub fn button_pressed(&self, button: MouseButton) -> bool {
        match self.buttons.0.get(&button) {
            Some(time_held) => {
                time_held.elapsed().as_secs_f64() < self.steps.0.elapsed().as_secs_f64()
            },
            None => false,
        }
    }

    /// Returns true when the given button goes from 'pressed' to 'not pressed'
    pub fn button_released(&self, button: MouseButton) -> bool {
        self.buttons.1.contains(&button)
    }

    /// Returns the time since the key was first pressed in seconds if pressed, else returns None
    pub fn key_held(&self, key: VirtualKeyCode) -> Option<f64> {
        self.keys.0.get(&key).map(|&t| t.elapsed().as_secs_f64())
    }

    /// Returns all keys currently pressed
    pub fn keys_held(&self) -> impl Iterator<Item = &VirtualKeyCode> + '_ {
        self.keys.0.keys()
    }

    /// Returns true when the given key goes from 'not pressed' to 'pressed'
    pub fn key_pressed(&self, key: VirtualKeyCode) -> bool {
        match self.keys.0.get(&key) {
            Some(time_held) => {
                time_held.elapsed().as_secs_f64() < self.steps.0.elapsed().as_secs_f64()
            },
            None => false,
        }
    }

    /// Returns true when the given key goes from 'pressed' to 'not pressed'
    pub fn key_released(&self, key: VirtualKeyCode) -> bool {
        self.keys.1.contains(&key)
    }

    /// Adds the given function to the queue to be called when the next event is received
    pub fn call_after(&mut self, callback: CB<D>) {
        self.call_after.push(callback);
    }
    
    /// Returns true if the window is focused
    pub fn focused(&self) -> bool {
        self.focused
    }

    /// Returns true if any alt key is being held
    pub fn alt_held(&self) -> bool {
        self.modifiers.alt()
    }

    /// Returns true if any ctrl key is being held
    pub fn ctrl_held(&self) -> bool {
        self.modifiers.ctrl()
    }

    /// Returns true if the logo key is being held
    pub fn logo_held(&self) -> bool {
        self.modifiers.logo()
    }

    /// Returns true if any shift key is being held
    pub fn shift_held(&self) -> bool {
        self.modifiers.shift()
    }

    /// Makes quit_requested(Quit::USER_REQUESTED) return true during the next step
    pub fn quit(&mut self) {
        self.quit.insert(Quit::USER_REQUESTED);
    }

    /// Returns true if a quit request of the given request type has been received
    pub fn quit_requested(&self, request_type: Quit) -> bool {
        self.quit.intersects(request_type)
    }

    /// Returns the time since the EventHelper struct was created
    pub fn secs_since_start(&self) -> f64 {
        self.time.elapsed().as_secs_f64()
    }

    /// Returns the time since the last update()
    pub fn secs_since_last_update(&self) -> f64 {
        self.steps.0.elapsed().as_secs_f64()
    }

    /// Returns the unicode characters received in the last step in the order received
    pub fn text(&self) -> Vec<char> {
        self.text.clone()
    }
}

/// Callbacks corresponding to winit events
impl<D> EventHelper<D> {
    pub fn device_added(&mut self, callback: CB<D>) {
        self.callbacks.device_added = callback;
    }

    /// Callback is called when an event with the given button and state is received
    pub fn device_button(&mut self, button: u32, state: ElementState, callback: CB<D>) {
        self.callbacks
            .device_button
            .insert((button, state), callback);
    }

    /// Callback is called for any button/state combination
    pub fn device_button_any(&mut self, callback: CBI<D, (u32, ElementState)>) {
        self.callbacks.device_button_any = callback;
    }

    /// Callback is called when an event with the given key and state is received
    pub fn device_key(&mut self, key: VirtualKeyCode, state: ElementState, callback: CB<D>) {
        self.callbacks.device_key.insert((key, state), callback);
    }

    /// Callback is called for any key/state combination
    pub fn device_key_any(&mut self, callback: CBI<D, (VirtualKeyCode, ElementState)>) {
        self.callbacks.device_key_any = callback;
    }

    pub fn device_motion(&mut self, callback: CBI<D, (u32, f64)>) {
        self.callbacks.device_motion = callback;
    }

    pub fn device_mouse_motion(&mut self, callback: CBI<D, (f64, f64)>) {
        self.callbacks.device_mouse_motion = callback;
    }

    pub fn device_mouse_wheel(&mut self, callback: CBI<D, MouseScrollDelta>) {
        self.callbacks.device_mouse_wheel = callback;
    }

    pub fn device_removed(&mut self, callback: CB<D>) {
        self.callbacks.device_removed = callback;
    }

    pub fn device_text(&mut self, callback: CBI<D, char>) {
        self.callbacks.device_text = callback;
    }

    pub fn loop_destroyed(&mut self, callback: CB<D>) {
        self.callbacks.loop_destroyed = callback;
    }

    pub fn new_events(&mut self, callback: CBI<D, StartCause>) {
        self.callbacks.new_events = callback;
    }

    pub fn redraw_events_cleared(&mut self, callback: CB<D>) {
        self.callbacks.redraw_events_cleared = callback;
    }

    pub fn redraw_requested(&mut self, callback: CBI<D, WindowId>) {
        self.callbacks.redraw_requested = callback;
    }

    pub fn resumed(&mut self, callback: CB<D>) {
        self.callbacks.resumed = callback;
    }

    pub fn suspended(&mut self, callback: CB<D>) {
        self.callbacks.suspended = callback;
    }

    pub fn window_axis_motion(&mut self, callback: CBI<D, (u32, f64)>) {
        self.callbacks.window_axis_motion = callback;
    }

    pub fn window_close_requested(&mut self, callback: CB<D>) {
        self.callbacks.window_close_requested = callback;
    }

    pub fn window_cursor_entered(&mut self, callback: CB<D>) {
        self.callbacks.window_cursor_entered = callback;
    }

    pub fn window_cursor_left(&mut self, callback: CB<D>) {
        self.callbacks.window_cursor_left = callback;
    }

    pub fn window_cursor_moved(&mut self, callback: CBI<D, PhysicalPosition<f64>>) {
        self.callbacks.window_cursor_moved = callback;
    }

    pub fn window_destroyed(&mut self, callback: CB<D>) {
        self.callbacks.window_destroyed = callback;
    }

    pub fn window_dropped_file(&mut self, callback: CBI<D, PathBuf>) {
        self.callbacks.window_dropped_file = callback;
    }

    pub fn window_focused(&mut self, callback: CBI<D, bool>) {
        self.callbacks.window_focused = callback;
    }

    pub fn window_hovered_file(&mut self, callback: CBI<D, PathBuf>) {
        self.callbacks.window_hovered_file = callback;
    }

    pub fn window_hovered_file_canceled(&mut self, callback: CB<D>) {
        self.callbacks.window_hovered_file_canceled = callback;
    }

    pub fn window_ime(&mut self, callback: CBI<D, Ime>) {
        self.callbacks.window_ime = callback;
    }

    /// Callback is called when an event with the given key and state is received
    pub fn window_keyboard_input(
        &mut self,
        key: VirtualKeyCode,
        state: ElementState,
        callback: CB<D>,
    ) {
        self.callbacks
            .window_keyboard_input
            .insert((key, state), callback);
    }

    /// Callback is called for any key/state combination
    pub fn window_keyboard_input_any(&mut self, callback: CBI<D, (VirtualKeyCode, ElementState)>) {
        self.callbacks.window_keyboard_input_any = callback;
    }

    pub fn window_modifiers_changed(&mut self, callback: CBI<D, ModifiersState>) {
        self.callbacks.window_modifiers_changed = callback;
    }

    /// Callback is called when an event with the given button and state is received
    pub fn window_mouse_input(
        &mut self,
        button: MouseButton,
        state: ElementState,
        callback: CB<D>,
    ) {
        self.callbacks
            .window_mouse_input
            .insert((button, state), callback);
    }

    /// Callback is called for any button/state combination
    pub fn window_mouse_input_any(&mut self, callback: CBI<D, (MouseButton, ElementState)>) {
        self.callbacks.window_mouse_input_any = callback;
    }

    pub fn window_mouse_wheel(&mut self, callback: CBI<D, (MouseScrollDelta, TouchPhase)>) {
        self.callbacks.window_mouse_wheel = callback;
    }

    pub fn window_moved(&mut self, callback: CBI<D, PhysicalPosition<i32>>) {
        self.callbacks.window_moved = callback;
    }

    pub fn window_occluded(&mut self, callback: CBI<D, bool>) {
        self.callbacks.window_occluded = callback;
    }

    pub fn window_received_character(&mut self, callback: CBI<D, char>) {
        self.callbacks.window_received_character = callback;
    }

    pub fn window_resized(&mut self, callback: CBI<D, PhysicalSize<u32>>) {
        self.callbacks.window_resized = callback;
    }

    pub fn window_scale_factor_changed(&mut self, callback: CBI<D, f64>) {
        self.callbacks.window_scale_factor_changed = callback;
    }

    pub fn window_theme_changed(&mut self, callback: CBI<D, Theme>) {
        self.callbacks.window_theme_changed = callback;
    }

    pub fn window_touch(&mut self, callback: CBI<D, Touch>) {
        self.callbacks.window_touch = callback;
    }

    pub fn window_touchpad_pressure(&mut self, callback: CBI<D, (f32, i64)>) {
        self.callbacks.window_touchpad_pressure = callback;
    }
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
