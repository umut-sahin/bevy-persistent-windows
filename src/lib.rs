#![cfg_attr(doctest, doc = "````no_test")]
#![doc = include_str!("../README.md")]

mod systems;
mod utils;

pub mod bundles;
pub mod components;
pub mod plugins;
pub mod prelude;

pub use crate::{
    bundles::PersistentWindowBundle,
    components::WindowState,
    plugins::PersistentWindowsPlugin,
};
