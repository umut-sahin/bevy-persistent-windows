//! Bundles.

use crate::prelude::*;

/// Persistent window bundle.
///
/// Made of `a window` and `a persistent state` for it.
///
/// Changes made to `window` or `state` components
/// will be synchronized by [PersistentWindowsPlugin].
#[derive(Bundle)]
pub struct PersistentWindowBundle {
    /// Window component.
    pub window: Window,
    /// Persistent window state component.
    pub state: Persistent<WindowState>,
}
