use arrayvec::{Array, ArrayVec};
use core::{
    fmt::{self, Debug, Formatter},
    iter::FromIterator,
};

pub struct MotionPlanner<const N: usize>
where
    [Step; N]: Array,
{
    items: ArrayVec<[Step; N]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Step;

impl<const N: usize> MotionPlanner<{ N }> where [Step; N]: Array {}

impl<const N: usize> Default for MotionPlanner<{ N }>
where
    [Step; N]: core::array::LengthAtMost32 + Array,
{
    fn default() -> MotionPlanner<{ N }> {
        MotionPlanner {
            items: ArrayVec::default(),
        }
    }
}

impl<const N: usize> Clone for MotionPlanner<{ N }>
where
    [Step; N]: Array + core::array::LengthAtMost32,
    <[Step; N] as Array>::Item: Clone,
{
    fn clone(&self) -> MotionPlanner<{ N }> {
        MotionPlanner {
            items: ArrayVec::from_iter(self.items.iter().cloned()),
        }
    }
}

impl<const N: usize> Debug for MotionPlanner<{ N }>
where
    [Step; N]: Array + core::array::LengthAtMost32,
    <[Step; N] as Array>::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let MotionPlanner { ref items } = *self;

        f.debug_struct("MotionPlanner")
            .field("items", &items.as_slice())
            .finish()
    }
}

impl<const N: usize> PartialEq for MotionPlanner<{ N }>
where
    [Step; N]: Array + core::array::LengthAtMost32,
    <[Step; N] as Array>::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let MotionPlanner { ref items } = *self;

        items.as_slice() == other.items.as_slice()
    }
}
