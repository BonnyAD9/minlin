use std::{cmp, ops::Range};

use crate::{TwoComponent, Vec2};

pub trait RangeExt: Sized + TwoComponent {
    /// If the second component is smaller than the first, it returns range
    /// where is the first component twice.
    fn valid_range_or_empty(self) -> Self
    where
        Self::Val: Ord + Clone,
    {
        let (a, b) = self.to_components();
        if a > b {
            Self::from_components(a.clone(), a)
        } else {
            Self::from_components(a, b)
        }
    }

    /// Checks whether the given value is in range.
    fn contains(&self, v: &Self::Val) -> bool
    where
        Self::Val: PartialOrd,
    {
        v >= self.comp1() && v < self.comp2()
    }

    /// Calculates intersection of the two ranges. If the intersection is empty
    /// the resulting range will have the largest value as both of its
    /// components.
    fn intersect(self, other: impl Into<Self>) -> Self
    where
        Self::Val: Ord + Clone,
    {
        let (s1, e1) = self.to_components();
        let (s2, e2) = other.into().to_components();
        Self::from_components(cmp::max(s1, s2), cmp::min(e1, e2))
            .valid_range_or_empty()
    }

    /// Checks whether the two ranges intersect.
    fn intersects(&self, other: impl AsRef<Self>) -> bool
    where
        Self::Val: Ord,
    {
        let other = other.as_ref();
        if self.comp1() < other.comp2() {
            self.comp2() > other.comp1()
        } else {
            other.comp2() > self.comp1()
        }
    }

    /// Checks whether the two ranges can be joined into single range.
    fn touches(&self, other: impl AsRef<Self>) -> bool
    where
        Self::Val: Ord,
    {
        let other = other.as_ref();
        if self.comp1() < other.comp2() {
            self.comp2() >= other.comp1()
        } else {
            other.comp2() >= self.comp1()
        }
    }

    /// Join the two ranges. If they cannot be joined, they are so (smaller
    /// range is first).
    fn join(self, other: impl Into<Self>) -> (Self, Option<Self>)
    where
        Self::Val: Ord,
    {
        let (s1, e1) = self.to_components();
        let (s2, e2) = other.into().to_components();
        if s1 < s2 {
            if e1 < s2 {
                // Ranges don't touch
                (
                    Self::from_components(s1, e1),
                    Some(Self::from_components(s2, e2)),
                )
            } else {
                // Ranges touch
                (Self::from_components(s1, cmp::max(e1, e2)), None)
            }
        } else if e2 < e1 {
            // Ranges don't touch
            (
                Self::from_components(s2, e2),
                Some(Self::from_components(s1, e1)),
            )
        } else {
            (Self::from_components(s2, cmp::max(e1, e2)), None)
        }
    }

    /// Join the two ranges. If they cannot be joined the resulting range will
    /// also cover the gap.
    fn join_gap(self, other: impl Into<Self>) -> Self
    where
        Self::Val: Ord,
    {
        let (s1, e1) = self.to_components();
        let (s2, e2) = other.into().to_components();
        Self::from_components(cmp::min(s1, s2), cmp::max(e1, e2))
    }

    /// Subtract the given range from this range.
    ///
    /// If the other range encapsulates this range, the result is range with
    /// the end of the inner range in both components.
    ///
    /// The resulting ranges are always ordered.
    fn sub_range(self, other: impl Into<Self>) -> (Self, Option<Self>)
    where
        Self::Val: Ord + Clone,
    {
        let (s1, e1) = self.to_components();
        let (s2, e2) = other.into().to_components();

        if s1 < s2 {
            // self is before or over
            if s2 < e1 {
                // ranges overlap
                if e2 < e1 {
                    // self is over
                    (
                        Self::from_components(s1, s2),
                        Some(Self::from_components(e2, e1)),
                    )
                } else {
                    // self is before with overlap
                    (Self::from_components(s1, s2), None)
                }
            } else {
                // Self is before without overlap
                (Self::from_components(s1, e1), None)
            }
        } else if e1 >= e2 {
            // Self is after
            if s1 > e2 {
                // No overlap
                (Self::from_components(s1, e2), None)
            } else {
                // Overlap
                (Self::from_components(e2, e1), None)
            }
        } else {
            // Self is inside
            (Self::from_components(e1.clone(), e1), None)
        }
    }

    /// Subtract the given range from this range. If the other range is fully
    /// inside this range, return this range.
    fn sub_range_gap(self, other: impl Into<Self>) -> Self
    where
        Self::Val: Ord + Clone,
    {
        let (s1, e1) = self.to_components();
        let (s2, e2) = other.into().to_components();

        if s1 < s2 {
            // self is before or over
            if s2 < e1 {
                // ranges overlap
                if e2 < e1 {
                    // self is over
                    Self::from_components(s1, e1)
                } else {
                    // self is before with overlap
                    Self::from_components(s1, s2)
                }
            } else {
                // Self is before without overlap
                Self::from_components(s1, e1)
            }
        } else if e1 >= e2 {
            // Self is after
            if s1 > e2 {
                // No overlap
                Self::from_components(s1, e2)
            } else {
                // Overlap
                Self::from_components(e2, e1)
            }
        } else {
            // Self is inside
            Self::from_components(e1.clone(), e1)
        }
    }

    /// Calculate symetric subtraction of the two ranges.
    ///
    /// The resulting ranges are always ordered.
    fn sym_sub_range(self, other: impl Into<Self>) -> (Self, Self)
    where
        Self::Val: Ord,
    {
        let (s1, e1) = self.to_components();
        let (s2, e2) = other.into().to_components();

        if s1 < s2 {
            if s2 >= e1 {
                (Self::from_components(s1, e1), Self::from_components(s2, e2))
            } else if e1 < e2 {
                (Self::from_components(s1, s2), Self::from_components(e1, e2))
            } else {
                (Self::from_components(s1, s2), Self::from_components(e2, e1))
            }
        } else if s1 >= e2 {
            (Self::from_components(s2, e2), Self::from_components(s1, e1))
        } else if e1 > e2 {
            (Self::from_components(s2, s1), Self::from_components(e2, e1))
        } else {
            (Self::from_components(s2, s1), Self::from_components(e1, e2))
        }
    }
}

impl<T> RangeExt for Vec2<T> {}
impl<T> RangeExt for Range<T> {}
