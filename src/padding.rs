use std::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

use crate::{CompArithm, Rect, Vec2, Vec4, Zero};

/// Type that represents padding.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Padding<T = usize>(pub Vec4<T>);

impl<T: Copy> Padding<T> {
    /// Create padding. Start from left and go clockwise.
    pub const fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Self(Vec4::new(left, top, right, bottom))
    }

    /// Get the offset of the padding.
    pub const fn offset(&self) -> Vec2<T> {
        self.0.xy()
    }

    /// Get the total size of the padding.
    pub fn size(&self) -> Vec2<T>
    where
        T: Add<Output = T>,
    {
        self.0.xy() + self.0.zw()
    }

    /// Get the left padding.
    pub const fn left(&self) -> T {
        self.0.x
    }

    /// Get the right padding.
    pub const fn right(&self) -> T {
        self.0.z
    }

    /// Get the top padding.
    pub const fn top(&self) -> T {
        self.0.y
    }

    /// Get the bottom padding.
    pub const fn bottom(&self) -> T {
        self.0.w
    }

    /// Create uniform padding.
    pub const fn uniform(v: T) -> Self {
        Self::new(v, v, v, v)
    }

    /// Create vertical padding.
    pub const fn vertical(v: T) -> Self
    where
        T: Zero,
    {
        Self::new(T::ZERO, v, T::ZERO, v)
    }

    /// Create horizontal padding.
    pub const fn horizontal(v: T) -> Self
    where
        T: Zero,
    {
        Self::new(v, T::ZERO, v, T::ZERO)
    }

    /// Get the horizontal size of the padding.
    pub fn xsize(&self) -> T
    where
        T: Add<Output = T>,
    {
        self.x + self.z
    }

    /// Get the vertical size of the padding.
    pub fn ysize(&self) -> T
    where
        T: Add<Output = T>,
    {
        self.y + self.w
    }
}

impl<T> Deref for Padding<T> {
    type Target = Vec4<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Padding<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy> From<T> for Padding<T> {
    fn from(value: T) -> Self {
        Self::uniform(value)
    }
}

impl<T: Copy> From<(T, T)> for Padding<T> {
    fn from((lr, tb): (T, T)) -> Self {
        Self(Vec4::new(lr, tb, lr, tb))
    }
}

impl<T> From<(T, T, T, T)> for Padding<T> {
    fn from((l, t, r, b): (T, T, T, T)) -> Self {
        Self(Vec4::new(l, t, r, b))
    }
}

impl<T: Copy> From<[T; 2]> for Padding<T> {
    fn from([lr, tb]: [T; 2]) -> Self {
        Self(Vec4::new(lr, tb, lr, tb))
    }
}

impl<T> From<[T; 4]> for Padding<T> {
    fn from([l, t, r, b]: [T; 4]) -> Self {
        Self(Vec4::new(l, t, r, b))
    }
}

impl<T: Copy + Sub<Output = T>> From<Rect<T>> for Padding<T> {
    fn from(value: Rect<T>) -> Self {
        let (pos, siz) = value.xy_zw();
        Self((pos, siz - pos).into())
    }
}

impl<A, T> Add<A> for Padding<T> where Vec4<T>: Add<A, Output = Vec4<T>> {
    type Output = Self;

    fn add(self, rhs: A) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl<T, R> Add<Padding<R>> for Padding<T> where T: Add<R> {
    type Output = Padding<T::Output>;

    fn add(self, rhs: Padding<R>) -> Self::Output {
        Padding(self.0 + rhs.0)
    }
}

impl<A, T> AddAssign<A> for Padding<T> where Vec4<T>: AddAssign<A> {
    fn add_assign(&mut self, rhs: A) {
        self.0 += rhs;
    }
}

impl<T, R> AddAssign<Padding<R>> for Padding<T> where T: AddAssign<R> {
    fn add_assign(&mut self, rhs: Padding<R>) {
        self.0 += rhs.0
    }
}

impl<A, T> Sub<A> for Padding<T> where Vec4<T>: Sub<A, Output = Vec4<T>> {
    type Output = Self;

    fn sub(self, rhs: A) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl<T, R> Sub<Padding<R>> for Padding<T> where T: Sub<R> {
    type Output = Padding<T::Output>;

    fn sub(self, rhs: Padding<R>) -> Self::Output {
        Padding(self.0 - rhs.0)
    }
}

impl<A, T> SubAssign<A> for Padding<T> where Vec4<T>: SubAssign<A> {
    fn sub_assign(&mut self, rhs: A) {
        self.0 -= rhs;
    }
}

impl<T, R> SubAssign<Padding<R>> for Padding<T> where T: SubAssign<R> {
    fn sub_assign(&mut self, rhs: Padding<R>) {
        self.0 -= rhs.0
    }
}
