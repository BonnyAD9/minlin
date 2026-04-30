use std::ops::Range;

use crate::{Vec2, Vec3, Vec4};

/// Trait for types that can be mapped.
pub trait MapExt: Sized {
    type Val;
    type This<R>;

    /// Map the components.
    fn map<R>(self, f: impl FnMut(Self::Val) -> R) -> Self::This<R>;
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
