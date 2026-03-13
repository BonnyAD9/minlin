pub trait One {
    const ONE: Self;
}

macro_rules! impl_one {
    ($($t:ident),* -> $v:expr) => {
        $(impl One for $t {
            const ONE: Self = $v;
        })*
    };
}

impl_one!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128 -> 1);
impl_one!(f32, f64 -> 1.);
