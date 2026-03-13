use crate::ContainingFloat;

/// Trait that allows calculation of goniometric functions.
pub trait Goniometric {
    /// The output type of the operation. For integers, this is float.
    type Output;

    /// Calculate the sinus of the value. Result is in radians.
    fn sin(self) -> Self::Output;

    /// Calculate cosinus of the value. Result is in radians.
    fn cos(self) -> Self::Output;

    /// Calculate the atan(a/b) with correct sign.
    fn atan2(a: Self, b: Self) -> Self::Output;
}

impl<T> Goniometric for T
where
    T: ContainingFloat,
    T::Float: Goniometric,
{
    type Output = <T::Float as Goniometric>::Output;

    fn cos(self) -> Self::Output {
        self.to_float().cos()
    }

    fn sin(self) -> Self::Output {
        self.to_float().sin()
    }

    fn atan2(a: Self, b: Self) -> Self::Output {
        T::Float::atan2(a.to_float(), b.to_float())
    }
}

macro_rules! impl_float_goniometric {
    ($($t:ident),*) => {
        $(impl Goniometric for $t {
            type Output = $t;

            fn sin(self) -> Self::Output {
                $t::sin(self)
            }

            fn cos(self) -> Self::Output {
                $t::cos(self)
            }

            fn atan2(a: Self, b: Self) -> Self::Output {
                $t::atan2(a, b)
            }
        })*
    };
}

impl_float_goniometric!(f32, f64);
