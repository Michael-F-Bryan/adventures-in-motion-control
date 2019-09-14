/// A top-level component which will be polled at frequent intervals.
pub trait System<In: ?Sized, Out: ?Sized> {
    fn poll(&mut self, inputs: &In, outputs: &mut Out);
}

impl<F, In: ?Sized, Out: ?Sized> System<In, Out> for F
where
    F: FnMut(&In, &mut Out),
{
    fn poll(&mut self, inputs: &In, outputs: &mut Out) {
        (*self)(inputs, outputs)
    }
}
