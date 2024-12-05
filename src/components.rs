//! Components.

use crate::prelude::*;

/// Window state.
#[derive(Clone, Component, Debug, Deserialize, PartialEq, Resource, Serialize)]
#[serde(default)]
pub struct WindowState {
    /// Mode of the window.
    pub mode: WindowMode,

    /// Resolution of the window.
    /// (`None` means pick the best resolution)
    pub resolution: Option<(u32, u32)>,

    /// Position of the window.
    /// (`None` means centered)
    pub position: Option<(i32, i32)>,

    /// Scale of the window.
    /// (`None` means pick the best scale)
    pub scale: Option<f64>,

    /// Whether the window scale should be set automatically at the beginning of the application.
    pub(crate) auto_scaled: bool,

    #[serde(skip)]
    pub(crate) sync: bool,
}

impl WindowState {
    /// Creates a borderless fullscreen state.
    pub fn borderless_fullscreen() -> WindowState {
        WindowState {
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            resolution: None,
            position: None,
            scale: None,
            auto_scaled: true,
            sync: true,
        }
    }

    /// Creates a fullscreen state.
    pub fn fullscreen() -> WindowState {
        WindowState {
            mode: WindowMode::Fullscreen(MonitorSelection::Primary),
            resolution: None,
            position: None,
            scale: None,
            auto_scaled: true,
            sync: true,
        }
    }
    /// Creates a sized fullscreen state.
    pub fn sized_fullscreen() -> WindowState {
        WindowState {
            mode: WindowMode::SizedFullscreen(MonitorSelection::Primary),
            resolution: None,
            position: None,
            scale: None,
            auto_scaled: true,
            sync: true,
        }
    }

    /// Creates a windowed state with given resolution.
    pub fn windowed(width: u32, height: u32) -> WindowState {
        WindowState {
            mode: WindowMode::Windowed,
            resolution: Some((width, height)),
            position: None,
            scale: None,
            auto_scaled: true,
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

    /// Adds scale information to the state.
    pub fn scaled(mut self, scale: f64) -> WindowState {
        self.scale = Some(scale);
        self
    }
}

impl Default for WindowState {
    fn default() -> WindowState {
        WindowState {
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            resolution: None,
            position: None,
            scale: None,
            auto_scaled: false,
            sync: true,
        }
    }
}
