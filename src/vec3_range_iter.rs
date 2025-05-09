use std::ops::AddAssign;

use crate::{One, Vec2, Vec3};

pub struct Vec3RangeIter<T> {
    start: Vec3<T>,
    end: Vec3<T>,
    xy: Vec2<T>,
}

impl<T> Vec3RangeIter<T> {
    pub fn new(start: Vec3<T>, end: Vec3<T>) -> Self
    where
        T: Copy,
    {
        Self {
            xy: start.xy(),
            start,
            end,
        }
    }

    pub fn contains(&self, other: impl Into<Vec3<T>>) -> bool
    where
        T: Ord,
    {
        let Vec3 { x, y, z } = other.into();
        x >= self.start.x
            && x < self.end.x
            && y >= self.start.y
            && y < self.end.y
            && z >= self.start.z
            && z < self.end.z
    }
}

impl<T> Iterator for Vec3RangeIter<T>
where
    T: Copy + AddAssign + PartialOrd + One,
{
    type Item = Vec3<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.xy.x >= self.end.x {
            if self.xy.x == self.start.x {
                return None;
            }
            self.xy.x = self.start.x;
            self.xy.y += T::ONE;
        }

        if self.xy.y >= self.end.y {
            if self.xy.y == self.start.y {
                return None;
            }
            self.xy.y = self.start.y;
            self.start.z += T::ONE;
        }

        if self.start.z > self.end.z {
            return None;
        }

        let res = (self.xy, self.start.z);
        self.xy.x += T::ONE;
        Some(res.into())
    }
}
