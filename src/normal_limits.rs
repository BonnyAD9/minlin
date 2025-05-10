/// Contains constants limiting the value to normal range.
pub trait NormalLimits {
    /// Maximum normal value. This is max for ints and 1 for floats.
    const NORM_MAX: Self;
    /// Minimum normal value. This is min for ints and 0 for floats.
    const NORM_MIN: Self;
}

macro_rules! impl_normal_limits_int {
    ($($i:ident),*) => {
        $(impl NormalLimits for $i {
            const NORM_MAX: Self = $i::MAX;
            const NORM_MIN: Self = $i::MIN;
        })*
    };
}

macro_rules! impl_normal_limits_float {
    ($($i:ident),*) => {
        $(impl NormalLimits for $i {
            const NORM_MAX: Self = 1.;
            const NORM_MIN: Self = 0.;
        })*
    };
}

impl_normal_limits_int!(u128, i128, u64, i64, u32, i32, u16, i16, u8, i8);
impl_normal_limits_float!(f64, f32);
