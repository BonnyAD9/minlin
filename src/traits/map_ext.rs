use std::ops::Range;

/// Trait for types that can be mapped.
pub trait MapExt: Sized {
    type Val;
    type This<R>;

    /// Map the components.
    fn map<R>(self, f: impl FnMut(Self::Val) -> R) -> Self::This<R>;

    /// Map the components in place.
    fn mutate(&mut self, f: impl FnMut(&mut Self::Val));
}

impl<T> MapExt for Range<T> {
    type Val = T;
    type This<R> = Range<R>;

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        f(self.start)..f(self.end)
    }

    fn mutate(&mut self, mut f: impl FnMut(&mut Self::Val)) {
        f(&mut self.start);
        f(&mut self.end);
    }
}

impl<T> MapExt for (T, T) {
    type Val = T;
    type This<R> = (R, R);

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        (f(self.0), f(self.1))
    }

    fn mutate(&mut self, mut f: impl FnMut(&mut Self::Val)) {
        f(&mut self.0);
        f(&mut self.1);
    }
}
