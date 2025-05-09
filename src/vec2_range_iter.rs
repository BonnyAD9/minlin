use std::ops::AddAssign;

use crate::{One, Vec2};

pub struct Vec2RangeIter<T> {
    start: Vec2<T>,
    end: Vec2<T>,
    x: T,
}

impl<T> Vec2RangeIter<T>
where
    T: Copy,
{
    pub fn new(start: Vec2<T>, end: Vec2<T>) -> Self {
        Self {
            x: start.x,
            start,
            end,
        }
    }
}

impl<T> Vec2RangeIter<T> {
    pub fn contains(&self, other: impl Into<Vec2<T>>) -> bool
    where
        T: Ord,
    {
        let Vec2 { x, y } = other.into();
        x >= self.start.x
            && x < self.end.x
            && y >= self.start.y
            && y < self.end.y
    }
}

impl<T> Iterator for Vec2RangeIter<T>
where
    T: Copy + AddAssign + PartialOrd + One,
{
    type Item = Vec2<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.end.x {
            if self.x == self.start.x {
                return None;
            }
            self.x = self.start.x;
            self.start.y += T::ONE;
        }

        if self.start.y >= self.end.y {
            return None;
        }

        let res = (self.x, self.start.y);
        self.x += T::ONE;
        Some(res.into())
    }
}
