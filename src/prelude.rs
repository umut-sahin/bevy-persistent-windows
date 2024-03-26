//! Preludes.

pub(crate) use crate::{
    systems::*,
    utils,
};
pub(crate) use bevy::{
    ecs::system::SystemState,
    log,
    prelude::*,
    utils::HashMap,
    window::{
        RequestRedraw,
        WindowMode,
        WindowResized,
        WindowResolution,
        WindowScaleFactorChanged,
    },
    winit::{
        create_windows,
        CreateWindowParams,
        WinitWindows,
    },
};
pub(crate) use bevy_persistent::prelude::*;
pub(crate) use serde::{
    Deserialize,
    Serialize,
};
pub(crate) use std::ops::{
    Deref,
    DerefMut,
};
pub(crate) use winit::{
    event_loop::EventLoop,
    monitor::MonitorHandle,
};

pub use crate::{
    PersistentWindowBundle,
    PersistentWindowsPlugin,
    WindowState,
};
