use ahash::AHashMap;

use crate::definitions::{GenericInput, Modifiers, CB};

/// A storage medium for input callbacks.
///
/// Inputs are keyboard keys and mouse buttons.
pub struct InputCallbacks<D> {
    pub pressed: AHashMap<(Vec<GenericInput>, Modifiers), CB<D>>,
    pub just_pressed: AHashMap<(Vec<GenericInput>, Modifiers), CB<D>>,
    pub just_released: AHashMap<(Vec<GenericInput>, Modifiers), CB<D>>,
}

impl<D> Clone for InputCallbacks<D> {
    fn clone(&self) -> Self {
        Self {
            pressed: self.pressed.clone(),
            just_pressed: self.just_pressed.clone(),
            just_released: self.just_released.clone(),
        }
    }
}

impl<D> Default for InputCallbacks<D> {
    fn default() -> Self {
        Self {
            pressed: Default::default(),
            just_pressed: Default::default(),
            just_released: Default::default(),
        }
    }
}

impl<D> InputCallbacks<D> {
    /// Adds a callback that will activate constantly while the given input is pressed,
    /// overwriting existing callbacks for the same keybinds.
    pub fn pressed<I: Into<GenericInput>>(&mut self, input: I, callback: CB<D>) {
        self.pressed_combination([input.into()], Modifiers::empty(), callback);
    }

    /// Adds a callback that will activate when the given input was just pressed,
    /// overwriting existing callbacks for the same keybinds.
    pub fn just_pressed<I: Into<GenericInput>>(&mut self, input: I, callback: CB<D>) {
        self.just_pressed_combination([input.into()], Modifiers::empty(), callback);
    }

    /// Adds a callback that will activate when the given input was just released,
    /// overwriting existing callbacks for the same keybinds.
    pub fn just_released<I: Into<GenericInput>>(&mut self, input: I, callback: CB<D>) {
        self.just_released_combination([input.into()], Modifiers::empty(), callback);
    }

    /// Adds a callback that will activate constantly while any of the given inputs is pressed,
    /// overwriting existing callbacks for the same keybinds.
    pub fn pressed_any<I: Into<GenericInput>>(
        &mut self,
        inputs: impl IntoIterator<Item = I>,
        callback: CB<D>,
    ) {
        inputs.into_iter().for_each(|input| {
            self.pressed_combination([input.into()], Modifiers::empty(), callback);
        });
    }

    /// Adds a callback that will activate when any of the given inputs was just pressed,
    /// overwriting existing callbacks for the same keybinds.
    pub fn just_pressed_any<I: Into<GenericInput>>(
        &mut self,
        inputs: impl IntoIterator<Item = I>,
        callback: CB<D>,
    ) {
        inputs.into_iter().for_each(|input| {
            self.just_pressed_combination([input.into()], Modifiers::empty(), callback);
        });
    }

    /// Adds a callback that will activate when any of the given inputs was just released,
    /// overwriting existing callbacks for the same keybinds.
    pub fn just_released_any<I: Into<GenericInput>>(
        &mut self,
        inputs: impl IntoIterator<Item = I>,
        callback: CB<D>,
    ) {
        inputs.into_iter().for_each(|input| {
            self.just_released_combination([input.into()], Modifiers::empty(), callback);
        });
    }

    /// Adds a callback that will activate constantly while all of the given inputs are pressed,
    /// overwriting existing callbacks for the same keybinds.
    pub fn pressed_all<I: Into<GenericInput>>(
        &mut self,
        inputs: impl IntoIterator<Item = I>,
        callback: CB<D>,
    ) {
        self.pressed_combination(inputs, Modifiers::empty(), callback);
    }

    /// Adds a callback that will activate when all of the given inputs were just pressed,
    /// overwriting existing callbacks for the same keybinds.
    pub fn just_pressed_all<I: Into<GenericInput>>(
        &mut self,
        inputs: impl IntoIterator<Item = I>,
        callback: CB<D>,
    ) {
        self.just_pressed_combination(inputs, Modifiers::empty(), callback);
    }

    /// Adds a callback that will activate when all of the given inputs were just released,
    /// overwriting existing callbacks for the same keybinds.
    pub fn just_released_all<I: Into<GenericInput>>(
        &mut self,
        inputs: impl IntoIterator<Item = I>,
        callback: CB<D>,
    ) {
        inputs.into_iter().for_each(|input| {
            self.just_released_combination([input.into()], Modifiers::empty(), callback);
        });
    }

    /// Adds a callback that will activate constantly while the given input-modifier combination is pressed.
    ///
    /// Overwrites any previous callback for the same combination.
    ///
    /// To add a callback that activates for only a modifier, set `inputs` to an array.
    pub fn pressed_combination<I: Into<GenericInput>>(
        &mut self,
        inputs: impl IntoIterator<Item = I>,
        modifiers: Modifiers,
        callback: CB<D>,
    ) {
        self.pressed.insert(
            (
                inputs.into_iter().map(|input| input.into()).collect(),
                modifiers,
            ),
            callback,
        );
    }

    /// Adds a callback that will activate when the given input-modifier combination is just pressed.
    ///
    /// Overwrites any previous callback for the same combination.
    ///
    /// To add a callback that activates for only a modifier, set `inputs` to an array.
    pub fn just_pressed_combination<I: Into<GenericInput>>(
        &mut self,
        inputs: impl IntoIterator<Item = I>,
        modifiers: Modifiers,
        callback: CB<D>,
    ) {
        self.just_pressed.insert(
            (
                inputs.into_iter().map(|input| input.into()).collect(),
                modifiers,
            ),
            callback,
        );
    }

    /// Adds a callback that will activate when the given input-modifier combination is just released.
    ///
    /// Overwrites any previous callback for the same combination.
    ///
    /// To add a callback that activates for only a modifier, set `inputs` to an array.
    pub fn just_released_combination<I: Into<GenericInput>>(
        &mut self,
        inputs: impl IntoIterator<Item = I>,
        modifiers: Modifiers,
        callback: CB<D>,
    ) {
        self.just_released.insert(
            (
                inputs.into_iter().map(|input| input.into()).collect(),
                modifiers,
            ),
            callback,
        );
    }
}
