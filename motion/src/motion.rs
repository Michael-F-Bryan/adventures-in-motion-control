use crate::{GcodeProgram, Home, StartHomingSequence};
use aimc_hal::{
    automation::{AutomationSequence, Transition},
    axes::{Axes, Limits},
    messaging::{Ack, Handler, Nack},
    System,
};
use uom::si::{f32::Velocity, velocity::millimeter_per_second};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Motion {
    pub motion_params: MotionParameters,
    pub control_mode: ControlMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlMode {
    Idle,
    Home(Home),
}

impl<L: Limits, A: Axes> System<L, A> for Motion {
    fn poll(&mut self, inputs: &L, outputs: &mut A) {
        match self.control_mode {
            ControlMode::Idle => {},
            ControlMode::Home(ref mut home) => match home.poll(inputs, outputs)
            {
                Transition::Complete | Transition::Fault(_) => {
                    self.control_mode = ControlMode::Idle
                },
                _ => {},
            },
        }
    }
}

impl Default for ControlMode {
    fn default() -> ControlMode { ControlMode::Idle }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MotionParameters {
    pub x_axis: usize,
    pub y_axis: usize,
    pub z_axis: usize,
    pub homing_speed: Velocity,
}

impl MotionParameters {
    pub fn homing_sequence(&self) -> Home {
        Home::new(self.x_axis, self.y_axis, self.z_axis, self.homing_speed)
    }
}

impl Default for MotionParameters {
    fn default() -> MotionParameters {
        MotionParameters {
            x_axis: 0,
            y_axis: 1,
            z_axis: 2,
            homing_speed: Velocity::new::<millimeter_per_second>(10.0),
        }
    }
}

impl Handler<StartHomingSequence> for Motion {
    type Response = Result<Ack, Nack>;

    fn handle(&mut self, _: StartHomingSequence) -> Self::Response {
        match self.control_mode {
            ControlMode::Idle => {
                self.control_mode =
                    ControlMode::Home(self.motion_params.homing_sequence());
                Ok(Ack::default())
            },
            _ => Err(Nack::default()),
        }
    }
}

impl Handler<GcodeProgram<'_>> for Motion {
    type Response = Result<Ack, Nack>;

    fn handle(&mut self, gcode: GcodeProgram<'_>) -> Self::Response {
        unimplemented!("Received a {:?}", gcode);
    }
}
