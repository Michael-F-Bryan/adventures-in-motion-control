use core::{
    fmt::{self, Debug, Formatter},
    mem::{self, MaybeUninit},
    ptr,
};

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
/// // Combine the sequences into one big automation sequence
/// let mut seq = All::new([CountDown(1), CountDown(5), CountDown(2)]);
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
/// [const]: https://github.com/rust-lang/rust/issues/44580
pub struct All<A, const N: usize> {
    sequences: [Option<A>; N],
}

impl<A, const N: usize> All<A, { N }> {
    pub fn new(items: [A; N]) -> Self {
        All {
            sequences: wrap_in_option(items),
        }
    }
}

/// Transform a `[T; N]` into a `[Option<T>; N]`, this is essentially a poor
/// man's `items.into_iter().map(Some).collect()` using static arrays.
fn wrap_in_option<T, const N: usize>(items: [T; N]) -> [Option<T>; N] {
    unsafe {
        let mut sequences =
            MaybeUninit::<[MaybeUninit<Option<T>>; N]>::uninit();

        for i in 0..N {
            // get a pointer to the item we want to copy
            let item = items.as_ptr().add(i);

            // an array of MaybeUninit is always valid
            let sequences = &mut *sequences.as_mut_ptr();

            // copy the item across, transferring ownership to sequences
            sequences
                .as_mut_ptr()
                .add(i)
                .write(MaybeUninit::new(Some(ptr::read(item))));
        }

        // The original variable no longer has ownership
        mem::forget(items);

        mem::transmute_copy(&sequences)
    }
}

impl<I, O, A, const N: usize> AutomationSequence<I, O> for All<A, { N }>
where
    A: AutomationSequence<I, O>,
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

// prefer to manually implement these instead of using #[derive] so it doesn't
// pollute the `All` type signature with implementation details. Should be
// resolved once `#[feature(const_generic_impls_guard)]` is removed.

impl<A, const N: usize> Default for All<A, { N }>
where
    [Option<A>; N]: core::array::LengthAtMost32,
    [Option<A>; N]: Default,
{
    fn default() -> All<A, { N }> {
        All {
            sequences: Default::default(),
        }
    }
}

impl<A, const N: usize> Copy for All<A, { N }>
where
    [Option<A>; N]: core::array::LengthAtMost32,
    [Option<A>; N]: Copy,
{
}

impl<A, const N: usize> Clone for All<A, { N }>
where
    [Option<A>; N]: core::array::LengthAtMost32,
    [Option<A>; N]: Clone,
{
    fn clone(&self) -> All<A, { N }> {
        All {
            sequences: self.sequences.clone(),
        }
    }
}

impl<A, const N: usize> Debug for All<A, { N }>
where
    [Option<A>; N]: core::array::LengthAtMost32,
    [Option<A>; N]: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let All { ref sequences } = *self;

        f.debug_struct("All").field("sequences", sequences).finish()
    }
}

impl<A, const N: usize> PartialEq for All<A, { N }>
where
    [Option<A>; N]: core::array::LengthAtMost32,
    [Option<A>; N]: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let All { ref sequences } = *self;

        sequences == &other.sequences
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct Countdown(usize);

    impl AutomationSequence<(), ()> for Countdown {
        type FaultInfo = ();

        fn poll(&mut self, _: &(), _: &mut ()) -> Transition<Self::FaultInfo> {
            unimplemented!()
        }
    }

    #[test]
    fn poll_all() {
        let items = All::new([Countdown(1), Countdown(5)]);

        fn assert_is_automation_sequence<A, I, O>(_: &A)
        where
            A: AutomationSequence<I, O>,
        {
        }

        assert_is_automation_sequence(&Countdown(0));
        assert_is_automation_sequence(&items);
    }
}
