pub trait Zero {
    const ZERO: Self;
}

#[macro_export]
macro_rules! impl_zero {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: $crate::Zero + Copy> $crate::Zero for $t<$g> {
            const ZERO: Self = Self::same_components($g::ZERO);
        }
    };
}

macro_rules! impl_base_zero {
    ($($t:ident),* -> $v:expr) => {
        $(impl Zero for $t {
            const ZERO: Self = $v;
        })*
    };
}

impl_base_zero!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128 -> 0);
impl_base_zero!(f32, f64 -> 0.);
