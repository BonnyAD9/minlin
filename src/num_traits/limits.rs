/// Implements number limits.
pub trait Limits {
    /// The smalles finite value.
    const MIN: Self;
    /// The largest finite value.
    const MAX: Self;
}

macro_rules! impl_limits {
    ($($t:ident),* $(,)?) => {
        $(impl Limits for $t {
            const MIN: Self = $t::MIN;
            const MAX: Self = $t::MAX;
        })*
    };
}

impl_limits!(
    u8, i8, u16, i16, u32, i32, f32, u64, i64, f64, usize, isize, u128, i128,
);
