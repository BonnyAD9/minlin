pub trait Convert<O> {
    fn convert(self) -> O;
}

#[macro_export]
macro_rules! impl_convert {
    ($t:ident < $g:ident >) => {
        impl<$g: $crate::Convert<O>, O> $crate::Convert<$t<O>> for $t<$g> {
            fn convert(self) -> $t<O> {
                self.map($g::convert)
            }
        }
    };
}

macro_rules! impl_base_convert {
    ($($i:ident),* -> $o:ident) => {
        $(impl Convert<$o> for $i {
            fn convert(self) -> $o {
                self as $o
            }
        })*
    };
}

impl_base_convert!(u8 -> u8);
impl_base_convert!(i8 -> i8);
impl_base_convert!(u8, u16 -> u16);
impl_base_convert!(u8, i8, i16 -> i16);
impl_base_convert!(u8, u16, u32 -> u32);
impl_base_convert!(u8, i8, u16, i16, i32 -> i32);
impl_base_convert!(u8, u16, u32, u64 -> u64);
impl_base_convert!(u8, i8, u16, i16, u32, i32, i64 -> i64);
impl_base_convert!(u8, u16, u32, u64, u128 -> u128);
impl_base_convert!(u8, i8, u16, i16, u32, i32, u64, i64, i128 -> i128);
impl_base_convert!(u8, i8, u16, i16, f32 -> f32);
impl_base_convert!(u8, i8, u16, i16, u32, i32, f32, f64 -> f64);
