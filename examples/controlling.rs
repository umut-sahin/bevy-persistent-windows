use bevy::{
    prelude::*,
    window::{
        PrimaryWindow,
        WindowMode,
    },
};
use bevy_persistent::prelude::*;
use bevy_persistent_windows::prelude::*;
use std::path::Path;

fn main() {
    let mut app = App::new();

    let window_plugin = WindowPlugin { primary_window: None, ..Default::default() };
    app.add_plugins(DefaultPlugins.set(window_plugin).build());

    let state_directory = dirs::data_dir()
        .map(|platform_data_dir| platform_data_dir.join("bevy-persistent-windows").join("state"))
        .unwrap_or(Path::new("session").join("data").join("state"))
        .join("primary");

    app.world_mut().spawn((
        PrimaryWindow,
        PersistentWindowBundle {
            window: Window {
                title: "I can be moved persistently.".to_owned(),
                ..Default::default()
            },
            state: Persistent::<WindowState>::builder()
                .name("primary window state")
                .format(StorageFormat::Toml)
                .path(state_directory.join("primary-window.toml"))
                .default(WindowState::fullscreen())
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .expect("failed to create the persistent primary window state"),
        },
    ));

    app.add_plugins(PersistentWindowsPlugin);

    app.add_systems(Update, (fullscreen_toggle, movement));

    app.run();
}

fn fullscreen_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Persistent<WindowState>, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut primary_window_state = query.get_single_mut().unwrap();

        if primary_window_state.mode == WindowMode::Windowed {
            primary_window_state.mode = WindowMode::Fullscreen;
        } else {
            primary_window_state.mode = WindowMode::Windowed;
        }

        primary_window_state.persist().ok();
    }
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Persistent<WindowState>, With<PrimaryWindow>>,
) {
    let mut position_change = (0.0, 0.0);
    let mut resolution_change = (0.0, 0.0);

    let change = if keyboard_input.pressed(KeyCode::ControlLeft) {
        &mut resolution_change
    } else {
        &mut position_change
    };

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        change.1 -= 3.0 * time.delta().as_millis() as f32;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        change.0 -= 3.0 * time.delta().as_millis() as f32;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        change.1 += 3.0 * time.delta().as_millis() as f32;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        change.0 += 3.0 * time.delta().as_millis() as f32;
    }

    if position_change == (0.0, 0.0) && resolution_change == (0.0, 0.0) {
        return;
    }

    let mut primary_window_state = query.get_single_mut().unwrap();
    if let Some(resolution) = &mut primary_window_state.resolution {
        resolution.0 = ((resolution.0 as f32) + (resolution_change.0)) as u32;
        resolution.1 = ((resolution.1 as f32) + (resolution_change.1)) as u32;
    }
    if let Some(position) = &mut primary_window_state.position {
        position.0 += position_change.0 as i32;
        position.1 += position_change.1 as i32;
    }
}
