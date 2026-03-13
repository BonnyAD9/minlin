use std::ops::Range;

use crate::{Cast, Vec2, Vec3};

pub trait CastExt: Sized {
    type Val;
    type This<R>;

    /// Map the components.
    fn map<R>(self, f: impl FnMut(Self::Val) -> R) -> Self::This<R>;

    /// Convert the values to the given type using `Into`.
    fn convert<R>(self) -> Self::This<R>
    where
        Self::Val: Into<R>,
    {
        self.map(|a| a.into())
    }

    /// Cast the values to the given type.
    fn cast<R>(self) -> Self::This<R>
    where
        Self::Val: Cast<R>,
    {
        self.map(|a| a.cast())
    }
}

impl<T> CastExt for Vec2<T> {
    type Val = T;
    type This<R> = Vec2<R>;

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        Vec2::new(f(self.x), f(self.y))
    }
}

impl<T> CastExt for Range<T> {
    type Val = T;
    type This<R> = Range<R>;

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        f(self.start)..f(self.end)
    }
}

impl<T> CastExt for (T, T) {
    type Val = T;
    type This<R> = (R, R);

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        (f(self.0), f(self.1))
    }
}

impl<T> CastExt for Vec3<T> {
    type Val = T;
    type This<R> = Vec3<R>;

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        Vec3::new(f(self.x), f(self.y), f(self.z))
    }
}
