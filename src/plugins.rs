//! Plugins.

use crate::prelude::*;

/// Persistent windows plugin.
///
/// Responsible for restoring window states before the application is run
/// and synchronizing windows with their states during the run.
///
/// # Panics
///
/// - Panics if it's added to the [App]
/// before [WinitPlugin](bevy::winit::WinitPlugin),
/// which is in the [DefaultPlugins].
pub struct PersistentWindowsPlugin;

impl Plugin for PersistentWindowsPlugin {
    fn build(&self, app: &mut App) {
        let event_loop = app
            .world
            .get_non_send_resource::<EventLoop<()>>()
            .expect("persistent windows plugin is added before winit plugin");

        match utils::available_monitors(event_loop) {
            Some(available_monitors) => {
                let best_monitor = utils::best_monitor(&available_monitors);

                let mut persistent_windows =
                    app.world.query::<(&mut Window, &mut Persistent<WindowState>)>();

                for (mut window, mut state) in persistent_windows.iter_mut(&mut app.world) {
                    utils::adjust_to_monitor(
                        &available_monitors,
                        best_monitor,
                        &mut window,
                        &mut state,
                    );
                }
            },
            None => {
                log::error!("unable to persist windows as no monitors are available");
            },
        }

        app.add_systems(
            PreUpdate,
            (
                on_persistent_window_moved,
                on_persistent_window_resized,
                on_persistent_window_scale_factor_changed,
            ),
        );
        app.add_systems(PostUpdate, on_persistent_window_state_changed);
    }
}
