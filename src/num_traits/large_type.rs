/// Represents type that has larger type for operations that may be outside of normal values.
pub trait LargeType {
    type Large;

    /// Convert this type to its larger type.
    fn to_large(self) -> Self::Large;

    /// Covert the large type from this type.
    fn from_large(l: Self::Large) -> Self;
}

macro_rules! impl_large_type {
    ($($s:ident -> $l:ident),* $(,)?) => {
        $(impl LargeType for $s {
            type Large = $l;

            fn to_large(self) -> Self::Large {
                self as $l
            }

            fn from_large(l: Self::Large) -> Self {
                l as $s
            }
        })*
    };
}

impl_large_type!(
    u128 -> f64, i128 -> f64, u64 -> u128, i64 -> i128, f64 -> f64, u32 -> u64,
    i32 -> i64, f32 -> f64, u16 -> u32, i16 -> i32, u8 -> u16, i8 -> i16,
);
