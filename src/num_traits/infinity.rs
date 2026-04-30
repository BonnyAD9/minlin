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

#[macro_export]
macro_rules! impl_infinity {
    ($t:ident < $g:ident >) => {
        impl<$g: $crate::Infinity + Copy> $crate::Infinity for $t<$g> {
            const INFINITY: Self = Self::same_components($g::INFINITY);
        }
    };
}
