use std::cmp;

use crate::TwoComponent;

pub trait RangeExt: Sized + TwoComponent {
    /// If the second component is smaller than the first, it returns range
    /// where is the first component twice.
    fn valid_or_empty(self) -> Self
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
            .valid_or_empty()
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
}
