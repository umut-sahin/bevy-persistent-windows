use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_persistent::prelude::*;
use bevy_persistent_windows::prelude::*;
use std::path::{
    Path,
    PathBuf,
};

#[derive(Resource)]
struct Directories {
    state_directory: PathBuf,
}

fn main() {
    let mut app = App::new();

    let window_plugin = WindowPlugin { primary_window: None, ..Default::default() };
    app.add_plugins(DefaultPlugins.set(window_plugin).build());

    let state_directory = dirs::data_dir()
        .map(|platform_data_dir| platform_data_dir.join("bevy-persistent-windows").join("state"))
        .unwrap_or(Path::new("session").join("data").join("state"))
        .join("primary");

    app.world.spawn((
        PrimaryWindow,
        PersistentWindowBundle {
            window: Window { title: "I am primary...".to_owned(), ..Default::default() },
            state: Persistent::<WindowState>::builder()
                .name("primary window state")
                .format(StorageFormat::Toml)
                .path(state_directory.join("primary-window.toml"))
                .default(WindowState::windowed(1280, 720))
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .expect("failed to create persistent primary window state"),
        },
    ));

    app.add_plugins(PersistentWindowsPlugin);

    app.insert_resource(FixedTime::new_from_secs(3.0));
    app.add_systems(FixedUpdate, spawn_persistent_window);

    app.insert_resource(Directories { state_directory });

    app.run();
}

fn spawn_persistent_window(
    mut commands: Commands,
    directories: Res<Directories>,
    persistent_windows: Query<(), (With<Window>, With<Persistent<WindowState>>)>,
) {
    let persistent_window_count = persistent_windows.iter().len();
    commands.spawn(PersistentWindowBundle {
        window: Window {
            title: format!("I am #{}", persistent_window_count),
            ..Default::default()
        },
        state: Persistent::<WindowState>::builder()
            .name(format!("window #{} state", persistent_window_count))
            .format(StorageFormat::Toml)
            .path(
                directories
                    .state_directory
                    .join(format!("window-{}.toml", persistent_window_count)),
            )
            .default(WindowState::windowed(400, 400))
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
            .unwrap_or_else(|error| {
                panic!(
                    "failed to create the persistent window #{} state: {}",
                    persistent_window_count, error
                )
            }),
    });
}
