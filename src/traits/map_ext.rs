use std::ops::Range;

use crate::{Cast, Scale, Vec2, Vec3, Vec4};

/// Trait for types that can be mapped.
pub trait MapExt: Sized {
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

    /// Scale the components to the given type. The components are scaled with
    /// their normal range. For floats this range is 0 to 1 and for ints it is
    /// their full range.
    fn scale<R>(self) -> Self::This<R>
    where
        Self::Val: Scale<R>,
    {
        self.map(|a| a.scale())
    }
}

impl<T> MapExt for Vec2<T> {
    type Val = T;
    type This<R> = Vec2<R>;

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        Vec2::new(f(self.x), f(self.y))
    }
}

impl<T> MapExt for Range<T> {
    type Val = T;
    type This<R> = Range<R>;

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        f(self.start)..f(self.end)
    }
}

impl<T> MapExt for (T, T) {
    type Val = T;
    type This<R> = (R, R);

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        (f(self.0), f(self.1))
    }
}

impl<T> MapExt for Vec3<T> {
    type Val = T;
    type This<R> = Vec3<R>;

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        Vec3::new(f(self.x), f(self.y), f(self.z))
    }
}

impl<T> MapExt for Vec4<T> {
    type Val = T;
    type This<R> = Vec4<R>;

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        Vec4::new(f(self.x), f(self.y), f(self.z), f(self.w))
    }
}
