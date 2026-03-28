use crate::Vec2;

/// Represents types that can represent the value of infinity.
pub trait Infinity {
    /// Infinity of the given type.
    const INFINITY: Self;
}

impl Infinity for f32 {
    const INFINITY: Self = f32::INFINITY;
}

impl Infinity for f64 {
    const INFINITY: Self = f64::INFINITY;
}

impl<T: Infinity> Infinity for Vec2<T> {
    const INFINITY: Self = Vec2::new(T::INFINITY, T::INFINITY);
}
