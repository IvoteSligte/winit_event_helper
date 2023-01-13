use crate::callback_data::CallbackData;

use bitflags::bitflags;

/// A callback function with no inputs
pub type CB<D> = fn(&mut CallbackData<D>);
/// A callback function with one input
pub type CBI<D, I> = fn(&mut CallbackData<D>, I);

bitflags! {
    /// Bitflags for quit requests
    pub struct Quit: u8 {
        const USER_REQUESTED   = 0b0000_0001;
        const LOOP_DESTROYED   = 0b0000_0010;
        const WINDOW_DESTROYED = 0b0000_0100;
    }
}