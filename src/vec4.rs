use std::{
    fmt::Display,
    ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Not, Sub, SubAssign},
};

use crate::{Array, CompArithm, MapExt, RectExt, Tuple, Vec2, impl_traits};

/// Four dimensional vector or any 4-tuple-like object (e.g. rectangle).
#[derive(
    Debug, Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct Vec4<T = usize> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T> {
    /// Construct new Vec4.
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Set all components to the same value.
    pub const fn same_components(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v, v, v)
    }

    /// Width of rectangle represented by this vector.
    pub fn width_mut(&mut self) -> &mut T {
        &mut self.z
    }

    /// Height of rectangle represented by this vector.
    pub fn height_mut(&mut self) -> &mut T {
        &mut self.w
    }

    /// Width of rectangle represented by this vector.
    pub fn set_width(&mut self, width: T) {
        self.z = width;
    }

    /// Height of rectangle represented by this vector.
    pub fn set_height(&mut self, height: T) {
        self.w = height;
    }

    /// Get the first two components.
    pub const fn xy(&self) -> Vec2<T>
    where
        T: Copy,
    {
        Vec2::new(self.x, self.y)
    }

    /// Get the last two components.
    pub fn zw(&self) -> Vec2<T>
    where
        T: Copy,
    {
        (self.z, self.w).into()
    }

    /// Split the vector.
    pub fn xy_zw(self) -> (Vec2<T>, Vec2<T>) {
        ((self.x, self.y).into(), (self.z, self.w).into())
    }

    /// Get vector of references.
    pub fn as_ref(&self) -> Vec4<&T> {
        (&self.x, &self.y, &self.z, &self.w).into()
    }

    /// Get vector of mutable references.
    pub fn as_mut(&mut self) -> Vec4<&mut T> {
        (&mut self.x, &mut self.y, &mut self.z, &mut self.w).into()
    }

    /// Iterate the vector.
    pub fn iter(&self) -> std::array::IntoIter<&T, 4> {
        let r: [_; 4] = self.as_ref().into();
        r.into_iter()
    }

    /// Iterate the vector as mutable.
    pub fn iter_mut(&mut self) -> std::array::IntoIter<&mut T, 4> {
        let r: [_; 4] = self.as_mut().into();
        r.into_iter()
    }

    /// Convert the vector into an array.
    pub fn into_array(self) -> [T; 4] {
        self.into()
    }
}

impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Self { x, y, z, w }
    }
}

impl<T> From<[T; 4]> for Vec4<T> {
    fn from([x, y, z, w]: [T; 4]) -> Self {
        Self { x, y, z, w }
    }
}

impl<T> From<(Vec2<T>, Vec2<T>)> for Vec4<T> {
    fn from((a, b): (Vec2<T>, Vec2<T>)) -> Self {
        Self::new(a.x, a.y, b.x, b.y)
    }
}

impl<T> From<Vec4<T>> for [T; 4] {
    fn from(value: Vec4<T>) -> Self {
        [value.x, value.y, value.z, value.w]
    }
}

impl<T> From<(T, T, Vec2<T>)> for Vec4<T> {
    fn from((x, y, zw): (T, T, Vec2<T>)) -> Self {
        Self::new(x, y, zw.x, zw.y)
    }
}

impl<T> From<(Vec2<T>, T, T)> for Vec4<T> {
    fn from((xy, z, w): (Vec2<T>, T, T)) -> Self {
        Self::new(xy.x, xy.y, z, w)
    }
}

impl<T> RectExt for Vec4<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    type Val = T;

    fn from_pos_size(
        pos: impl Into<Vec2<Self::Val>>,
        size: impl Into<Vec2<Self::Val>>,
    ) -> Self {
        let pos = pos.into();
        let size = size.into();
        Self::new(pos.x, pos.y, size.x, size.y)
    }

    fn width(&self) -> Self::Val {
        self.z
    }

    fn height(&self) -> Self::Val {
        self.w
    }

    fn x(&self) -> Self::Val {
        self.x
    }

    fn y(&self) -> Self::Val {
        self.y
    }

    fn set_width(&mut self, w: Self::Val) {
        self.z = w;
    }

    fn set_height(&mut self, h: Self::Val) {
        self.w = h;
    }

    fn set_x(&mut self, x: Self::Val) {
        self.x = x;
    }

    fn set_y(&mut self, y: Self::Val) {
        self.y = y;
    }
}

impl<T> Index<usize> for Vec4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            4 => &self.w,
            _ => panic!("Inxe `{index}` is out of bounds for Vec3."),
        }
    }
}

impl<T> IndexMut<usize> for Vec4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Inxe `{index}` is out of bounds for Vec3."),
        }
    }
}

impl<Left, Right> PartialEq<(Right, Right, Right, Right)> for Vec4<Left>
where
    Left: PartialEq<Right>,
{
    fn eq(&self, (x, y, z, w): &(Right, Right, Right, Right)) -> bool {
        self.x == *x && self.y == *y && self.z == *z && self.w == *w
    }
}

impl<Left, Right> PartialEq<[Right; 4]> for Vec4<Left>
where
    Left: PartialEq<Right>,
{
    fn eq(&self, [x, y, z, w]: &[Right; 4]) -> bool {
        self.x == *x && self.y == *y && self.z == *z && self.w == *w
    }
}

impl<T: Display> Display for Vec4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl<T> IntoIterator for Vec4<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        let r: [_; 4] = self.into();
        r.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Vec4<T> {
    type Item = &'a T;
    type IntoIter = std::array::IntoIter<&'a T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec4<T> {
    type Item = &'a mut T;
    type IntoIter = std::array::IntoIter<&'a mut T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> MapExt for Vec4<T> {
    type Val = T;
    type This<R> = Vec4<R>;

    fn map<R>(self, mut f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        Vec4::new(f(self.x), f(self.y), f(self.z), f(self.w))
    }

    fn mutate(&mut self, mut f: impl FnMut(&mut Self::Val)) {
        f(&mut self.x);
        f(&mut self.y);
        f(&mut self.z);
        f(&mut self.w);
    }
}

impl<T: Default> FromIterator<T> for Vec4<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut i = iter.into_iter();
        Self::new(
            i.next().unwrap_or_default(),
            i.next().unwrap_or_default(),
            i.next().unwrap_or_default(),
            i.next().unwrap_or_default(),
        )
    }
}

impl<'a, T: Default + Copy> FromIterator<&'a T> for Vec4<T> {
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let mut i = iter.into_iter().copied();
        Self::new(
            i.next().unwrap_or_default(),
            i.next().unwrap_or_default(),
            i.next().unwrap_or_default(),
            i.next().unwrap_or_default(),
        )
    }
}

impl_traits!(Vec4<T> [<Array, 4>, <Tuple, (x, y, z, w)>] => VecTraits);
