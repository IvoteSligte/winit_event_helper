use std::{
    ops::{Deref, DerefMut},
    time::{Duration, Instant},
};

use winit::event::Event;

use crate::{callback_data::CallbackData, callbacks::Callbacks, definitions::CB};

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
    pub callback_data: CallbackData<D>,
    pub callbacks: Callbacks<D>,
    call_after: Vec<CB<D>>,
    last_updates: [Instant; 2],
    time_since_start: Instant,
}

impl<D> EventHelper<D> {
    /// Create an EventHelper instance
    ///
    /// The `data` object holds all the variables you need inside of the callback functions.
    pub fn new(user_data: D) -> EventHelper<D> {
        EventHelper {
            callback_data: CallbackData::new(user_data),
            callbacks: Callbacks::default(),
            call_after: vec![],
            last_updates: [Instant::now(); 2],
            time_since_start: Instant::now(),
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
        self.last_updates = [self.last_updates[1], Instant::now()];

        self.call_after.clone().iter().for_each(|f| f(self));
        self.call_after.clear();

        self.callback_data.update();

        self.callbacks.update(&mut self.callback_data, event)
    }

    /// Adds the given function to the queue to be called when the next event is received
    pub fn call_after(&mut self, callback: CB<D>) {
        self.call_after.push(callback);
    }

    /// Returns the time since the `EventHelper` struct was created
    pub fn time_since_start(&self) -> Duration {
        self.time_since_start.elapsed()
    }

    /// Returns the time since the previous update()
    pub fn time_since_previous_update(&self) -> Duration {
        self.last_updates[0].elapsed()
    }
}

impl<D> Deref for EventHelper<D> {
    type Target = CallbackData<D>;

    fn deref(&self) -> &Self::Target {
        &self.callback_data
    }
}

impl<D> DerefMut for EventHelper<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.callback_data
    }
}
