pub trait Float {}

macro_rules! impl_float {
    ($($t:ident),*) => {
        $(impl Float for $t {})*
    };
}

impl_float!(f32, f64);
