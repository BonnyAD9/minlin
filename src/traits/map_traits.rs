#[macro_export]
macro_rules! impl_traits {
    ($t:ident < $g:ident > => $trait:ident $($next_traits:ident)*) => {
        place_macro::place!( $crate::__ident__(impl_ __to_case__($trait))!($t < $g >); );

        impl_traits!($t < $g > => $($next_traits)*);
    };

    ($t:ident < $g:ident > => ) => {

    }
}

#[macro_export]
macro_rules! impl_map_traits {
    ($t:ident < $g:ident >) => {
        $crate::impl_traits!($t < $g > => Cast Convert Scale);
    };
}

#[macro_export]
macro_rules! impl_const_traits {
    ($t:ident < $g:ident >) => {
        $crate::impl_traits!($t < $g > => Infinity One Two Zero NegInfinity);
    };
}

#[macro_export]
macro_rules! impl_vec_traits {
    ($t:ident < $g:ident >) => {
        $crate::impl_traits!($t < $g > => MapTraits ConstTraits);
    };
}
