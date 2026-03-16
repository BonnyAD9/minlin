/// Generic trait for types that support saturating arithmetic.
pub trait Checked: Sized {
    /// Subtract the value. Clamp the result to valid range instead of
    /// overflowing.
    fn checked_sub(self, other: Self) -> Option<Self>;

    /// Add the value. Clamp the result to valid range instead of overflowing.
    fn checked_add(self, other: Self) -> Option<Self>;

    /// Multiply by the value. Clamp the result to valid range instead of
    /// overflowing.
    fn checked_mul(self, other: Self) -> Option<Self>;
}

macro_rules! impl_saturating {
    ($($i:ident),*) => {
        $(impl Checked for $i {
            fn checked_sub(self, other: Self) -> Option<Self> {
                $i::checked_sub(self, other)
            }

            fn checked_add(self, other: Self) -> Option<Self> {
                $i::checked_add(self, other)
            }

            fn checked_mul(self, other: Self) -> Option<Self> {
                $i::checked_mul(self, other)
            }
        })*
    };
}

impl_saturating!(
    u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128
);
