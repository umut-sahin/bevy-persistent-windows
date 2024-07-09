use bevy::{
    prelude::*,
    window::PrimaryWindow,
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
        .join("setup");

    app.world_mut().spawn((
        PrimaryWindow,
        PersistentWindowBundle {
            window: Window { title: "I persist!".to_owned(), ..Default::default() },
            state: Persistent::<WindowState>::builder()
                .name("primary window state")
                .format(StorageFormat::Toml)
                .path(state_directory.join("primary-window.toml"))
                .default(WindowState::windowed(1280, 720))
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .expect("failed to create the persistent primary window state"),
        },
    ));

    app.add_plugins(PersistentWindowsPlugin);

    app.run();
}
