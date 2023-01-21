use winit::{
    event::{Event, StartCause},
    window::WindowId,
};

use crate::create_callbacks;

create_callbacks! {
    /// A collection of data used for general event callbacks.
    ///
    /// [GeneralCallbacks] holds the callbacks themselves.
    ///
    /// General events are all events that do not belong to the device and window categories.
    pub struct GeneralCallbackData: GeneralCallbacks<D> {
        clr boo pub suspended: bool,
        clr boo pub resumed: bool,
        clr boo pub redraw_events_cleared: bool,
        clr boo pub loop_destroyed: bool,
        clr opt pub new_events: StartCause,
        clr set pub redraw_requested: WindowId,
    }
}

impl GeneralCallbackData {
    pub fn update<'a, E>(&mut self, event: &Event<'a, E>) {
        match event {
            Event::LoopDestroyed => self.loop_destroyed = true,
            &Event::NewEvents(start_cause) => self.new_events = Some(start_cause),
            Event::Suspended => self.suspended = true,
            Event::Resumed => self.resumed = true,
            Event::RedrawRequested(window_id) => {
                self.redraw_requested.insert(*window_id);
            }
            Event::RedrawEventsCleared => self.redraw_events_cleared = true,
            _ => (),
        }
    }
}
