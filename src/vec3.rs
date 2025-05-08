use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg,
        Rem, RemAssign, Sub, SubAssign,
    },
};

use crate::{Isqrt, Sqrt};

/// Represents three dimensional vector. Can be also use as color or any
/// 3-tuple-like object where vector operations are benefit.
///
/// It is ment to be as convinient as possible to work with in many use cases.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Vec3<T = usize> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Converts vector reference to vector of reference.
    pub fn as_ref(&self) -> Vec3<&T> {
        (&self.x, &self.y, &self.z).into()
    }

    /// Construct vector of mutable references to the components of the vector.
    pub fn as_mut(&mut self) -> Vec3<&mut T> {
        (&mut self.x, &mut self.y, &mut self.z).into()
    }

    /// Iterate over the three components.
    pub fn iter(&self) -> std::array::IntoIter<&T, 3> {
        let r: [_; 3] = self.as_ref().into();
        r.into_iter()
    }

    /// Get mutable iterator over the three components.
    pub fn iter_mut(&mut self) -> std::array::IntoIter<&mut T, 3> {
        let r: [_; 3] = self.as_mut().into();
        r.into_iter()
    }

    /// Map the individual components.
    pub fn map<R>(self, mut f: impl FnMut(T) -> R) -> Vec3<R> {
        (f(self.x), f(self.y), f(self.z)).into()
    }

    /// Get the red value. Alias to the first coordinate (x, [0]).
    pub fn r(&self) -> &T {
        &self.x
    }

    /// Get the green value. Alias to the second coordinate (y, [1]).
    pub fn g(&self) -> &T {
        &self.y
    }

    /// Get the blue value. Alias to the third coordinate (y, [2]).
    pub fn b(&self) -> &T {
        &self.z
    }

    /// Get the red value. Alias to the first coordinate (x, [0]).
    pub fn r_mut(&mut self) -> &mut T {
        &mut self.x
    }

    /// Get the green value. Alias to the second coordinate (y, [1]).
    pub fn g_mut(&mut self) -> &mut T {
        &mut self.y
    }

    /// Get the blue value. Alias to the third coordinate (y, [2]).
    pub fn b_mut(&mut self) -> &mut T {
        &mut self.z
    }

    /// Set the red value. Alias to the first coordinate (x, [0]).
    pub fn set_r(&mut self, r: T) {
        self.x = r;
    }

    /// Set the green value. Alias to the first coordinate (y, [1]).
    pub fn set_g(&mut self, g: T) {
        self.x = g;
    }

    /// Set the blue value. Alias to the first coordinate (z, [2]).
    pub fn set_b(&mut self, b: T) {
        self.x = b;
    }

    /// Calculate the dot product of two 3D vectors.
    pub fn dot<Right>(
        self,
        other: impl Into<Vec3<Right>>,
    ) -> <<T::Output as Add>::Output as Add<T::Output>>::Output
    where
        T: Mul<Right>,
        T::Output: Add,
        <T::Output as Add>::Output: Add<T::Output>,
    {
        let o = other.into();
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    /// Calculate the square of the length of the vector.
    pub fn sq_len(
        &self,
    ) -> <<T::Output as Add>::Output as Add<T::Output>>::Output
    where
        T: Copy + Mul,
        T::Output: Add,
        <T::Output as Add>::Output: Add<T::Output>,
    {
        self.dot(*self)
    }

    /// Calculate the length of the vector.
    pub fn len(
        &self,
    ) -> <<<T::Output as Add>::Output as Add<T::Output>>::Output as Sqrt>::Output
    where
        T: Copy + Mul,
        T::Output: Add<T::Output>,
        <T::Output as Add>::Output: Add<T::Output>,
        <<T::Output as Add>::Output as Add<T::Output>>::Output: Sqrt,
    {
        self.sq_len().sqrt()
    }

    /// Calculate the integer length of the vector.
    pub fn ilen(
        &self,
    ) -> <<T::Output as Add>::Output as Add<T::Output>>::Output
    where
        T: Copy + Mul,
        T::Output: Add<T::Output>,
        <T::Output as Add>::Output: Add<T::Output>,
        <<T::Output as Add>::Output as Add<T::Output>>::Output: Isqrt,
    {
        self.sq_len().isqrt()
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<[T; 3]> for Vec3<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<Vec3<T>> for (T, T, T) {
    fn from(value: Vec3<T>) -> Self {
        (value.x, value.y, value.z)
    }
}

impl<T> From<Vec3<T>> for [T; 3] {
    fn from(value: Vec3<T>) -> Self {
        [value.x, value.y, value.z]
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Inxe `{index}` is out of bounds for Vec3."),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Inxe `{index}` is out of bounds for Vec3."),
        }
    }
}

impl<Left, Right> PartialEq<(Right, Right, Right)> for Vec3<Left>
where
    Left: PartialEq<Right>,
{
    fn eq(&self, (x, y, z): &(Right, Right, Right)) -> bool {
        self.x == *x && self.y == *y && self.z == *z
    }
}

impl<Left, Right> PartialEq<[Right; 3]> for Vec3<Left>
where
    Left: PartialEq<Right>,
{
    fn eq(&self, [x, y, z]: &[Right; 3]) -> bool {
        self.x == *x && self.y == *y && self.z == *z
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<T> IntoIterator for Vec3<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        let r: [_; 3] = self.into();
        r.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Vec3<T> {
    type Item = &'a T;
    type IntoIter = std::array::IntoIter<&'a T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec3<T> {
    type Item = &'a mut T;
    type IntoIter = std::array::IntoIter<&'a mut T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: Neg> Neg for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn neg(self) -> Self::Output {
        self.map(|x| -x)
    }
}

macro_rules! op_single {
    ($op:ident, $fn:ident) => {
        impl<Left, Right> $op<Right> for Vec3<Left>
        where
            Left: $op<Right>,
            Right: Copy,
        {
            type Output = Vec3<Left::Output>;

            fn $fn(self, rhs: Right) -> Self::Output {
                self.map(|x| x.$fn(rhs))
            }
        }
    };
}

macro_rules! op_assign_single {
    ($op:ident, $fn:ident) => {
        impl<Left, Right> $op<Right> for Vec3<Left>
        where
            Left: $op<Right>,
            Right: Copy,
        {
            fn $fn(&mut self, rhs: Right) {
                self.x.$fn(rhs);
                self.y.$fn(rhs);
            }
        }
    };
}

macro_rules! op_triple {
    ($op:ident, $fn:ident) => {
        impl<Left, Right> $op<Vec3<Right>> for Vec3<Left>
        where
            Left: $op<Right>,
        {
            type Output = Vec3<Left::Output>;

            fn $fn(self, rhs: Vec3<Right>) -> Self::Output {
                (self.x.$fn(rhs.x), self.y.$fn(rhs.y), self.z.$fn(rhs.z))
                    .into()
            }
        }

        impl<Left, Right> $op<(Right, Right, Right)> for Vec3<Left>
        where
            Left: $op<Right>,
        {
            type Output = Vec3<Left::Output>;

            fn $fn(self, (x, y, z): (Right, Right, Right)) -> Self::Output {
                (self.x.$fn(x), self.y.$fn(y), self.z.$fn(z)).into()
            }
        }

        impl<Left, Right> $op<[Right; 3]> for Vec3<Left>
        where
            Left: $op<Right>,
        {
            type Output = Vec3<Left::Output>;

            fn $fn(self, [x, y, z]: [Right; 3]) -> Self::Output {
                (self.x.$fn(x), self.y.$fn(y), self.z.$fn(z)).into()
            }
        }
    };
}

macro_rules! op_assign_triple {
    ($op:ident, $fn:ident) => {
        impl<Left, Right> $op<Vec3<Right>> for Vec3<Left>
        where
            Left: $op<Right>,
        {
            fn $fn(&mut self, rhs: Vec3<Right>) {
                self.x.$fn(rhs.x);
                self.y.$fn(rhs.y);
                self.z.$fn(rhs.z);
            }
        }

        impl<Left, Right> $op<(Right, Right, Right)> for Vec3<Left>
        where
            Left: $op<Right>,
        {
            fn $fn(&mut self, (x, y, z): (Right, Right, Right)) {
                self.x.$fn(x);
                self.y.$fn(y);
                self.z.$fn(z);
            }
        }

        impl<Left, Right> $op<[Right; 3]> for Vec3<Left>
        where
            Left: $op<Right>,
        {
            fn $fn(&mut self, [x, y, z]: [Right; 3]) {
                self.x.$fn(x);
                self.y.$fn(y);
                self.z.$fn(z);
            }
        }
    };
}

op_single!(Mul, mul);
op_assign_single!(MulAssign, mul_assign);

op_single!(Div, div);
op_assign_single!(DivAssign, div_assign);

op_single!(Rem, rem);
op_assign_single!(RemAssign, rem_assign);

op_triple!(Add, add);
op_assign_triple!(AddAssign, add_assign);

op_triple!(Sub, sub);
op_assign_triple!(SubAssign, sub_assign);
