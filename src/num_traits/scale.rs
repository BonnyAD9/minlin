pub trait Scale<T> {
    /// Scale this type to the given other type.
    fn scale(self) -> T;
}

impl<T> Scale<T> for T {
    fn scale(self) -> T {
        self
    }
}

macro_rules! impl_scale_int {
    ($($l:ident),* -> $s:ident) => {
        $(
            impl Scale<$s> for $l {
                fn scale(self) -> $s {
                    (self >> ($l::BITS - $s::BITS)) as $s
                }
            }

            impl Scale<$l> for $s {
                fn scale(self) -> $l {
                    let mut res = (self as $l) << ($l::BITS - $s::BITS);
                    let mut shift = $s::BITS;
                    while shift < $l::BITS {
                        res |= res >> shift;
                        shift *= 2;
                    }
                    res
                }
            }
        )*
    };
}

macro_rules! impl_scale_int_float {
    ($($i:ident),* -> $f:ident) => {
        $(
            impl Scale<$f> for $i {
                fn scale(self) -> $f {
                    self as $f / $i::MAX as $f
                }
            }

            impl Scale<$i> for $f {
                fn scale(self) -> $i {
                    (self * $i::MAX as $f) as $i
                }
            }
        )*
    };
}

macro_rules! impl_scale_float {
    ($($l:ident),* -> $s:ident) => {
        $(
            impl Scale<$s> for $l {
                fn scale(self) -> $s {
                    self as $s
                }
            }

            impl Scale<$l> for $s {
                fn scale(self) -> $l {
                    self as $l
                }
            }
        )*
    };
}

macro_rules! impl_scale_unsigned_signed {
    ($($u:ident),* -> $i:ident = $h:ident) => {
        $(
            impl Scale<$i> for $u {
                fn scale(self) -> $i {
                    <$u as Scale<$h>>::scale(self)
                        .wrapping_add($i::MIN as $h) as $i
                }
            }

            impl Scale<$u> for $i {
                fn scale(self) -> $u {
                    (self.wrapping_sub($i::MIN) as $h).scale()
                }
            }
        )*
    };
}

impl_scale_int!(u128, u64, u32, u16 -> u8);
impl_scale_int!(u128, u64, u32 -> u16);
impl_scale_int!(u128, u64 -> u32);
impl_scale_int!(u128 -> u64);

impl_scale_int_float!(u128, u64, u32, u16, u8 -> f64);
impl_scale_int_float!(u128, u64, u32, u16, u8 -> f32);

impl_scale_float!(f64 -> f32);

impl_scale_unsigned_signed!(
    u128, u64, f64, u32, f32, u16, u8, i128, i64, i32, i16 -> i8 = u8
);
impl_scale_unsigned_signed!(
    u128, u64, f64, u32, f32, u16, u8, i128, i64, i32 -> i16 = u16
);
impl_scale_unsigned_signed!(
    u128, u64, f64, u32, f32, u16, u8, i128, i64 -> i32 = u32
);
impl_scale_unsigned_signed!(
    u128, u64, f64, u32, f32, u16, u8, i128 -> i64 = u64
);
impl_scale_unsigned_signed!(
    u128, u64, f64, u32, f32, u16, u8 -> i128 = u128
);

//impl_scale_signed!(i128 = u128, i64 = u64, i32 = u32, i16 = u16, i8 = u8);
