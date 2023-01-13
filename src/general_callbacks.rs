use winit::{
    event::{Event, StartCause},
    window::WindowId,
};

use crate::{
    callback_data::CallbackData,
    definitions::{Quit, CB, CBI},
};

#[derive(Clone)]
/// A struct containing some general events, such as [Event::Suspended];
pub struct GeneralCallbacks<D> {
    pub(crate) loop_destroyed: CB<D>,
    pub(crate) new_events: CBI<D, StartCause>,
    pub(crate) suspended: CB<D>,
    pub(crate) redraw_events_cleared: CB<D>,
    pub(crate) redraw_requested: CBI<D, WindowId>,
    pub(crate) resumed: CB<D>,
}

impl<D> Default for GeneralCallbacks<D> {
    fn default() -> Self {
        Self {
            loop_destroyed: |_| {},
            new_events: |_, _| {},
            suspended: |_| {},
            redraw_events_cleared: |_| {},
            redraw_requested: |_, _| {},
            resumed: |_| {},
        }
    }
}

impl<D> GeneralCallbacks<D> {
    pub fn loop_destroyed(&mut self, callback: CB<D>) {
        self.loop_destroyed = callback;
    }

    pub fn new_events(&mut self, callback: CBI<D, StartCause>) {
        self.new_events = callback;
    }

    pub fn redraw_events_cleared(&mut self, callback: CB<D>) {
        self.redraw_events_cleared = callback;
    }

    pub fn redraw_requested(&mut self, callback: CBI<D, WindowId>) {
        self.redraw_requested = callback;
    }

    pub fn resumed(&mut self, callback: CB<D>) {
        self.resumed = callback;
    }

    pub fn suspended(&mut self, callback: CB<D>) {
        self.suspended = callback;
    }

    pub fn update<'a, E>(&self, data: &mut CallbackData<D>, event: &Event<'a, E>) {
        match event {
            Event::LoopDestroyed => {
                data.quit.insert(Quit::LOOP_DESTROYED);
                (self.loop_destroyed)(data);
            }
            &Event::NewEvents(start_cause) => {
                (self.new_events)(data, start_cause);
            }
            Event::Suspended => (self.suspended)(data),
            Event::Resumed => (self.resumed)(data),
            Event::RedrawEventsCleared => {
                (self.redraw_events_cleared)(data);
            }
            &Event::RedrawRequested(window_id) => {
                (self.redraw_requested)(data, window_id);
            }
            _ => (),
        }
    }
}
