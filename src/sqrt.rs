use crate::ContainingFloat;

/// Calculate the floating point square root.
pub trait Sqrt {
    type Output;

    /// Calculate the floating point square root.
    fn sqrt(self) -> Self::Output;
}

impl<T> Sqrt for T
where
    T: ContainingFloat,
    T::Float: Sqrt,
{
    type Output = <T::Float as Sqrt>::Output;

    fn sqrt(self) -> Self::Output {
        self.to_float().sqrt()
    }
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

impl_float_sqrt!(f32, f64);
