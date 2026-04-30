/// Represents types that can represent the value of negative infinity.
pub trait NegInfinity {
    /// Negative infinity of the given type.
    const NEG_INFINITY: Self;
}

impl NegInfinity for f32 {
    const NEG_INFINITY: Self = f32::NEG_INFINITY;
}

impl NegInfinity for f64 {
    const NEG_INFINITY: Self = f64::NEG_INFINITY;
}

#[macro_export]
macro_rules! impl_neg_infinity {
    ($t:ident < $g:ident >) => {
        impl<$g: $crate::NegInfinity + Copy> $crate::NegInfinity for $t<$g> {
            const NEG_INFINITY: Self = Self::same_components($g::NEG_INFINITY);
        }
    };
}
