use crate::{ContainingFloat, Float};

pub trait IntoFloat {
    type Float: Float;

    fn into_float(self) -> Self::Float;
}

impl<T> IntoFloat for T
where
    T: ContainingFloat,
{
    type Float = <T as ContainingFloat>::Float;

    fn into_float(self) -> Self::Float {
        self.to_float()
    }
}

macro_rules! impl_float_into_float {
    ($($t:ident),*) => {
        $(impl IntoFloat for $t {
            type Float = $t;

            fn into_float(self) -> Self::Float {
                self
            }
        })*
    };
}

impl_float_into_float!(f32, f64);
