//! Utilities.

use crate::prelude::*;

/// Gets available monitors from the event loop.
///
/// Result is a mapping from monitor name to monitor index and monitor handle.
///
/// Returns [None] if no monitors are available.
pub fn available_monitors(
    event_loop: &EventLoop<RequestRedraw>,
) -> Option<HashMap<String, (usize, MonitorHandle)>> {
    let available_monitors = event_loop
        .available_monitors()
        .enumerate()
        .filter_map(|(index, monitor)| monitor.name().map(|name| (name, (index, monitor))))
        .collect::<HashMap<String, (usize, MonitorHandle)>>();

    if !available_monitors.is_empty() { Some(available_monitors) } else { None }
}

/// Gets the best monitor from available monitors.
///
/// Best monitor is selected according to:
/// - Width `(Bigger is Better)`
/// - Height `(Bigger is Better)`
/// - Refresh rate `(Bigger is Better)`
/// - Monitor index `(Smaller is Better)`
/// in that order.
///
/// If refresh rate information is not available, monitor is not preferred.
///
/// Result is a tuple of `monitor name` and a tuple of `monitor index` and `monitor handle`.
///
/// # Panics
///
/// - Panics if `available_monitors` is empty.
pub fn best_monitor(
    available_monitors: &HashMap<String, (usize, MonitorHandle)>,
) -> (&String, &(usize, MonitorHandle)) {
    let mut available_monitors = available_monitors.iter();

    let mut selected_monitor = available_monitors.next().unwrap();
    for current_monitor in available_monitors {
        let selected_monitor_handle = &selected_monitor.1.1;
        let current_monitor_handle = &current_monitor.1.1;

        let selected_monitor_size = selected_monitor_handle.size();
        let current_monitor_size = current_monitor_handle.size();

        if current_monitor_size.width < selected_monitor_size.width {
            continue;
        }
        if current_monitor_size.height < selected_monitor_size.height {
            continue;
        }

        let selected_monitor_refresh_rate = selected_monitor_handle.refresh_rate_millihertz();
        let current_monitor_refresh_rate = current_monitor_handle.refresh_rate_millihertz();

        match (current_monitor_refresh_rate, selected_monitor_refresh_rate) {
            (Some(_), None) => {},
            (None, Some(_)) => continue,

            (Some(current_monitor_refresh_rate), Some(selected_monitor_refresh_rate))
                if current_monitor_refresh_rate != selected_monitor_refresh_rate =>
            {
                if current_monitor_refresh_rate < selected_monitor_refresh_rate {
                    continue;
                }
            },

            _ => {
                let selected_monitor_index = &selected_monitor.1.0;
                let current_monitor_index = &current_monitor.1.0;

                if current_monitor_index > selected_monitor_index {
                    continue;
                }
            },
        }

        selected_monitor = current_monitor;
    }
    selected_monitor
}

/// Adjusts a persistent window for a monitor.
///
/// If state already has an existing monitor, it's used.
/// Otherwise, the best monitor is used.
///
/// Changes made by this function persist immediately.
pub fn adjust_to_monitor(
    available_monitors: &HashMap<String, (usize, MonitorHandle)>,
    best_monitor: (&String, &(usize, MonitorHandle)),
    window: &mut Window,
    state: &mut Persistent<WindowState>,
) {
    let (name, (index, handle)) = state
        .monitor
        .as_ref()
        .and_then(|name| available_monitors.get(name).map(|monitor| (name, monitor)))
        .unwrap_or(best_monitor);

    let new_state = WindowState {
        monitor: Some(name.to_string()),
        mode: state.mode,
        resolution: Some(state.resolution.unwrap_or_else(|| {
            let best_video_mode = bevy::winit::get_best_videomode(handle);
            let best_resolution = best_video_mode.size();
            (best_resolution.width, best_resolution.height)
        })),
        scale: Some(state.scale.unwrap_or_else(|| handle.scale_factor())),
        position: state.position,
        sync: state.sync,
    };

    if &new_state != state.get() {
        state.set(new_state).ok();
    }

    apply_state_to_window(window, state, Some(*index))
}

/// Propagates the changes made to the state to the window.
pub fn apply_state_to_window(
    window: &mut Window,
    state: &Persistent<WindowState>,
    monitor_index: Option<usize>,
) {
    let mode = state.mode;
    let mut resolution = WindowResolution::new(
        state.resolution.unwrap().0 as f32,
        state.resolution.unwrap().1 as f32,
    );
    if let Some(scale) = state.scale {
        resolution = resolution.with_scale_factor_override(scale as f32);
    }
    let position = if let Some(position) = state.position {
        WindowPosition::new(position.into())
    } else {
        monitor_index
            .map(|monitor_index| WindowPosition::Centered(MonitorSelection::Index(monitor_index)))
            .unwrap_or(WindowPosition::Automatic)
    };

    window.mode = mode;
    window.resolution = resolution;
    window.position = position;
}

/// Propagates the changes made to the window to the state.
pub fn apply_window_to_state(
    window: &Window,
    state: &mut Persistent<WindowState>,
    winit_window: &winit::window::Window,
) {
    let monitor = winit_window.current_monitor().and_then(|monitor| monitor.name());
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

    let new_state = WindowState { monitor, mode, resolution, scale, position, sync: state.sync };
    if new_state != *state.get() {
        state.set(new_state).ok();
        state.sync = false;
    }
}
