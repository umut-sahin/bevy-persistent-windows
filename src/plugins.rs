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
///   before [WinitPlugin](bevy::winit::WinitPlugin),
///   which is in the [DefaultPlugins].
pub struct PersistentWindowsPlugin;

impl Plugin for PersistentWindowsPlugin {
    fn build(&self, app: &mut App) {
        let mut persistent_windows =
            app.world_mut().query::<(&mut Window, &Persistent<WindowState>)>();

        for (mut window, state) in persistent_windows.iter_mut(app.world_mut()) {
            utils::apply_state_to_window(&mut window, state);
        }

        app.add_systems(Startup, auto_scale);
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
