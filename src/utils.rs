//! Utilities.

use crate::prelude::*;

/// Propagates the changes made to the state to the window.
pub fn apply_state_to_window(window: &mut Window, state: &Persistent<WindowState>) {
    window.mode = state.mode;

    if let Some((width, height)) = state.resolution {
        window.resolution = WindowResolution::default();
        if let Some(scale) = state.scale {
            window.resolution.set_scale_factor_override(Some(scale as f32));
        }
        window.resolution.set_physical_resolution(width, height);
    }

    if let Some(position) = state.position {
        window.position = WindowPosition::new(position.into());
    }
}

/// Propagates the changes made to the window to the state.
pub fn apply_window_to_state(
    window: &Window,
    state: &mut Persistent<WindowState>,
    winit_window: &winit::window::Window,
) {
    let mode = window.mode;
    let resolution = Some((winit_window.inner_size().width, winit_window.inner_size().height));
    let scale = Some(window.scale_factor() as f64);
    let position = winit_window
        .outer_position()
        .map(|outer_position| Some((outer_position.x, outer_position.y)))
        .unwrap_or_else(|_| {
            match window.position {
                WindowPosition::At(position) => Some((position.x, position.y)),
                _ => state.position,
            }
        });

    let new_state = WindowState { mode, resolution, scale, position, sync: state.sync };
    if new_state != *state.get() {
        state.set(new_state).ok();
        state.sync = false;
    }
}
