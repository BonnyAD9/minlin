use std::ops::{AddAssign, Bound, Deref, DerefMut, RangeBounds};

use crate::{One, Vec2};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Range<T = usize>(pub Vec2<T>);

impl<T> From<std::ops::Range<T>> for Range<T> {
    fn from(value: std::ops::Range<T>) -> Self {
        Self(value.into())
    }
}

impl<T> From<Vec2<T>> for Range<T> {
    fn from(value: Vec2<T>) -> Self {
        Self(value)
    }
}

impl<T> From<Range<T>> for std::ops::Range<T> {
    fn from(value: Range<T>) -> Self {
        value.0.into()
    }
}

impl<T> From<Range<T>> for Vec2<T> {
    fn from(value: Range<T>) -> Self {
        value.0
    }
}

impl<T> Deref for Range<T> {
    type Target = Vec2<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Range<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.x)
    }

    fn end_bound(&self) -> Bound<&T> {
        Bound::Excluded(&self.y)
    }
}

impl<T> Iterator for Range<T>
where
    T: AddAssign + One + Copy + PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.y {
            let res = self.x;
            self.x += T::ONE;
            Some(res)
        } else {
            None
        }
    }
}
