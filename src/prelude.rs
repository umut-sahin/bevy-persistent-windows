//! Preludes.

pub(crate) use crate::{
    systems::*,
    utils,
};
pub(crate) use bevy::{
    prelude::*,
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

pub use crate::{
    PersistentWindowBundle,
    PersistentWindowsPlugin,
    WindowState,
};
