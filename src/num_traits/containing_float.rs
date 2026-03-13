use crate::Float;

/// Trait for integers. For each interger, it uses the smallest float that can
/// contain all value of the integer loslessly.
pub trait ContainingFloat {
    /// Best float type for this integer.
    type Float: Float;

    /// Convert intgerer to float.
    fn to_float(self) -> Self::Float;
}

macro_rules! impl_containing_float {
    ($($t:ident),* -> $f:ident) => {
        $(impl ContainingFloat for $t {
            type Float = $f;

            fn to_float(self) -> Self::Float {
                self as $f
            }
        })*
    };
}

impl_containing_float!(u8, i8, u16, i16 -> f32);
impl_containing_float!(u32, i32, u64, i64, usize, isize, u128, i128 -> f64);
