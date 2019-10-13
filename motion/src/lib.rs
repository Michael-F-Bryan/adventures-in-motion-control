#![no_std]

#[cfg(test)]
extern crate std;

mod home;
mod motion;

pub use crate::{
    home::{Fault, FaultKind, Home, MoveAxisHome, StartHomingSequence},
    motion::{ControlMode, Motion, MotionParameters},
};
