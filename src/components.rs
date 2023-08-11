//! Components.

use crate::prelude::*;

/// Window state.
#[derive(Clone, Component, Debug, Deserialize, Eq, PartialEq, Resource, Serialize)]
#[serde(default)]
pub struct WindowState {
    /// Mode of the window.
    pub mode: WindowMode,

    /// Name of the monitor that the window is in.
    /// (`None` means pick the best monitor)
    pub monitor: Option<String>,

    /// Resolution of the window.
    /// (`None` means pick the best resolution)
    pub resolution: Option<(u32, u32)>,

    /// Position of the window.
    /// (`None` means centered)
    pub position: Option<(i32, i32)>,

    #[serde(skip)]
    pub(crate) sync: bool,
}

impl WindowState {
    /// Creates a fullscreen state.
    pub fn fullscreen() -> WindowState {
        WindowState {
            mode: WindowMode::Fullscreen,
            monitor: None,
            resolution: None,
            position: None,
            sync: true,
        }
    }

    /// Creates a windowed state with given resolution.
    pub fn windowed(width: u32, height: u32) -> WindowState {
        WindowState {
            mode: WindowMode::Windowed,
            monitor: None,
            resolution: Some((width, height)),
            position: None,
            sync: true,
        }
    }
}

impl WindowState {
    /// Adds position information to the state.
    pub fn at(mut self, x: i32, y: i32) -> WindowState {
        if self.mode == WindowMode::Windowed {
            self.position = Some((x, y));
        }
        self
    }
}

impl Default for WindowState {
    fn default() -> WindowState {
        WindowState::fullscreen()
    }
}
