//! Systems.

use crate::prelude::*;

/// System to update persistent state when window is moved.
pub fn on_persistent_window_moved(
    mut window_moved_events: EventReader<WindowMoved>,
    mut persistent_windows: Query<(Entity, &Window, &mut Persistent<WindowState>)>,
    winit_windows: NonSend<WinitWindows>,
) {
    if window_moved_events.is_empty() {
        return;
    }

    for event in window_moved_events.iter() {
        if let Some((entity, window, mut state)) =
            persistent_windows.iter_mut().find(|(entity, _, _)| event.entity == *entity)
        {
            let winit_window_id = &winit_windows.entity_to_winit[&entity];
            let winit_window = &winit_windows.windows[winit_window_id];

            utils::apply_window_to_state(window, &mut state, winit_window);
        }
    }
}

/// System to update persistent state when window is resized.
pub fn on_persistent_window_resized(
    mut window_resized_events: EventReader<WindowResized>,
    mut persistent_windows: Query<(Entity, &Window, &mut Persistent<WindowState>)>,
    winit_windows: NonSend<WinitWindows>,
) {
    if window_resized_events.is_empty() {
        return;
    }

    for event in window_resized_events.iter() {
        if let Some((entity, window, mut state)) =
            persistent_windows.iter_mut().find(|(entity, _, _)| event.window == *entity)
        {
            let winit_window_id = &winit_windows.entity_to_winit[&entity];
            let winit_window = &winit_windows.windows[winit_window_id];

            utils::apply_window_to_state(window, &mut state, winit_window);
        }
    }
}

/// System to update window when persistent state is modified programmatically.
pub fn on_persistent_window_state_changed(
    mut persistent_windows: Query<
        (&mut Window, &mut Persistent<WindowState>),
        Changed<Persistent<WindowState>>,
    >,
) {
    for (mut window, mut state) in persistent_windows.iter_mut() {
        if !state.sync {
            state.sync = true;
            continue;
        }

        state.persist().ok();
        utils::apply_state_to_window(window.deref_mut(), state.deref_mut(), None);
    }
}
