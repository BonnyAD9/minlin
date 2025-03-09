pub trait Zero {
    const ZERO: Self;
}

macro_rules! impl_one {
    ($($t:ident),* -> $v:expr) => {
        $(impl Zero for $t {
            const ZERO: Self = $v;
        })*
    };
}

impl_one!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128 -> 0);
impl_one!(f32, f64 -> 0.);
