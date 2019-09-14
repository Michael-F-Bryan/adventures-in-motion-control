use aimc_hal::{
    automation::{AutomationSequence, Transition},
    axes::{Axes, Limits},
};
use uom::si::f32::Velocity;

#[derive(Debug, Clone, PartialEq)]
pub struct MoveAxisHome {
    homing_speed: Velocity,
    axis_number: usize,
}

impl MoveAxisHome {
    pub fn new(homing_speed: Velocity, axis_number: usize) -> Self {
        MoveAxisHome {
            homing_speed: homing_speed.abs(),
            axis_number,
        }
    }
}

impl<L: Limits, A: Axes> AutomationSequence<L, A> for MoveAxisHome {
    fn poll(&mut self, inputs: &L, outputs: &mut A) -> Transition {
        unimplemented!()
    }
}
