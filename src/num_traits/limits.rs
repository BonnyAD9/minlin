/// Implements number limits.
pub trait Limits {
    /// The smalles finite value.
    const MIN: Self;
    /// The largest finite value.
    const MAX: Self;
}

#[macro_export]
macro_rules! impl_limits {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: $crate::Limits + Copy> $crate::Limits for $t<$g> {
            const MIN: Self = Self::same_components($g::MIN);
            const MAX: Self = Self::same_components($g::MAX);
        }
    };
}

macro_rules! impl_base_limits {
    ($($t:ident),* $(,)?) => {
        $(impl Limits for $t {
            const MIN: Self = $t::MIN;
            const MAX: Self = $t::MAX;
        })*
    };
}

impl_base_limits!(
    u8, i8, u16, i16, u32, i32, f32, u64, i64, f64, usize, isize, u128, i128,
);
