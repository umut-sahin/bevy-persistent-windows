//! Preludes.

pub(crate) use crate::{
    systems::*,
    utils,
};
pub(crate) use bevy::{
    log,
    prelude::*,
    utils::HashMap,
    window::{
        WindowMode,
        WindowResized,
        WindowResolution,
        WindowScaleFactorChanged,
    },
    winit::WinitWindows,
};
pub(crate) use bevy_persistent::prelude::*;
pub(crate) use serde::{
    Deserialize,
    Serialize,
};
pub(crate) use std::ops::DerefMut;
pub(crate) use winit::{
    event_loop::EventLoop,
    monitor::MonitorHandle,
};

pub use crate::{
    PersistentWindowBundle,
    PersistentWindowsPlugin,
    WindowState,
};
