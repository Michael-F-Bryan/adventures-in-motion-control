#![no_std]

pub mod clock;
pub mod messaging;
mod system;

pub use crate::{
    messaging::Handler,
    system::{Ignored, System},
};
