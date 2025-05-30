use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg,
        Range, Rem, RemAssign, Sub, SubAssign,
    },
};

use crate::{
    Cast, Float, Goniometric, IntoFloat, Isqrt, LargeType, NormalLimits,
    Scale, Sqrt, Vec2RangeIter, Zero,
};

/// Represents two dimensional vector. Can be used as vector, point, size or
/// any tuple-like object where vector math operations are benefit.
///
/// It is meant to be as convinient as possible to work with in many use cases.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Vec2<T = usize> {
    /// The first coordinate of the vector (x, w, [0]).
    pub x: T,
    /// The second coordinate of the vector (y, h, [1])
    pub y: T,
}

impl<T> Vec2<T> {
    /// Creates new two dimensional vector from its components.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Get the width. Alias to the first coordinate (x, [0])
    pub fn w(&self) -> &T {
        &self.x
    }

    /// Get the height. Alias to the second coorinate (y, [1])
    pub fn h(&self) -> &T {
        &self.y
    }

    /// Get the width. Alias to the first coordinate (x, [0])
    pub fn w_mut(&mut self) -> &mut T {
        &mut self.x
    }

    /// Get the height. Alias to the second coorinate (y, [1])
    pub fn h_mut(&mut self) -> &mut T {
        &mut self.y
    }

    /// Set the width. Alias to the first coordinate (x, [0])
    pub fn set_w(&mut self, w: T) {
        self.x = w;
    }

    /// Set the height. Alias to the second coorinate (y, [1])
    pub fn set_h(&mut self, h: T) {
        self.y = h;
    }

    /// Gets the length (absolute value) of the vector squared.
    pub fn sq_len(&self) -> <T::Output as Add>::Output
    where
        T: Copy + Mul<T>,
        T::Output: Add<T::Output>,
    {
        self.dot(*self)
    }

    /// Gets fractional length (absolute value) of the vector.
    pub fn len(&self) -> <<T::Output as Add>::Output as Sqrt>::Output
    where
        T: Copy + Mul<T>,
        T::Output: Add<T::Output>,
        <T::Output as Add>::Output: Sqrt,
    {
        self.sq_len().sqrt()
    }

    /// Gets the integer length (absolute value) of the vector.
    pub fn ilen(&self) -> <T::Output as Add>::Output
    where
        T: Copy + Mul<T>,
        T::Output: Add<T::Output>,
        <T::Output as Add>::Output: Isqrt,
    {
        self.sq_len().isqrt()
    }

    /// Calculates the dot product of the two vectors.
    pub fn dot<Right>(
        self,
        other: impl Into<Vec2<Right>>,
    ) -> <T::Output as Add>::Output
    where
        T: Mul<Right>,
        T::Output: Add,
    {
        let o = other.into();
        self.x * o.x + self.y * o.y
    }

    /// Join the components of the two vetors with the given function.
    pub fn cjoin<R, O>(
        self,
        other: impl Into<Vec2<R>>,
        mut f: impl FnMut(T, R) -> O,
    ) -> Vec2<O> {
        let o = other.into();
        (f(self.x, o.x), f(self.y, o.y)).into()
    }

    /// Join the components of the vectors with the given function.
    pub fn cjoin_assign<R>(
        &mut self,
        other: impl Into<Vec2<R>>,
        mut f: impl FnMut(&mut T, R),
    ) {
        let o = other.into();
        f(&mut self.x, o.x);
        f(&mut self.y, o.y);
    }

    /// Componentwise multiplication.
    pub fn cmul<Right>(self, other: impl Into<Vec2<Right>>) -> Vec2<T::Output>
    where
        T: Mul<Right>,
    {
        let o = other.into();
        (self.x * o.x, self.y * o.y).into()
    }

    /// Componentwise mul-assign.
    pub fn cmul_assign<Right>(&mut self, other: impl Into<Vec2<Right>>)
    where
        T: MulAssign<Right>,
    {
        let o = other.into();
        self.x *= o.x;
        self.y *= o.y;
    }

    /// Componentwise division.
    pub fn cdiv<Right>(self, other: impl Into<Vec2<Right>>) -> Vec2<T::Output>
    where
        T: Div<Right>,
    {
        let o = other.into();
        (self.x / o.x, self.y / o.y).into()
    }

    /// Componentwise div-assign.
    pub fn cdiv_assign<Right>(&mut self, other: impl Into<Vec2<Right>>)
    where
        T: DivAssign<Right>,
    {
        let o = other.into();
        self.x /= o.x;
        self.y /= o.y;
    }

    /// Componentwise remainder.
    pub fn crem<Right>(self, other: impl Into<Vec2<Right>>) -> Vec2<T::Output>
    where
        T: Rem<Right>,
    {
        let o = other.into();
        (self.x % o.x, self.y % o.y).into()
    }

    /// Componentwise rem-assign.
    pub fn crem_assign<Right>(&mut self, other: impl Into<Vec2<Right>>)
    where
        T: RemAssign<Right>,
    {
        let o = other.into();
        self.x %= o.x;
        self.y %= o.y;
    }

    /// Sums the components.
    pub fn sum(self) -> T::Output
    where
        T: Add,
    {
        self.x + self.y
    }

    /// Subtracts the components.
    pub fn diff(self) -> T::Output
    where
        T: Sub,
    {
        self.x - self.y
    }

    /// Calculates the absolute difference of the components.
    pub fn abs_diff(self) -> T::Output
    where
        T: Sub + Ord,
    {
        if self.x < self.y {
            self.y - self.x
        } else {
            self.x - self.y
        }
    }

    /// Multiplies the components.
    pub fn prod(self) -> T::Output
    where
        T: Mul,
    {
        self.x * self.y
    }

    /// Divides the components.
    pub fn quot(self) -> T::Output
    where
        T: Div,
    {
        self.x / self.y
    }

    /// Gets the remainder of division of the components.
    pub fn quot_rem(self) -> T::Output
    where
        T: Rem,
    {
        self.x % self.y
    }

    /// Checks if the components are same.
    pub fn same(&self) -> bool
    where
        T: PartialEq,
    {
        self.x == self.y
    }

    /// Checks if the components are different.
    pub fn different(&self) -> bool
    where
        T: PartialEq,
    {
        self.x != self.y
    }

    /// Gets index of the max component.
    pub fn max_idx(&self) -> usize
    where
        T: Ord,
    {
        if self.y > self.x { 1 } else { 0 }
    }

    /// Get the larger of the two components. If both are same, x is returned.
    pub fn max(&self) -> &T
    where
        T: Ord,
    {
        if self.y > self.x { &self.y } else { &self.x }
    }

    /// Gets mutable reference to the largest element.
    pub fn max_mut(&mut self) -> &mut T
    where
        T: Ord,
    {
        if self.y > self.x {
            &mut self.y
        } else {
            &mut self.x
        }
    }

    /// Gets index to the smallest component.
    pub fn min_idx(&self) -> usize
    where
        T: Ord,
    {
        if self.y < self.x { 1 } else { 0 }
    }

    /// The the smaller of the two components. If both are same, x is returned.
    pub fn min(&self) -> &T
    where
        T: Ord,
    {
        if self.y < self.x { &self.y } else { &self.x }
    }

    /// The the smaller of the two components. If both are same, x is returned.
    pub fn min_mut(&mut self) -> &mut T
    where
        T: Ord,
    {
        if self.y < self.x {
            &mut self.y
        } else {
            &mut self.x
        }
    }

    /// Checks if value `v` is in the i-e range `self.x..self.y`.
    pub fn in_range(&self, v: &T) -> bool
    where
        T: Ord,
    {
        v >= &self.x && v < &self.y
    }

    /// Maps the individual components.
    pub fn map<R>(self, mut f: impl FnMut(T) -> R) -> Vec2<R> {
        (f(self.x), f(self.y)).into()
    }

    /// Converts vector reference to vector of reference.
    pub fn as_ref(&self) -> Vec2<&T> {
        (&self.x, &self.y).into()
    }

    /// Construct vector of mutable references to the components of the vector.
    pub fn as_mut(&mut self) -> Vec2<&mut T> {
        (&mut self.x, &mut self.y).into()
    }

    /// Checks if both of the components satisfy the condition.
    pub fn are_both(&self, mut f: impl FnMut(&T) -> bool) -> bool {
        f(&self.x) && f(&self.y)
    }

    /// Checks if any of the components satisfies the condition.
    pub fn is_any(&self, mut f: impl FnMut(&T) -> bool) -> bool {
        f(&self.x) || f(&self.y)
    }

    /// Checks if exactly one of the components satisfies the given condition.
    pub fn is_one(&self, f: impl FnMut(&T) -> bool) -> bool {
        self.as_ref().map(f).one()
    }

    /// Checks if no component satisfies the condition.
    pub fn is_none(&self, f: impl FnMut(&T) -> bool) -> bool {
        !self.is_any(f)
    }

    /// Iterate over the two components.
    pub fn iter(&self) -> std::array::IntoIter<&T, 2> {
        let r: [_; 2] = self.as_ref().into();
        r.into_iter()
    }

    /// Get mutable iterator over the two components.
    pub fn iter_mut(&mut self) -> std::array::IntoIter<&mut T, 2> {
        let r: [_; 2] = self.as_mut().into();
        r.into_iter()
    }

    /// Maps the components of the vector to the given type.
    pub fn convert<T2>(self) -> Vec2<T2>
    where
        T: Into<T2>,
    {
        self.map(|a| a.into())
    }

    /// Swaps the two components.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }

    /// Swaps the two components.
    pub fn swapped(self) -> Self {
        (self.y, self.x).into()
    }

    /// Swaps the two components.
    pub fn yx(self) -> Self {
        self.swapped()
    }

    /// Identity
    pub fn xy(self) -> Self {
        self
    }

    /// Sort the components.
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        if self.x > self.y {
            self.swap();
        }
    }

    /// Sort the components.
    pub fn sorted(mut self) -> Self
    where
        T: Ord,
    {
        self.sort();
        self
    }

    /// Clamp the value to the range in this vector.
    pub fn clamp<'a>(&'a self, v: &'a T) -> &'a T
    where
        T: Ord,
    {
        let s = self.as_ref().sorted();
        if v < s.x {
            s.x
        } else if v > s.y {
            s.y
        } else {
            v
        }
    }

    /// Clamp the value to the range in this vector.
    pub fn clamped(mut self, v: T) -> T
    where
        T: Ord,
    {
        self.sort();
        if v < self.x {
            self.x
        } else if v > self.y {
            self.y
        } else {
            v
        }
    }

    /// Cast components to a smaller type and ignore overflows.
    pub fn cast<O>(self) -> Vec2<O>
    where
        T: Cast<O>,
    {
        self.map(|a| a.cast())
    }

    /// Get 2D position in 2D space with the size of self represented by 1D
    /// container from index into the 1D container.
    ///
    /// This is inverse opration to [`Self::idx_of_pos`].
    ///
    /// E.g. if we have [`Vec`] representing 2D space with dimesions given in
    /// this [`Vec2`], we can give index into the [`Vec`], and this will return
    /// position of that element within the 2D space of size given by this
    /// [`Vec2`].
    pub fn pos_of_idx<I, R>(self, i: I) -> Vec2<R>
    where
        T: Copy,
        I: Copy + Rem<T, Output = R> + Div<T, Output = R>,
    {
        (i % self.y, i / self.y).into()
    }

    /// Get index corresponding to pos to 1D container that represents 2D space
    /// with size of this.
    ///
    /// This is inverse opration to [`Self::pos_of_idx`].
    ///
    /// E.g. if we have [`Vec`] representing 2D space with dimentions given in
    /// this [`Vec2`], we give position within the 2D space, and this will
    /// return index into the [`Vec`].
    pub fn idx_of_pos<R>(
        self,
        pos: impl Into<Vec2<R>>,
    ) -> <T::Output as Add<R>>::Output
    where
        T: Mul<R>,
        T::Output: Add<R>,
    {
        let pos = pos.into();
        self.x * pos.y + pos.x
    }

    /// Get the angle of the vector.
    pub fn angle(self) -> T::Output
    where
        T: Goniometric,
    {
        T::atan2(self.y, self.x)
    }

    /// Calculate the polar coordinates of the vector.
    pub fn polar(
        self,
    ) -> (
        <<<T as Mul>::Output as Add>::Output as Sqrt>::Output,
        <T as Goniometric>::Output,
    )
    where
        T: Copy + Mul + Goniometric,
        <T as Mul>::Output: Add,
        <<T as Mul>::Output as Add>::Output: Sqrt,
    {
        (self.len(), self.angle())
    }

    /// Gets normalized version of the vector as float.
    #[allow(clippy::type_complexity)]
    pub fn normalized(
        self,
    ) -> Vec2<
        <T::Float as Div<
            <<<T::Float as Mul>::Output as Add>::Output as Sqrt>::Output,
        >>::Output,
    >
    where
        T: IntoFloat,
        T::Float: Copy + Mul,
        <T::Float as Mul>::Output: Add<<T::Float as Mul>::Output>,
        <<T::Float as Mul>::Output as Add>::Output: Sqrt,
        <<<T::Float as Mul>::Output as Add>::Output as Sqrt>::Output: Copy,
        T::Float:
            Div<<<<T::Float as Mul>::Output as Add>::Output as Sqrt>::Output>,
    {
        let v = self.map(|a| a.into_float());
        let len = v.len();
        (v.x / len, v.y / len).into()
    }

    /// Normalizes this vector.
    pub fn normalize(&mut self)
    where
        T: Copy + Float + Mul,
        T::Output: Add<T::Output>,
        <T::Output as Add>::Output: Sqrt,
        <<T::Output as Add>::Output as Sqrt>::Output: Copy,
        T: DivAssign<<<T::Output as Add>::Output as Sqrt>::Output>,
    {
        *self /= self.len();
    }

    /// Craetes range from this vector to the other vector.
    pub fn to(self, other: impl Into<Vec2<T>>) -> Vec2RangeIter<T>
    where
        T: Copy,
    {
        Vec2RangeIter::new(self, other.into())
    }

    /// Creates vector from polar coordinates.
    pub fn from_polar<L, A>(length: L, angle: A) -> Self
    where
        A: Copy + Float + Goniometric,
        A::Output: Mul<L, Output = T>,
        L: Copy,
    {
        Vec2::new(angle.cos(), angle.sin()) * length
    }

    /// Check if 2D space of this size contains pos.
    pub fn contains(&self, pos: impl Into<Vec2<T>>) -> bool
    where
        T: PartialOrd + Zero,
    {
        let pos = pos.into();
        pos.are_both(|a| *a >= T::ZERO) && pos.x < self.x && pos.y < self.y
    }

    /// Scales the vector to different type. Floating point types are in range
    /// 0..=1 where integer types are in range MIN..=MAX.
    pub fn scale<S>(self) -> Vec2<S>
    where
        T: Scale<S>,
    {
        (self.x.scale(), self.y.scale()).into()
    }

    /// Change the range of values from `ss..=se` to `ds..=de`.
    pub fn change_range(self, ss: T, se: T, ds: T, de: T) -> Vec2<T>
    where
        T: LargeType + Copy + Sub<Output = T>,
        T::Large: Add<Output = T::Large>
            + Sub<Output = T::Large>
            + Mul<Output = T::Large>
            + Div<Output = T::Large>,
    {
        self.map(|a| {
            T::from_large(
                (a.to_large() - ss.to_large()) * (de - ds).to_large()
                    / (se - ss).to_large()
                    + ds.to_large(),
            )
        })
    }

    /// Transform values from normal range to the given range.
    pub fn norm_to_range(self, s: T, e: T) -> Vec2<T>
    where
        T: LargeType + Copy + NormalLimits + Sub<Output = T>,
        T::Large: Add<Output = T::Large>
            + Sub<Output = T::Large>
            + Mul<Output = T::Large>
            + Div<Output = T::Large>,
    {
        self.change_range(T::NORM_MIN, T::NORM_MAX, s, e)
    }

    /// Transform values from the given range to normal range.
    pub fn to_norm_range(self, s: T, e: T) -> Vec2<T>
    where
        T: LargeType + Copy + NormalLimits + Sub<Output = T>,
        T::Large: Add<Output = T::Large>
            + Sub<Output = T::Large>
            + Mul<Output = T::Large>
            + Div<Output = T::Large>,
    {
        self.change_range(s, e, T::NORM_MIN, T::NORM_MAX)
    }

    /// Calculate the absolute value of each component.
    pub fn cabs(self) -> Vec2<T>
    where
        T: PartialOrd + Zero + Neg<Output = T>,
    {
        self.map(|a| if a < T::ZERO { -a } else { a })
    }
}

impl Vec2<bool> {
    /// Checks if both are true.
    pub fn both(self) -> bool {
        self.x & self.y
    }

    /// Checks if any of the components is true.
    pub fn any(self) -> bool {
        self.x | self.y
    }

    /// Returns true if only one of the components is true.
    pub fn one(self) -> bool {
        self.different()
    }

    /// Checks if bot of the components are false.
    pub fn none(self) -> bool {
        !self.any()
    }
}

impl<T> Vec2<&T> {
    /// Clones the values in references of the vector.
    pub fn cloned(self) -> Vec2<T>
    where
        T: Clone,
    {
        (self.x.clone(), self.y.clone()).into()
    }

    /// Copies the values in references of the vector.
    pub fn copied(self) -> Vec2<T>
    where
        T: Copy,
    {
        (*self.x, *self.y).into()
    }
}

impl<T: Zero> Vec2<T> {
    /// 2D vectors with all components set to zero.
    pub const ZERO: Vec2<T> = Vec2 {
        x: T::ZERO,
        y: T::ZERO,
    };
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y }
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(value: Vec2<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T> From<Vec2<T>> for [T; 2] {
    fn from(value: Vec2<T>) -> Self {
        [value.x, value.y]
    }
}

impl<T> From<Vec2<T>> for Range<T> {
    fn from(value: Vec2<T>) -> Self {
        value.x..value.y
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index `{index}` is out of bounds for Vec2."),
        }
    }
}

impl<T> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index `{index}` is out of bounds for Vec2."),
        }
    }
}

impl<Left, Right> PartialEq<(Right, Right)> for Vec2<Left>
where
    Left: PartialEq<Right>,
{
    fn eq(&self, (x, y): &(Right, Right)) -> bool {
        self.x == *x && self.y == *y
    }
}

impl<Left, Right> PartialEq<[Right; 2]> for Vec2<Left>
where
    Left: PartialEq<Right>,
{
    fn eq(&self, [x, y]: &[Right; 2]) -> bool {
        self.x == *x && self.y == *y
    }
}

impl<T> Display for Vec2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T> IntoIterator for Vec2<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 2>;

    fn into_iter(self) -> Self::IntoIter {
        let r: [_; 2] = self.into();
        r.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Vec2<T> {
    type Item = &'a T;
    type IntoIter = std::array::IntoIter<&'a T, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec2<T> {
    type Item = &'a mut T;
    type IntoIter = std::array::IntoIter<&'a mut T, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Neg for Vec2<T>
where
    T: Neg,
{
    type Output = Vec2<T::Output>;

    fn neg(self) -> Self::Output {
        (-self.x, -self.y).into()
    }
}

macro_rules! op_single {
    ($op:ident, $fn:ident) => {
        impl<Left, Right> $op<Right> for Vec2<Left>
        where
            Left: $op<Right>,
            Right: Copy,
        {
            type Output = Vec2<Left::Output>;

            fn $fn(self, rhs: Right) -> Self::Output {
                (self.x.$fn(rhs), self.y.$fn(rhs)).into()
            }
        }
    };
}

macro_rules! op_assign_single {
    ($op:ident, $fn:ident) => {
        impl<Left, Right> $op<Right> for Vec2<Left>
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

macro_rules! op_double {
    ($op:ident, $fn:ident) => {
        impl<Left, Right> $op<Vec2<Right>> for Vec2<Left>
        where
            Left: $op<Right>,
        {
            type Output = Vec2<Left::Output>;

            fn $fn(self, rhs: Vec2<Right>) -> Self::Output {
                (self.x.$fn(rhs.x), self.y.$fn(rhs.y)).into()
            }
        }

        impl<Left, Right> $op<(Right, Right)> for Vec2<Left>
        where
            Left: $op<Right>,
        {
            type Output = Vec2<Left::Output>;

            fn $fn(self, (x, y): (Right, Right)) -> Self::Output {
                (self.x.$fn(x), self.y.$fn(y)).into()
            }
        }

        impl<Left, Right> $op<[Right; 2]> for Vec2<Left>
        where
            Left: $op<Right>,
        {
            type Output = Vec2<Left::Output>;

            fn $fn(self, [x, y]: [Right; 2]) -> Self::Output {
                (self.x.$fn(x), self.y.$fn(y)).into()
            }
        }

        impl<Left, Right> $op<Range<Right>> for Vec2<Left>
        where
            Left: $op<Right>,
        {
            type Output = Vec2<Left::Output>;

            fn $fn(self, rhs: Range<Right>) -> Self::Output {
                (self.x.$fn(rhs.start), self.y.$fn(rhs.end)).into()
            }
        }
    };
}

macro_rules! op_assign_double {
    ($op:ident, $fn:ident) => {
        impl<Left, Right> $op<Vec2<Right>> for Vec2<Left>
        where
            Left: $op<Right>,
        {
            fn $fn(&mut self, rhs: Vec2<Right>) {
                self.x.$fn(rhs.x);
                self.y.$fn(rhs.y);
            }
        }

        impl<Left, Right> $op<(Right, Right)> for Vec2<Left>
        where
            Left: $op<Right>,
        {
            fn $fn(&mut self, (x, y): (Right, Right)) {
                self.x.$fn(x);
                self.y.$fn(y);
            }
        }

        impl<Left, Right> $op<[Right; 2]> for Vec2<Left>
        where
            Left: $op<Right>,
        {
            fn $fn(&mut self, [x, y]: [Right; 2]) {
                self.x.$fn(x);
                self.y.$fn(y);
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

op_double!(Add, add);
op_assign_double!(AddAssign, add_assign);

op_double!(Sub, sub);
op_assign_double!(SubAssign, sub_assign);
