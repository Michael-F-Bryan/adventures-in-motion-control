use core::ops::Add;
use uom::{
    si::{f32::Length, length::Unit},
    Conversion,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Point {
    pub x: Length,
    pub y: Length,
    pub z: Length,
}

impl Point {
    /// Create a new [`Point`] in a particular unit system.
    pub fn new<N>(x: f32, y: f32, z: f32) -> Self
    where
        N: Unit + Conversion<f32, T = f32>,
    {
        Point {
            x: Length::new::<N>(x),
            y: Length::new::<N>(y),
            z: Length::new::<N>(z),
        }
    }

    /// Get the underlying values in a particular unit system.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aimc_motion::movements::Point;
    /// use uom::si::length::{inch, millimeter};
    ///
    /// let p = Point::new::<inch>(10.0, 20.0, 30.0);
    /// # let p = p.round::<millimeter>(); // ugh, floating point math...
    /// assert_eq!(p.converted_to::<millimeter>(), (254.0, 254.0*2.0, 254.0*3.0));
    /// ```
    pub fn converted_to<N>(self) -> (f32, f32, f32)
    where
        N: Unit + Conversion<f32, T = f32>,
    {
        (self.x.get::<N>(), self.y.get::<N>(), self.z.get::<N>())
    }

    /// Round `x`, `y`, and `z` to the nearest integer when converted to `N`
    /// units.
    pub fn round<N>(self) -> Self
    where
        N: Unit + Conversion<f32, T = f32>,
    {
        Point {
            x: self.x.round::<N>(),
            y: self.y.round::<N>(),
            z: self.z.round::<N>(),
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
