#[doc(hidden)]
#[macro_export]
macro_rules! impl_traits {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),* $(,)?])? =>
        $trait:ident $($next_traits:ident)*
    ) => {
        place_macro::place!( $crate::__ident__(impl_ __to_case__($trait))!(
            $t < $g > $([$(<$a $(,$p)?>),*])?
        ); );

        $crate::impl_traits!(
            $t < $g > $([$(<$a $(, $p)?>),*])? => $($next_traits)*
        );
    };

    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),* $(,)?])? => ) => { }
}

#[doc(hidden)]
#[macro_export]
macro_rules! second {
    ($a:tt, $b:tt) => {
        $b
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! Array {
    ($t:ident, $l:literal) => {
        [$t; $l]
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! Tuple {
    ($t:ident, ($($c:ident),*)) => {
        ($($crate::second!($c, $t)),*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_map_traits {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        $crate::impl_traits!($t < $g > $([$(<$a $(, $p)?>),*])? =>
            Cast Convert Scale
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_const_traits {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        $crate::impl_traits!($t < $g > $([$(<$a $(, $p)?>),*])? =>
            Infinity One Two Zero NegInfinity Limits NormalLimits
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_vec_traits {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        $crate::impl_traits!($t < $g > $([$(<$a $(, $p)?>),*])? =>
            MapTraits ConstTraits CAdd CAddAssign CSub CSubAssign SMul
            SMulAssign SDiv SDivAssign SRem SRemAssign CNeg CNot Mix
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_mix {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g> $t<$g> {
            /// Mix this and other instance in the ratio given by `amt`. `amt`
            /// determines how much of `other` will be mixed into `self`.
            ///
            /// This is equivalent to `(self * (1 - amt) + other * amt)` for
            /// each component to self and other.
            pub fn mix(self, amt: $g::Float, other: impl Into<Self>) -> Self
            where
                $g: $crate::ContainingFloat,
                $g::Float: std::ops::Mul<Output = $g::Float>
                    + std::ops::Add<Output = $g::Float>
                    + std::ops::Sub<Output = $g::Float>
                    + $crate::Cast<$g>
                    + $crate::One
                    + Copy,
            {
                $crate::CompArithm::cjoin(self, other, |a, b| {
                    let one = <$g::Float as $crate::One>::ONE;
                    let res = a.to_float() * (one - amt) + b.to_float() * amt;
                    $crate::Cast::<$g>::cast(res)
                })
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_c_add {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: std::ops::Add<O>, O> std::ops::Add<$t<O>> for $t<$g> {
            type Output = $t<$g::Output>;

            fn add(self, other: $t<O>) -> Self::Output {
                self.cjoin(other, |a, b| a + b)
            }
        }

        $($(impl<$g: Add<O>, O> Add<$a!(O $(, $p)?)> for $t<$g> {
            type Output = $t<$g::Output>;

            fn add(self, other: $a!(O $(, $p)?)) -> Self::Output {
                self.cjoin(other, |a, b| a + b)
            }
        })*)?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_c_add_assign {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g, O> std::ops::AddAssign<$t<O>> for $t<$g>
        where
            $g: std::ops::AddAssign<O>,
        {
            fn add_assign(&mut self, other: $t<O>) {
                self.cjoin_assign(other, |a, b| *a += b);
            }
        }

        $($(impl<$g: AddAssign<O>, O> AddAssign<$a!(O $(, $p)?)> for $t<$g> {
            fn add_assign(&mut self, other: $a!(O $(, $p)?)) {
                self.cjoin_assign(other, |a, b| *a += b)
            }
        })*)?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_add {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: std::ops::Add<O>, O> std::ops::Add<$t<O>> for $t<$g> {
            type Output = $t<$g::Output>;

            fn add(self, other: $t<O>) -> Self::Output {
                $t(self.0 + other.0)
            }
        }

        $($(impl<$g, O> std::ops::Add<$a!(O $(, $p)?)> for $t<$g>
            where
                $g: std::ops::Add<O>,
            {
            type Output = $t<$g::Output>;

            fn add(self, other: $a!(O $(, $p)?)) -> Self::Output {
                $t(self.0 + other.0.into())
            }
        })*)?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_add_assign {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g, O> std::ops::AddAssign<$t<O>> for $t<$g>
        where
            $g: std::ops::AddAssign<O>,
        {
            fn add_assign(&mut self, other: $t<O>) {
                self.0 += other.0
            }
        }

        $($(impl<$g: AddAssign<O>, O> AddAssign<$a!(O $(, $p)?)> for $t<$g> {
            fn add_assign(&mut self, other: $a!(O $(, $p)?)) {
                let o: $g<O> = other.into();
                self.0 += o.0;
            }
        })*)?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_c_sub {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: std::ops::Sub<O>, O> std::ops::Sub<$t<O>> for $t<$g> {
            type Output = $t<$g::Output>;

            fn sub(self, other: $t<O>) -> Self::Output {
                self.cjoin(other, |a, b| a - b)
            }
        }

        $($(impl<$g: Sub<O>, O> Sub<$a!(O $(, $p)?)> for $t<$g> {
            type Output = $t<$g::Output>;

            fn sub(self, other: $a!(O $(, $p)?)) -> Self::Output {
                self.cjoin(other, |a, b| a - b)
            }
        })*)?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_c_sub_assign {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g, O> std::ops::SubAssign<$t<O>> for $t<$g>
        where
            $g: std::ops::SubAssign<O>,
        {
            fn sub_assign(&mut self, other: $t<O>) {
                self.cjoin_assign(other, |a, b| *a -= b);
            }
        }

        $($(impl<$g: SubAssign<O>, O> SubAssign<$a!(O $(, $p)?)> for $t<$g> {
            fn sub_assign(&mut self, other: $a!(O $(, $p)?)) {
                self.cjoin_assign(other, |a, b| *a -= b)
            }
        })*)?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_s_mul {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: Mul<O>, O: Copy> Mul<O> for $t<$g> {
            type Output = $t<$g::Output>;

            fn mul(self, other: O) -> Self::Output {
                self.map(|a| a * other)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_s_mul_assign {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: std::ops::MulAssign<O>, O: Copy> std::ops::MulAssign<O>
            for $t<$g>
        {
            fn mul_assign(&mut self, other: O) {
                self.mutate(|a| *a *= other);
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_s_div {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: std::ops::Div<O>, O: Copy> std::ops::Div<O> for $t<$g> {
            type Output = $t<$g::Output>;

            fn div(self, other: O) -> Self::Output {
                self.map(|a| a / other)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_s_div_assign {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: std::ops::DivAssign<O>, O: Copy> std::ops::DivAssign<O>
            for $t<$g>
        {
            fn div_assign(&mut self, other: O) {
                self.mutate(|a| *a /= other);
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_s_rem {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: std::ops::Rem<O>, O: Copy> std::ops::Rem<O> for $t<$g> {
            type Output = $t<$g::Output>;

            fn rem(self, other: O) -> Self::Output {
                self.map(|a| a % other)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_s_rem_assign {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: std::ops::RemAssign<O>, O: Copy> std::ops::RemAssign<O>
            for $t<$g>
        {
            fn rem_assign(&mut self, other: O) {
                self.mutate(|a| *a %= other);
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_c_neg {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: std::ops::Neg> std::ops::Neg for $t<$g> {
            type Output = $t<$g::Output>;

            fn neg(self) -> Self::Output {
                self.map(|a| -a)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_c_not {
    ($t:ident < $g:ident > $([$(<$a:ident $(, $p:tt)?>),*])?) => {
        impl<$g: std::ops::Not> std::ops::Not for $t<$g> {
            type Output = $t<$g::Output>;

            fn not(self) -> Self::Output {
                self.map(|a| !a)
            }
        }
    };
}
