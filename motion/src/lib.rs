#![no_std]
#![feature(const_generics, const_generic_impls_guard)]

#[cfg(test)]
#[macro_use]
extern crate std;

mod gcode_program;
mod home;
mod motion;
pub mod movements;
pub mod planning;

pub use crate::{
    gcode_program::GcodeProgram,
    home::{Fault, FaultKind, Home, MoveAxisHome, StartHomingSequence},
    motion::{ControlMode, Motion, MotionParameters},
};
