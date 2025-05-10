use std::ops::{Add, Div};

use crate::{Two, Vec2};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Vec4<T = usize> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T> {
    /// Construct new Vec4.
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Width of rectangle represented by this vector.
    pub fn width(&self) -> &T {
        &self.z
    }

    /// Height of rectangle represented by this vector.
    pub fn height(&self) -> &T {
        &self.w
    }

    /// Width of rectangle represented by this vector.
    pub fn width_mut(&mut self) -> &mut T {
        &mut self.z
    }

    /// Height of rectangle represented by this vector.
    pub fn height_mut(&mut self) -> &mut T {
        &mut self.w
    }

    /// Width of rectangle represented by this vector.
    pub fn set_width(&mut self, width: T) {
        self.z = width;
    }

    /// Height of rectangle represented by this vector.
    pub fn set_height(&mut self, height: T) {
        self.w = height;
    }

    /// Get the first two components.
    pub fn xy(self) -> Vec2<T> {
        (self.x, self.y).into()
    }

    /// Get the last two components.
    pub fn zw(self) -> Vec2<T> {
        (self.z, self.w).into()
    }

    /// Get position of this vector as rectangle.
    pub fn position(self) -> Vec2<T> {
        self.xy()
    }

    /// Get size of this vector as rectangle.
    pub fn size(self) -> Vec2<T> {
        self.zw()
    }

    /// Split the vector.
    pub fn xy_zw(self) -> (Vec2<T>, Vec2<T>) {
        ((self.x, self.y).into(), (self.z, self.w).into())
    }

    /// Interpret this vec4 as rectangle and return its center.
    pub fn rect_center(self) -> Vec2<<T as Add<<T as Div>::Output>>::Output>
    where
        T: Two + Div + Add<<T as Div>::Output> + Copy,
    {
        let (p, s) = self.xy_zw();
        p + s / T::TWO
    }
}

impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Self { x, y, z, w }
    }
}

impl<T> From<[T; 4]> for Vec4<T> {
    fn from([x, y, z, w]: [T; 4]) -> Self {
        Self { x, y, z, w }
    }
}
