use std::path::PathBuf;

use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{AxisId, Ime, KeyboardInput, WindowEvent, TouchPhase},
    window::Theme,
};

use crate::{
    create_callbacks,
    definitions::{CursorState, LineDelta, PixelDelta, QuitWindow},
    input::data::InputData,
    IdLessTouch,
};

#[cfg(feature = "windows_with_device_ids")]
use crate::input::data::InputDataWithId;

create_callbacks! {
    /// A collection of data used for [WindowEvent] callbacks.
    ///
    /// [WindowCallbacks] holds the callbacks themselves.
    pub struct WindowCallbackData: WindowCallbacks<D> {
        ign opt pub position: PhysicalPosition<i32>,
        ign opt pub size: PhysicalSize<u32>,
        clr opt pub focused: bool,
        clr opt pub occluded: bool,
        clr opt pub moved: PhysicalPosition<i32>,
        clr opt pub resized: PhysicalSize<u32>,
        clr opt pub cursor_state: CursorState,
        clr opt pub cursor_entered: bool,
        clr opt pub cursor_moved: PhysicalPosition<f64>,
        clr opt pub quit: QuitWindow,
        clr opt pub scale_factor: f64,
        clr opt pub theme: Theme,
        clr opt pub hover_cancelled: bool,
        clr opt pub mouse_wheel: (LineDelta, PixelDelta),
        clr opt pub smart_magnify: usize,
        clr vec pub text: char,
        clr vec pub ime: Ime,
        clr vec pub touch: IdLessTouch,
        clr vec pub touchpad_pressure: (i64, f32),
        clr vec pub touchpad_magnify: (f64, TouchPhase),
        clr vec pub touchpad_rotate: (f32, TouchPhase),
        clr vec pub axis_motion: (AxisId, f64),
        clr set pub hovered_files: PathBuf,
        clr set pub dropped_files: PathBuf,
        clr cus pub inputs: InputData,
        #[cfg(feature="windows_with_device_ids")]
        clr map pub cursor_entered_with_id: DeviceId => Option<bool>,
        #[cfg(feature="windows_with_device_ids")]
        clr map pub cursor_moved_with_id: DeviceId => Option<PhysicalPosition<f64>>,
        #[cfg(feature="windows_with_device_ids")]
        clr map pub touch_with_id: DeviceId => Vec<IdLessTouch>,
        #[cfg(feature="windows_with_device_ids")]
        clr map pub touchpad_magnify_with_id: DeviceId => Vec<(f64, TouchPhase)>,
        #[cfg(feature="windows_with_device_ids")]
        clr map pub touchpad_rotate_with_id: DeviceId => Vec<(f32, TouchPhase)>,
        #[cfg(feature="windows_with_device_ids")]
        clr map pub touchpad_pressure_with_id: DeviceId => Vec<(i64, f32)>,
        #[cfg(feature="windows_with_device_ids")]
        clr map pub mouse_wheel_with_id: DeviceId => (LineDelta, PixelDelta),
        #[cfg(feature="windows_with_device_ids")]
        clr map pub smart_magnify_with_id: DeviceId => Option<usize>,
        #[cfg(feature="windows_with_device_ids")]
        clr map pub axis_motion_with_id: DeviceId => Vec<(AxisId, f64)>,
        #[cfg(feature="windows_with_device_ids")]
        clr cus pub inputs_with_id: InputDataWithId<D>,
    }
}

impl WindowCallbackData {
    pub fn update(&mut self, event: &WindowEvent) {
        #[allow(unused_variables)]
        match event {
            &WindowEvent::Focused(is_focused) => self.focused = Some(is_focused),
            &WindowEvent::Moved(new_position) => {
                self.moved = Some(new_position);
                self.position = Some(new_position);
            }
            &WindowEvent::Resized(new_size) => {
                self.resized = Some(new_size);
                self.size = Some(new_size);
            }
            &WindowEvent::MouseInput {
                device_id,
                button,
                state,
                ..
            } => {
                self.inputs.update(button, state);

                #[cfg(feature = "windows_with_device_ids")]
                self.inputs_with_id
                    .entry(device_id)
                    .or_default()
                    .update(button, state);
            }
            &WindowEvent::Destroyed => {
                self.quit
                    .get_or_insert(QuitWindow::empty())
                    .insert(QuitWindow::DESTROYED);
            }
            &WindowEvent::ReceivedCharacter(codepoint) => {
                self.text.push(codepoint);
            }
            &WindowEvent::KeyboardInput {
                device_id,
                input:
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        scancode,
                        ..
                    },
                ..
            } => {
                self.inputs.update(scancode, state);

                #[cfg(feature = "windows_with_device_ids")]
                self.inputs_with_id
                    .entry(device_id)
                    .or_default()
                    .update(scancode, state);

                if let Some(key) = virtual_keycode {
                    self.inputs.update(key, state);

                    #[cfg(feature = "windows_with_device_ids")]
                    self.inputs_with_id
                        .entry(device_id)
                        .or_default()
                        .update(key, state);
                }
            }
            &WindowEvent::ModifiersChanged(modifiers) => {
                self.inputs.update_modifiers(modifiers);
            }
            &WindowEvent::MouseWheel {
                device_id, delta, ..
            } => {
                let (lines, pixels) = self.mouse_wheel.get_or_insert(Default::default());
                *lines += delta.try_into().unwrap_or_default();
                *pixels += delta.try_into().unwrap_or_default();

                #[cfg(feature = "windows_with_device_ids")]
                {
                    let (lines, pixels) = self.mouse_wheel_with_id.entry(device_id).or_default();
                    *lines += delta.try_into().unwrap_or_default();
                    *pixels += delta.try_into().unwrap_or_default();
                }
            }
            &WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
                ..
            } => {
                self.axis_motion.push((axis, value));

                #[cfg(feature = "windows_with_device_ids")]
                self.axis_motion_with_id
                    .entry(device_id)
                    .or_default()
                    .push((axis, value));
            }
            WindowEvent::CloseRequested => {
                self.quit
                    .get_or_insert(QuitWindow::empty())
                    .insert(QuitWindow::CLOSE_REQUESTED);
            }
            WindowEvent::DroppedFile(path) => {
                self.dropped_files.insert(path.clone());
            }
            WindowEvent::HoveredFile(path) => {
                self.hovered_files.insert(path.clone());
            }
            WindowEvent::HoveredFileCancelled => self.hover_cancelled = Some(true),
            WindowEvent::Ime(ime) => self.ime.push(ime.clone()),
            &WindowEvent::Occluded(is_occluded) => self.occluded = Some(is_occluded),
            &WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = Some(scale_factor)
            }
            &WindowEvent::ThemeChanged(theme) => self.theme = Some(theme),
            &WindowEvent::Touch(touch) => {
                self.touch.push(touch.into());

                #[cfg(feature = "windows_with_device_ids")]
                self.touch_with_id
                    .entry(touch.device_id)
                    .or_default()
                    .push(touch.into());
            }
            &WindowEvent::TouchpadPressure {
                device_id,
                stage,
                pressure,
                ..
            } => {
                self.touchpad_pressure.push((stage, pressure));

                #[cfg(feature = "windows_with_device_ids")]
                self.touchpad_pressure_with_id
                    .entry(device_id)
                    .or_default()
                    .push((stage, pressure));
            }
            &WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                self.cursor_moved = Some(position);

                #[cfg(feature = "windows_with_device_ids")]
                {
                    *self.cursor_moved_with_id.entry(device_id).or_default() = Some(position);

                }
            },
            &WindowEvent::CursorEntered { device_id } => {
                self.cursor_entered = Some(true);

                #[cfg(feature = "windows_with_device_ids")]
                {
                    *self.cursor_entered_with_id.entry(device_id).or_default() = Some(true);
                }
            }
            &WindowEvent::CursorLeft { device_id } => {
                self.cursor_entered = Some(false);

                #[cfg(feature = "windows_with_device_ids")]
                {
                    *self.cursor_entered_with_id.entry(device_id).or_default() = Some(false);
                }
            }
            &WindowEvent::TouchpadMagnify {
                device_id,
                delta,
                phase,
            } => {
                self.touchpad_magnify.push((delta, phase));

                #[cfg(feature = "windows_with_device_ids")]
                {
                    self.touchpad_magnify_with_id.entry(device_id).or_default().push((delta, phase));
                }
            },
            &WindowEvent::TouchpadRotate {
                device_id,
                delta,
                phase,
            } => {
                self.touchpad_rotate.push((delta, phase));

                #[cfg(feature = "windows_with_device_ids")]
                {
                    self.touchpad_rotate_with_id.entry(device_id).or_default().push((delta, phase));
                }
            },
            &WindowEvent::SmartMagnify { device_id } => {
                *self.smart_magnify.get_or_insert(0) += 1;

                #[cfg(feature = "windows_with_device_ids")]
                {
                    *self.smart_magnify_with_id.entry(device_id).or_default() += 1;
                }
            },
        }
    }
}
