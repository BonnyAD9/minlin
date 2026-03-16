use std::ops::{Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

use crate::{MapExt, Saturating, Vec2, Vec3, Vec4};

/// Trait for componentwise arithmetic.
pub trait CompArithm: MapExt {
    /// Do componentwise operation.
    fn cjoin<R, O>(
        self,
        other: impl Into<Self::This<O>>,
        f: impl FnMut(Self::Val, O) -> R,
    ) -> Self::This<R>;

    /// Do componentwise mutation.
    fn cjoin_assign<O>(
        &mut self,
        other: impl Into<Self::This<O>>,
        f: impl FnMut(&mut Self::Val, O),
    );

    /// Do componentwise multiplication.
    fn cmul<O>(
        self,
        other: impl Into<Self::This<O>>,
    ) -> Self::This<<Self::Val as Mul<O>>::Output>
    where
        Self::Val: Mul<O>,
    {
        self.cjoin(other, |a, b| a * b)
    }

    /// Do componentwise in place multiplication.
    fn cmul_assign<O>(&mut self, other: impl Into<Self::This<O>>)
    where
        Self::Val: MulAssign<O>,
    {
        self.cjoin_assign(other, |a, b| *a *= b)
    }

    /// Do componentwise division.
    fn cdiv<O>(
        self,
        other: impl Into<Self::This<O>>,
    ) -> Self::This<<Self::Val as Div<O>>::Output>
    where
        Self::Val: Div<O>,
    {
        self.cjoin(other, |a, b| a / b)
    }

    /// Do componentwise in place division.
    fn cdiv_assign<O>(&mut self, other: impl Into<Self::This<O>>)
    where
        Self::Val: DivAssign<O>,
    {
        self.cjoin_assign(other, |a, b| *a /= b)
    }

    /// Do componentwise remainder (modulo).
    fn crem<O>(
        self,
        other: impl Into<Self::This<O>>,
    ) -> Self::This<<Self::Val as Rem<O>>::Output>
    where
        Self::Val: Rem<O>,
    {
        self.cjoin(other, |a, b| a % b)
    }

    /// Do componentwise in place division.
    fn crem_assign<O>(&mut self, other: impl Into<Self::This<O>>)
    where
        Self::Val: RemAssign<O>,
    {
        self.cjoin_assign(other, |a, b| *a %= b)
    }

    /// Do componentwise saturating subtraction.
    fn saturating_sub(
        self,
        other: impl Into<Self::This<Self::Val>>,
    ) -> Self::This<Self::Val>
    where
        Self::Val: Saturating,
    {
        self.cjoin(other, Self::Val::saturating_sub)
    }

    /// Do componentwise saturating subtraction in place.
    fn saturating_sub_assign(
        &mut self,
        other: impl Into<Self::This<Self::Val>>,
    ) where
        Self::Val: Saturating + Copy,
    {
        self.cjoin_assign(other, |a, b| *a = a.saturating_sub(b))
    }

    /// Do componentwise saturating addition.
    fn saturating_add(
        self,
        other: impl Into<Self::This<Self::Val>>,
    ) -> Self::This<Self::Val>
    where
        Self::Val: Saturating,
    {
        self.cjoin(other, Self::Val::saturating_add)
    }

    /// Do componentwise saturating addition in place.
    fn saturating_add_assign(
        &mut self,
        other: impl Into<Self::This<Self::Val>>,
    ) where
        Self::Val: Saturating + Copy,
    {
        self.cjoin_assign(other, |a, b| *a = a.saturating_add(b))
    }

    /// Do componentwise saturating multiplication.
    fn saturating_cmul(
        self,
        other: impl Into<Self::This<Self::Val>>,
    ) -> Self::This<Self::Val>
    where
        Self::Val: Saturating,
    {
        self.cjoin(other, Self::Val::saturating_mul)
    }

    /// Do componentwise saturating multiplication in place.
    fn saturating_cmul_assign(
        &mut self,
        other: impl Into<Self::This<Self::Val>>,
    ) where
        Self::Val: Saturating + Copy,
    {
        self.cjoin_assign(other, |a, b| *a = a.saturating_mul(b))
    }
}

impl<T> CompArithm for Vec2<T> {
    fn cjoin<R, O>(
        self,
        other: impl Into<Self::This<O>>,
        mut f: impl FnMut(Self::Val, O) -> R,
    ) -> Self::This<R> {
        let o = other.into();
        Vec2::new(f(self.x, o.x), f(self.y, o.y))
    }

    fn cjoin_assign<O>(
        &mut self,
        other: impl Into<Self::This<O>>,
        mut f: impl FnMut(&mut Self::Val, O),
    ) {
        let o = other.into();
        f(&mut self.x, o.x);
        f(&mut self.y, o.y);
    }
}

impl<T> CompArithm for Vec3<T> {
    fn cjoin<R, O>(
        self,
        other: impl Into<Self::This<O>>,
        mut f: impl FnMut(Self::Val, O) -> R,
    ) -> Self::This<R> {
        let o = other.into();
        Vec3::new(f(self.x, o.x), f(self.y, o.y), f(self.z, o.z))
    }

    fn cjoin_assign<O>(
        &mut self,
        other: impl Into<Self::This<O>>,
        mut f: impl FnMut(&mut Self::Val, O),
    ) {
        let o = other.into();
        f(&mut self.x, o.x);
        f(&mut self.y, o.y);
        f(&mut self.z, o.z);
    }
}

impl<T> CompArithm for Vec4<T> {
    fn cjoin<R, O>(
        self,
        other: impl Into<Self::This<O>>,
        mut f: impl FnMut(Self::Val, O) -> R,
    ) -> Self::This<R> {
        let o = other.into();
        Vec4::new(
            f(self.x, o.x),
            f(self.y, o.y),
            f(self.z, o.z),
            f(self.w, o.w),
        )
    }

    fn cjoin_assign<O>(
        &mut self,
        other: impl Into<Self::This<O>>,
        mut f: impl FnMut(&mut Self::Val, O),
    ) {
        let o = other.into();
        f(&mut self.x, o.x);
        f(&mut self.y, o.y);
        f(&mut self.z, o.z);
        f(&mut self.w, o.w);
    }
}
