use core::marker::PhantomData;

/// An automation sequence which will either be polled to completion or abort
/// early with a fault.
pub trait AutomationSequence<Input, Output> {
    /// Extra info attached to a fault.
    type FaultInfo;

    fn poll(
        &mut self,
        inputs: &Input,
        outputs: &mut Output,
    ) -> Transition<Self::FaultInfo>;
}

/// The result of a single call to [`AutomationSequence::poll()`].
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Transition<F> {
    /// The [`AutomationSequence`] completed successfully.
    Complete,
    /// The [`AutomationSequence`] failed with a particular fault code.
    Fault(F),
    /// The [`AutomationSequence`] is still running.
    Incomplete,
}

impl<F> Transition<F> {
    pub fn at_end_state(&self) -> bool {
        match self {
            Transition::Complete | Transition::Fault(..) => true,
            Transition::Incomplete => false,
        }
    }
}

/// A combinator which combines many [`AutomationSequence`]s and will poll them
/// all to completion, stopping when either a fault is raised or there are no
/// more incomplete sequences.
///
/// # Examples
///
/// ```rust
/// use aimc_hal::automation::{AutomationSequence, Transition, All};
///
/// /// A simple automation sequence which will return `Transition::Incomplete`
/// /// until it reaches zero.
/// struct CountDown(usize);
///
/// impl<I, O> AutomationSequence<I, O> for CountDown {
///     type FaultInfo = ();
///
///     fn poll(&mut self, inputs: &I, outputs: &mut O) -> Transition<()> {
///         if self.0 == 0 {
///             Transition::Complete
///         } else {
///             self.0 -= 1;
///             Transition::Incomplete
///         }
///     }
/// }
///
/// // make the list of sequences to combine. This needs to be a
/// // `[Option<CountDown>]` instead of `[CountDown]` for technical reasons.
/// let items = [Some(CountDown(1)), Some(CountDown(5)), Some(CountDown(2))];
/// // then combine them
/// let mut seq = All::new(items);
///
/// // we'll keep track of the number of polls
/// let mut polls = 0;
///
/// // keep polling until all the timers have reached zero
/// while seq.poll(&(), &mut ()) != Transition::Complete {
///     polls += 1;
/// }
///
/// // we should have polled 5 times (`max(1, 5, 2)`)
/// assert_eq!(polls, 5);
/// ```
///
/// # Technical Reasons/Excuses
///
/// Sorry for the horrible type signature! Once [const generics][const] come
/// along it can be changed to just be generic over the [`AutomationSequence`]
/// type and number of items being combined, as one would expect (i.e. `seq` in
/// the example would have been `All<CountDown, const 3>` instead of
/// `All<CountDown, [Option<CountDown>; 3], (), ()>`).
///
/// In general `impl Trait` won't help here, because we'll need to store an
/// instance of [`All`] in a struct, and that requires either making that struct
/// generic (leaky abstraction) or being able to name the type.
///
/// [const]: https://github.com/rust-lang/rust/issues/44580
#[derive(Debug, Clone, PartialEq)]
pub struct All<A, V, I, O> {
    sequences: V,
    _automation_type: PhantomData<A>,
    _input_type: PhantomData<I>,
    _output_type: PhantomData<O>,
}

impl<A, V, I, O> All<A, V, I, O>
where
    V: AsMut<[Option<A>]>,
    A: AutomationSequence<I, O>,
{
    pub fn new(items: V) -> Self {
        All {
            sequences: items,
            _automation_type: PhantomData,
            _input_type: PhantomData,
            _output_type: PhantomData,
        }
    }
}

impl<I, O, A: AutomationSequence<I, O>, V: AsMut<[Option<A>]>>
    AutomationSequence<I, O> for All<A, V, I, O>
{
    type FaultInfo = A::FaultInfo;

    fn poll(
        &mut self,
        inputs: &I,
        outputs: &mut O,
    ) -> Transition<Self::FaultInfo> {
        let variants = self.sequences.as_mut();

        for variant in variants.iter_mut() {
            if let Transition::Fault(f) = poll_variant(variant, inputs, outputs)
            {
                return Transition::Fault(f);
            }
        }

        if variants.iter().all(|v| v.is_none()) {
            Transition::Complete
        } else {
            Transition::Incomplete
        }
    }
}

fn poll_variant<I, O, A>(
    variant: &mut Option<A>,
    inputs: &I,
    outputs: &mut O,
) -> Transition<A::FaultInfo>
where
    A: AutomationSequence<I, O>,
{
    let trans = match variant {
        Some(ref mut sequence) => sequence.poll(inputs, outputs),
        None => Transition::Complete,
    };

    if trans.at_end_state() {
        let _ = variant.take();
    }

    trans
}
