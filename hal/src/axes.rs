use uom::si::f32::Velocity;

/// A driver for controlling axis motion using *velocity control*.
pub trait Axes {
    /// Tell the specified axis to move at a desired velocity.
    fn set_target_velocity(&mut self, axis_number: usize, velocity: Velocity);

    /// Get the actual velocity a particular axis is moving at.
    fn velocity(&self, axis_number: usize) -> Option<Velocity>;
}

/// A driver which tracks the limit switch state.
pub trait Limits {
    fn limit_switches(&self, axis_number: usize) -> Option<LimitSwitchState>;
}

/// The state of a set of limit switches.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct LimitSwitchState {
    pub at_lower_limit: bool,
    pub at_upper_limit: bool,
}
