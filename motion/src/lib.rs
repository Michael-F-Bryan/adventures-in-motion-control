#![no_std]

#[cfg(test)]
extern crate std;

use aimc_hal::{
    automation::{All, AutomationSequence, Transition},
    axes::{Axes, Limits},
};
use arrayvec::ArrayVec;
use uom::si::f32::Velocity;

#[derive(Debug, Clone, PartialEq)]
pub struct MoveAxisHome {
    homing_speed: Velocity,
    axis_number: usize,
}

impl MoveAxisHome {
    pub const AXIS_NOT_FOUND: u16 = 2;
    /// The upper limit was reached when we were expecting to hit the lower one.
    pub const UNEXPECTED_UPPER_LIMIT: u16 = 1;

    pub fn new(homing_speed: Velocity, axis_number: usize) -> Self {
        MoveAxisHome {
            homing_speed: homing_speed.abs(),
            axis_number,
        }
    }
}

impl<L: Limits, A: Axes> AutomationSequence<L, A> for MoveAxisHome {
    type FaultInfo = Fault;

    fn poll(&mut self, inputs: &L, outputs: &mut A) -> Transition<Self::FaultInfo> {
        let limits = match inputs.limit_switches(self.axis_number) {
            Some(l) => l,
            None => return Transition::Fault(Fault::axis_not_found(self.axis_number)),
        };

        if limits.at_upper_limit {
            outputs.set_target_velocity(self.axis_number, Velocity::default());
            Transition::Fault(Fault::unexpected_upper_limit(self.axis_number))
        } else if limits.at_lower_limit {
            outputs.set_target_velocity(self.axis_number, Velocity::default());
            Transition::Complete
        } else {
            outputs.set_target_velocity(self.axis_number, -1.0 * self.homing_speed);
            Transition::Incomplete
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Fault {
    pub axis_number: usize,
    pub kind: FaultKind,
}

impl Fault {
    pub const fn axis_not_found(axis_number: usize) -> Self {
        Fault {
            axis_number,
            kind: FaultKind::AxisNotFound,
        }
    }

    pub const fn unexpected_upper_limit(axis_number: usize) -> Self {
        Fault {
            axis_number,
            kind: FaultKind::UnexpectedUpperLimit,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FaultKind {
    /// It doesn't look like the machine has this axis.
    AxisNotFound,
    /// The upper limit was reached when we were expecting to hit the lower
    /// one.
    UnexpectedUpperLimit,
}

/// The *Go To Home* [`AutomationSequence`].
#[derive(Debug, Clone, PartialEq)]
pub struct Home {
    inner: All<ArrayVec<[Option<MoveAxisHome>; 3]>>,
}

impl Home {
    pub fn new(x_axis: usize, y_axis: usize, z_axis: usize, homing_speed: Velocity) -> Self {
        Home {
            inner: All::new(ArrayVec::from([
                Some(MoveAxisHome::new(homing_speed, x_axis)),
                Some(MoveAxisHome::new(homing_speed, y_axis)),
                Some(MoveAxisHome::new(homing_speed, z_axis)),
            ])),
        }
    }
}

impl<L: Limits, A: Axes> AutomationSequence<L, A> for Home {
    type FaultInfo = Fault;

    fn poll(&mut self, inputs: &L, outputs: &mut A) -> Transition<Self::FaultInfo> {
        self.inner.poll(inputs, outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aimc_hal::axes::LimitSwitchState;
    use std::collections::HashMap;
    use uom::si::velocity::meter_per_second;

    #[derive(Debug, Default)]
    struct DummyLimits(HashMap<usize, LimitSwitchState>);

    impl Limits for DummyLimits {
        fn limit_switches(&self, axis_number: usize) -> Option<LimitSwitchState> {
            self.0.get(&axis_number).copied()
        }
    }

    #[derive(Debug, Default)]
    struct DummyAxes(HashMap<usize, Velocity>);

    impl Axes for DummyAxes {
        fn set_target_velocity(&mut self, axis_number: usize, velocity: Velocity) {
            self.0.insert(axis_number, velocity);
        }

        fn velocity(&self, axis_number: usize) -> Option<Velocity> {
            self.0.get(&axis_number).copied()
        }
    }

    #[test]
    fn polling_without_hitting_limits_makes_an_axis_move_backwards() {
        let mut seq = MoveAxisHome::new(Velocity::new::<meter_per_second>(1.0), 7);
        let mut axes = DummyAxes::default();
        let mut limits = DummyLimits::default();
        limits.0.insert(7, LimitSwitchState::default());

        let trans = seq.poll(&limits, &mut axes);

        assert_eq!(trans, Transition::Incomplete);
        assert_eq!(axes.0.len(), 1);
        assert_eq!(axes.velocity(7), Some(-1.0 * seq.homing_speed));
    }

    #[test]
    fn actuating_the_upper_limit_is_a_fault() {
        let mut seq = MoveAxisHome::new(Velocity::new::<meter_per_second>(1.0), 7);
        let mut axes = DummyAxes::default();
        let mut limits = DummyLimits::default();
        limits.0.insert(
            7,
            LimitSwitchState {
                at_lower_limit: false,
                at_upper_limit: true,
            },
        );

        let trans = seq.poll(&limits, &mut axes);

        assert_eq!(trans, Transition::Fault(Fault::unexpected_upper_limit(7)));
        assert_eq!(axes.velocity(7), Some(Velocity::default()));
    }

    #[test]
    fn actuating_the_lower_limit_completes_the_sequence() {
        let mut seq = MoveAxisHome::new(Velocity::new::<meter_per_second>(1.0), 7);
        let mut axes = DummyAxes::default();
        let mut limits = DummyLimits::default();
        limits.0.insert(
            7,
            LimitSwitchState {
                at_lower_limit: true,
                at_upper_limit: false,
            },
        );

        let trans = seq.poll(&limits, &mut axes);

        assert_eq!(trans, Transition::Complete);
        assert_eq!(axes.velocity(7), Some(Velocity::default()));
    }

    #[test]
    fn trying_to_home_a_nonexistent_axis_is_a_fault() {
        let mut seq = MoveAxisHome::new(Velocity::new::<meter_per_second>(1.0), 7);
        let mut axes = DummyAxes::default();
        let limits = DummyLimits::default();
        assert!(limits.limit_switches(seq.axis_number).is_none());

        let trans = seq.poll(&limits, &mut axes);

        assert_eq!(trans, Transition::Fault(Fault::axis_not_found(7)));
    }
}
