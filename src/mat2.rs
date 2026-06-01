use std::ops::{Add, Mul, Sub};

use crate::{MapExt, Vec2, impl_traits};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Mat2<T = usize>(pub Vec2<Vec2<T>>);

impl<T> Mat2<T> {
    pub fn new(m00: T, m10: T, m01: T, m11: T) -> Self {
        Self(Vec2::new(Vec2::new(m00, m10), Vec2::new(m01, m11)))
    }

    pub fn rows(r0: impl Into<Vec2<T>>, r1: impl Into<Vec2<T>>) -> Self {
        Self(Vec2::new(r0.into(), r1.into()))
    }

    pub fn det(&self) -> <T::Output as Sub>::Output
    where
        T: Copy + Mul,
        T::Output: Sub,
    {
        self.0.x.x * self.0.y.y - self.0.x.y * self.0.y.x
    }

    pub fn r0(&self) -> Vec2<T>
    where
        T: Copy,
    {
        self.0.x
    }

    pub fn r1(&self) -> Vec2<T>
    where
        T: Copy,
    {
        self.0.y
    }

    pub fn c0(&self) -> Vec2<T>
    where
        T: Copy,
    {
        Vec2::new(self.0.x.x, self.0.y.x)
    }

    pub fn c1(&self) -> Vec2<T>
    where
        T: Copy,
    {
        Vec2::new(self.0.x.y, self.0.y.y)
    }
}

impl<T> MapExt for Mat2<T> {
    type Val = T;
    type This<R> = Mat2<R>;

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        Mat2(self.0.map(|a| a.map(&mut f)))
    }

    fn mutate(&mut self, mut f: impl FnMut(&mut Self::Val)) {
        self.0.mutate(|a| a.mutate(&mut f))
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul<Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn mul(self, rhs: Mat2<T>) -> Self::Output {
        Mat2::new(
            self.r0().dot(rhs.c0()),
            self.r0().dot(rhs.c1()),
            self.r1().dot(rhs.c0()),
            self.r1().dot(rhs.c1()),
        )
    }
}

impl_traits!(Mat2<T> =>
    Add AddAssign SDiv SDivAssign SRem SRemAssign
);
