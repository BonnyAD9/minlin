pub trait Two {
    const TWO: Self;
}

macro_rules! impl_two {
    ($($t:ident),* -> $v:expr) => {
        $(impl Two for $t {
            const TWO: Self = $v;
        })*
    };
}

impl_two!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128 -> 2);
impl_two!(f32, f64 -> 2.);
