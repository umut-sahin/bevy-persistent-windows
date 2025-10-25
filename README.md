# bevy-persistent-windows

A [Bevy](https://bevyengine.org/) plugin to easily create and manage windows that remember where they were.

## Background

When you're developing a game, thus frequently restarting it, you may (understandably) desire that the windows just stay where they were in the last run. Implementing this manually in every project you create is error-prone and time-consuming (trust me, I know). This plugin aims to make it as seamless as possible!

## Warning

This plugin doesn't work in Wayland with `wayland` feature. The problem is that Wayland doesn’t expose the windows position to client applications, so `winit` doesn't emit any `WindowMoved` events. Feel free to disable `wayland` feature it if it's required.

## Installation

We'll be using [bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) to store window states persistently, so let's add it first:

```shell
cargo add bevy-persistent --features all  # you probably don't need all features, see installation section of bevy-persistent to learn more
```

Now let's add [bevy-persistent-windows](https://github.com/umut-sahin/bevy-persistent-windows/) to make our windows persistent:

```shell
cargo add bevy-persistent-windows
```

## Usage

### Prelude

As mentioned before, we'll be using [bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) to store the window state persistently, so lets prelude it first:

```rust
use bevy_persistent::prelude::*;
```

We need [WindowState](https://docs.rs/bevy-persistent-windows/latest/bevy_persistent_windows/components/struct.WindowState.html), [PersistentWindowBundle](https://docs.rs/bevy-persistent-windows/latest/bevy_persistent_windows/bundles/struct.PersistentWindowBundle.html) and [PersistentWindowsPlugin](https://docs.rs/bevy-persistent-windows/latest/bevy_persistent_windows/plugins/struct.PersistentWindowsPlugin.html) types to use the library, and they are exported from the prelude module:

```rust
use bevy_persistent_windows::prelude::*;
```

### Setup

Let's start by creating an `App` within `main`:

```rust
let mut app = App::new();
```

We'll add the default plugins to this app, but we should edit the window plugin to avoid creating a default primary window:

```rust
let window_plugin = WindowPlugin { primary_window: None, ..Default::default() };
app.add_plugins(DefaultPlugins.set(window_plugin).build());
```

We need somewhere to store the window state, to restore the window later:

```rust
let state_directory = dirs::data_dir()
    .expect("failed to get the platforms data directory")
    .join("your-amazing-game")
    .join("state");
```

Time to create the primary window:

```rust
app.world_mut().spawn((
    PrimaryWindow,
    PersistentWindowBundle {
        window: Window { title: "I am the primary window!".to_owned(), ..Default::default() },
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
```

Feel free to spawn additional windows, without the `PrimaryWindow` component of course!

Once, you're done, you can add `PersistentWindowsPlugin` plugin to the app:

```rust
app.add_plugins(PersistentWindowsPlugin);
```

And run your game:

```rust
app.run();
```

You'll see a `1280x720` window appear in the center of your best monitor, move it around, resize, and play with it. Now close the application, and run it again. You'll see that the window will open in the exact same monitor, with the exact same resolution, and the exact same position!

See [examples/setup.rs](https://github.com/umut-sahin/bevy-persistent-windows/blob/main/examples/setup.rs) for the full example!

### Controlling

You may wish to control the persistent windows programmatically. You can edit the window itself, but if you want your changes to persist, you should modify the window state directly!

Say you want to toggle fullscreen when space bar is pressed, you can add this system to your app:

```rust
fn fullscreen_toggle(
    keyboard_input: Res<Input<KeyCode>>,
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
```

Or if you want to move the window with arrow keys, and resize it with ctrl + arrow keys, you can use this system:

```rust
fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Persistent<WindowState>, With<PrimaryWindow>>,
) {
    let mut position_change = (0.0, 0.0);
    let mut resolution_change = (0.0, 0.0);

    let change = if keyboard_input.pressed(KeyCode::ControlLeft) {
        &mut resolution_change
    } else {
        &mut position_change
    };

    if keyboard_input.pressed(KeyCode::Up) {
        change.1 -= 3.0 * time.delta().as_millis() as f32;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        change.0 -= 3.0 * time.delta().as_millis() as f32;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        change.1 += 3.0 * time.delta().as_millis() as f32;
    }
    if keyboard_input.pressed(KeyCode::Right) {
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
```

(ps: anyone wants to write a snake clone using persistent windows?)

See [examples/controlling.rs](https://github.com/umut-sahin/bevy-persistent-windows/blob/main/examples/controlling.rs) for the full example!

### Spawning

When you want to spawn additional windows, you can just spawn a new [PersistentWindowBundle](https://docs.rs/bevy-persistent-windows/latest/bevy_persistent_windows/bundles/struct.PersistentWindowBundle.html), just like you did in the setup:

```rust
fn spawn_persistent_window(
    mut commands: Commands,
    persistent_windows: Query<(), (With<Persistent<WindowState>>, With<Window>)>,
) {
    let state_directory = dirs::data_dir()
        .expect("failed to get the platforms data directory")
        .join("your-amazing-game")
        .join("state");

    let persistent_window_count = persistent_windows.iter().len();
    commands.spawn((
        PersistentWindowBundle {
            window: Window {
                title: format!("I am #{}", persistent_window_count),
                ..Default::default()
            },
            state: Persistent::<WindowState>::builder()
                .name(format!("window #{} state", persistent_window_count))
                .format(StorageFormat::Toml)
                .path(state_directory.join(format!("window-{}.toml", persistent_window_count)))
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
        },
    ));
}
```

When this system runs, it'll create a `400x400` persistent window in the center of your monitor.

See [examples/spawning.rs](https://github.com/umut-sahin/bevy-persistent-windows/blob/main/examples/spawning.rs) for the full example!

## Examples

You can run all the mentioned examples using:

```shell
cargo run --release --example name-of-the-example
```

## Limitations

- If you're a psychopath who like to put your window half in one monitor and half in another, I have bad news. Bevy clips the windows to the monitor they're in, so the window state cannot be restored entirely.
- Best monitor cannot be decided for persistent windows that are spawned after application starts running. This is because [WinitPlugin](https://docs.rs/bevy/latest/bevy/winit/struct.WinitPlugin.html) removes the [EventLoop](https://docs.rs/winit/latest/winit/event_loop/struct.EventLoop.html) from the world before application starts running and without the event loop, I couldn't find a way to get the available monitors.

Please let me know if you know ways to get around these limitations!

## License

[bevy-persistent-windows](https://github.com/umut-sahin/bevy-persistent-windows/) is free, open source and permissively licensed, just like [Bevy](https://bevyengine.org/)!

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](https://github.com/umut-sahin/bevy-persistent-windows/blob/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE]((https://github.com/umut-sahin/bevy-persistent-windows/blob/main/LICENSE-APACHE)) or <https://www.apache.org/licenses/LICENSE-2.0>)

This means you can select the license you prefer!
