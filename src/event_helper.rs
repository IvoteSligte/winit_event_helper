use std::{
    ops::{Deref, DerefMut},
    time::{Duration, Instant},
};

use winit::event::Event;

use crate::{
    callbacks::all::{CallbackData, Callbacks},
    definitions::CB,
    Quit, QuitWindow,
};

/// A struct holding all the callback functions and user function data.
/// Also has some helper functions.
///
/// Create an instance using [EventHelper::new].
pub struct EventHelper<D> {
    /// User-supplied data that is passed as mutable reference to the event callbacks.
    pub user_data: D,
    /// The data for the event callbacks.
    pub data: CallbackData,
    clear_callback_data: bool,
    call_after: Vec<CB<D>>,
    /// Stores the instants the last two [EventHelper::update]s were called.
    ///
    /// Required for [EventHelper::time_since_previous_step]
    last_steps: [Instant; 2],
    time_since_start: Instant,
    update_count: usize,
    quit: Quit,
}

impl<D: Clone> Clone for EventHelper<D> {
    fn clone(&self) -> Self {
        Self {
            user_data: self.user_data.clone(),
            data: self.data.clone(),
            clear_callback_data: self.clear_callback_data.clone(),
            call_after: self.call_after.clone(),
            last_steps: self.last_steps.clone(),
            time_since_start: self.time_since_start.clone(),
            update_count: self.update_count.clone(),
            quit: self.quit.clone(),
        }
    }
}

impl<D: Default> Default for EventHelper<D> {
    fn default() -> Self {
        Self {
            user_data: Default::default(),
            data: Default::default(),
            clear_callback_data: false,
            call_after: vec![],
            last_steps: [Instant::now(); 2],
            time_since_start: Instant::now(),
            update_count: 0,
            quit: Default::default(),
        }
    }
}

impl<D> Deref for EventHelper<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.user_data
    }
}

impl<D> DerefMut for EventHelper<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.user_data
    }
}

impl<D> EventHelper<D> {
    /// Create an [EventHelper] instance
    pub fn new(user_data: D) -> Self {
        EventHelper {
            user_data,
            data: Default::default(),
            clear_callback_data: false,
            call_after: vec![],
            last_steps: [Instant::now(); 2],
            time_since_start: Instant::now(),
            update_count: 0,
            quit: Default::default(),
        }
    }

    #[inline]
    /// Pass all [Event]s to this function.
    /// When it returns true, a `step` has passed and application logic can be run.
    pub fn update<'a, E: PartialEq>(
        &mut self,
        callbacks: &Callbacks<D>,
        event: &Event<'a, E>,
    ) -> bool {
        self.call_after.clone().iter().for_each(|func| func(self));
        self.call_after.clear();

        if self.clear_callback_data {
            self.clear_callback_data = false;
            self.data.clear();
        }

        if *event == Event::MainEventsCleared {
            self.update_count += 1;
            self.last_steps = [self.last_steps[1], Instant::now()];
            self.data.clone().call_callbacks(self, callbacks);
            self.clear_callback_data = true;
            return true;
        }

        self.data.update(event);

        self.quit.loop_destroyed = self.data.general.loop_destroyed;
        #[cfg(not(feature = "unique_windows"))]
        {
            self.quit.window = self.data.window.quit.clone().unwrap_or(QuitWindow::empty());
        }
        #[cfg(feature = "unique_windows")]
        {
            self.quit.windows = self.data.window.iter().filter_map(|(id, data)| (id, data.quit.clone())).collect()
        }

        false
    }

    /// Returns the number of steps that have passed so far
    pub fn update_count(&self) -> usize {
        self.update_count
    }

    /// Adds the given function to the queue to be called before the next event is handled
    pub fn call_after(&mut self, callback: CB<D>) {
        self.call_after.push(callback);
    }

    /// Returns the time since the [EventHelper] struct was created
    pub fn time_since_start(&self) -> Duration {
        self.time_since_start.elapsed()
    }

    /// Returns the time since the previous time [EventHelper::update] returned `true`
    pub fn time_since_previous_step(&self) -> Duration {
        self.last_steps[0].elapsed()
    }

    /// Sets the `self.quit.user_requested` to `true`
    pub fn request_quit(&mut self) {
        self.quit.user_requested = true;
    }

    /// Returns the quit states of the application
    pub fn quit(&self) -> Quit {
        self.quit.clone()
    }
}
