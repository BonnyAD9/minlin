pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

macro_rules! impl_int_sqrt {
    ($($i:ident),* -> $o:ident) => {
        $(impl Sqrt for $i {
            type Output = $o;

            fn sqrt(self) -> Self::Output {
                (self as $o).sqrt()
            }
        })*
    };
}

macro_rules! impl_float_sqrt {
    ($($i:ident),*) => {
        $(impl Sqrt for $i {
            type Output = Self;

            fn sqrt(self) -> Self::Output {
                $i::sqrt(self)
            }
        })*
    };
}

impl_int_sqrt!(u8, i8, u16, i16 -> f32);
impl_int_sqrt!(u32, i32, u64, i64, usize, isize, u128, i128 -> f64);

impl_float_sqrt!(f32, f64);
