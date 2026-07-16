//! Utilities.

use crate::prelude::*;

/// Propagates the changes made to the state to the window.
///
/// Deliberately never applies a `scale_factor_override`: winit already
/// reports each monitor's real backing scale, and locking in whatever scale
/// happened to be captured once (e.g. a laptop's Retina display) would keep
/// applying that same scale after the window moves to a different monitor
/// (e.g. an external display at a different native scale), making the UI
/// wrong-sized on whichever monitor wasn't the one it was captured on.
///
/// `resolution` is stored and applied in *logical* pixels for the same
/// reason: applying a captured *physical* pixel count via
/// `set_physical_resolution` would pin the window to that many raw pixels
/// regardless of the new monitor's scale factor, shrinking or growing the
/// logical canvas (and everything laid out in it) whenever the window
/// reopens on a monitor with a different DPI than the one it was saved on.
pub fn apply_state_to_window(window: &mut Window, state: &Persistent<WindowState>) {
    window.mode = state.mode;

    if let Some((width, height)) = state.resolution {
        window.resolution = WindowResolution::new(width, height);
    }

    if let Some(position) = state.position {
        window.position = WindowPosition::new(position.into());
    }
}

/// Propagates the changes made to the window to the state.
pub fn apply_window_to_state(
    window: &Window,
    state: &mut Persistent<WindowState>,
    _winit_window: &winit::window::Window,
) {
    let mode = window.mode;
    let resolution = Some((window.width() as u32, window.height() as u32));
    let scale = Some(window.scale_factor() as f64);
    let position = match window.position {
        WindowPosition::At(position) => Some((position.x, position.y)),
        _ => state.position,
    };

    let new_state = WindowState {
        mode,
        resolution,
        position,
        scale,
        auto_scaled: state.auto_scaled,
        sync: state.sync,
    };
    if new_state != *state.get() {
        state.set(new_state).ok();
        state.sync = false;
    }
}
