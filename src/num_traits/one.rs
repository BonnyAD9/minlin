pub trait One {
    const ONE: Self;
}

#[macro_export]
macro_rules! impl_one {
    ($t:ident < $g:ident >) => {
        impl<$g: $crate::One + Copy> $crate::One for $t<$g> {
            const ONE: Self = Self::same_components($g::ONE);
        }
    };
}

macro_rules! impl_base_one {
    ($($t:ident),* -> $v:expr) => {
        $(impl One for $t {
            const ONE: Self = $v;
        })*
    };
}

impl_base_one!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128 -> 1);
impl_base_one!(f32, f64 -> 1.);
