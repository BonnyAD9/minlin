pub trait Cast<O> {
    /// Cast type to numeric type with smaller range. Ignore potential
    /// overflows.
    fn cast(self) -> O;
}

macro_rules! impl_wrap {
    ($($i:ident),* -> $o:ident) => {
        $(impl Cast<$o> for $i {
            fn cast(self) -> $o {
                self as $o
            }
        })*
    };
}

impl_wrap!(u128, i128, usize, isize, u64, i64, u32, i32, u16, i16, i8 -> u8);
impl_wrap!(u128, i128, usize, isize, u64, i64, u32, i32, u16, i16, u8 -> i8);
impl_wrap!(u128, i128, usize, isize, u64, i64, u32, i32, i16, i8 -> u16);
impl_wrap!(u128, i128, usize, isize, u64, i64, u32, i32, u16 -> i16);
impl_wrap!(u128, i128, usize, isize, u64, i64, i32, i16, i8 -> u32);
impl_wrap!(u128, i128, usize, isize, u64, i64, u32 -> i32);
impl_wrap!(u128, i128, usize, isize, i64, i32, i16, i8 -> u64);
impl_wrap!(u128, i128, usize, isize, u64 -> i64);
impl_wrap!(u128, i128, isize, u64, i64, i32, i16, i8 -> usize);
impl_wrap!(u128, i128, usize, u64, i64, i32 -> isize);
impl_wrap!(i128, isize, i64, i32, i16, i8 -> u128);
impl_wrap!(u128 -> i128);
