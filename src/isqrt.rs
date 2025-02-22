pub trait Isqrt {
    fn isqrt(self) -> Self;
}

macro_rules! impl_isqrt {
    ($($i:ident),*) => {
        $(impl Isqrt for $i {
            fn isqrt(self) -> Self {
                $i::isqrt(self)
            }
        })*
    };
}

impl_isqrt!(
    u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128
);
