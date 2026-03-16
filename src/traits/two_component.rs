use std::ops::Range;

use crate::Vec2;

/// Trait that represents types that have two components.
pub trait TwoComponent {
    type Val;

    /// Creates two component value from its components.
    fn from_components(c1: Self::Val, c2: Self::Val) -> Self;

    /// Gets the components.
    fn to_components(self) -> (Self::Val, Self::Val);

    /// Gets the first of the two components.
    fn comp1(&self) -> &Self::Val;

    /// Gets the second of the two components.
    fn comp2(&self) -> &Self::Val;
}

impl<T> TwoComponent for Vec2<T> {
    type Val = T;

    fn from_components(c1: Self::Val, c2: Self::Val) -> Self {
        Vec2::new(c1, c2)
    }

    fn to_components(self) -> (Self::Val, Self::Val) {
        self.into()
    }

    fn comp1(&self) -> &Self::Val {
        &self.x
    }

    fn comp2(&self) -> &Self::Val {
        &self.y
    }
}

impl<T> TwoComponent for (T, T) {
    type Val = T;

    fn from_components(c1: Self::Val, c2: Self::Val) -> Self {
        (c1, c2)
    }

    fn to_components(self) -> (Self::Val, Self::Val) {
        self
    }

    fn comp1(&self) -> &Self::Val {
        &self.0
    }

    fn comp2(&self) -> &Self::Val {
        &self.1
    }
}

impl<T> TwoComponent for Range<T> {
    type Val = T;

    fn from_components(c1: Self::Val, c2: Self::Val) -> Self {
        c1..c2
    }

    fn to_components(self) -> (Self::Val, Self::Val) {
        (self.start, self.end)
    }

    fn comp1(&self) -> &Self::Val {
        &self.start
    }

    fn comp2(&self) -> &Self::Val {
        &self.end
    }
}
