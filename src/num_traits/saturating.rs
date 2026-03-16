/// Generic trait for types that support saturating arithmetic.
pub trait Saturating {
    /// Subtract the value. Clamp the result to valid range instead of
    /// overflowing.
    fn saturating_sub(self, other: Self) -> Self;

    /// Add the value. Clamp the result to valid range instead of overflowing.
    fn saturating_add(self, other: Self) -> Self;

    /// Multiply by the value. Clamp the result to valid range instead of
    /// overflowing.
    fn saturating_mul(self, other: Self) -> Self;
}

macro_rules! impl_saturating {
    ($($i:ident),*) => {
        $(impl Saturating for $i {
            fn saturating_sub(self, other: Self) -> Self {
                $i::saturating_sub(self, other)
            }

            fn saturating_add(self, other: Self) -> Self {
                $i::saturating_add(self, other)
            }

            fn saturating_mul(self, other: Self) -> Self {
                $i::saturating_mul(self, other)
            }
        })*
    };
}

impl_saturating!(
    u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128
);
