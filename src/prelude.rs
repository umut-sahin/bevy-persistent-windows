//! Preludes.

pub(crate) use crate::{
    systems::*,
    utils,
};
pub(crate) use bevy::{
    ecs::system::NonSendMarker,
    prelude::*,
    window::{
        WindowMode,
        WindowResized,
        WindowResolution,
        WindowScaleFactorChanged,
    },
    winit::WINIT_WINDOWS,
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
