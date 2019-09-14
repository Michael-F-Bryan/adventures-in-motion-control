/// An automation sequence which will either be polled to completion or abort
/// early with a fault.
pub trait AutomationSequence<Input, Output> {
    fn poll(&mut self, inputs: &Input, outputs: &mut Output) -> Transition;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Transition {
    /// The [`AutomationSequence`] completed successfully.
    Complete,
    /// The [`AutomationSequence`] failed with a particular fault code.
    Fault { code: u16 },
    /// The [`AutomationSequence`] is still running.
    Incomplete,
}
