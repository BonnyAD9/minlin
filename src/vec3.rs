use std::{
    fmt::Display,
    mem,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg,
        Rem, RemAssign, Sub, SubAssign,
    },
};

use crate::{
    Cast, Float, Goniometric, IntoFloat, Isqrt, LargeType, NormalLimits,
    Scale, Sqrt, Vec2, Zero,
};

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

    /// Join the components of the two vectors with the given function.
    pub fn cjoin<R, O>(
        self,
        other: impl Into<Vec3<R>>,
        mut f: impl FnMut(T, R) -> O,
    ) -> Vec3<O> {
        let o = other.into();
        (f(self.x, o.x), f(self.y, o.y), f(self.z, o.z)).into()
    }

    /// Join the components of the vectors with the given function.
    pub fn cjoin_assign<R>(
        &mut self,
        other: impl Into<Vec3<R>>,
        mut f: impl FnMut(&mut T, R),
    ) {
        let o = other.into();
        f(&mut self.x, o.x);
        f(&mut self.y, o.y);
        f(&mut self.z, o.z);
    }

    /// Do componentwise multiplication.
    pub fn cmul<Right>(self, other: impl Into<Vec3<Right>>) -> Vec3<T::Output>
    where
        T: Mul<Right>,
    {
        self.cjoin(other, T::mul)
    }

    /// Componentwise multiplication in place.
    pub fn cmul_assign<R>(&mut self, other: impl Into<Vec3<R>>)
    where
        T: MulAssign<R>,
    {
        self.cjoin_assign(other, T::mul_assign);
    }

    /// Componentwise division.
    pub fn cdiv<R>(self, other: impl Into<Vec3<R>>) -> Vec3<T::Output>
    where
        T: Div<R>,
    {
        self.cjoin(other, T::div)
    }

    /// Componentwise division in place.
    pub fn cdiv_assign<R>(&mut self, other: impl Into<Vec3<R>>)
    where
        T: DivAssign<R>,
    {
        self.cjoin_assign(other, T::div_assign);
    }

    /// Componentwise remainder.
    pub fn crem<R>(self, other: impl Into<Vec3<R>>) -> Vec3<T::Output>
    where
        T: Rem<R>,
    {
        self.cjoin(other, T::rem)
    }

    /// Componentwise remainder in place.
    pub fn crem_assign<R>(&mut self, other: impl Into<Vec3<R>>)
    where
        T: RemAssign<R>,
    {
        self.cjoin_assign(other, T::rem_assign);
    }

    /// Sum all the components.
    pub fn sum(self) -> <T::Output as Add<T>>::Output
    where
        T: Add,
        T::Output: Add<T>,
    {
        self.x + self.y + self.z
    }

    /// Multiply all the components.
    pub fn prod(self) -> <T::Output as Mul<T>>::Output
    where
        T: Mul,
        T::Output: Mul<T>,
    {
        self.x * self.y * self.z
    }

    /// Checks if all the components are same.
    pub fn same(&self) -> bool
    where
        T: PartialEq,
    {
        self.x == self.y && self.x == self.z
    }

    /// Counts how many different values are in the components. Returns `1`,
    /// `2` or `3`.
    pub fn group_cnt(&self) -> usize
    where
        T: PartialEq,
    {
        if self.x == self.y {
            if self.x == self.z { 1 } else { 2 }
        } else if self.x == self.z || self.y == self.z {
            2
        } else {
            3
        }
    }

    /// Gets index to the largest of the components.
    pub fn max_idx(&self) -> usize
    where
        T: Ord,
    {
        if self.z > self.y {
            if self.z > self.x { 2 } else { 0 }
        } else if self.y > self.x {
            1
        } else {
            0
        }
    }

    /// Gets reference to the largest component.
    pub fn max(&self) -> &T
    where
        T: Ord,
    {
        &self[self.max_idx()]
    }

    /// Gets mutable reference to the largest component.
    pub fn max_mut(&mut self) -> &mut T
    where
        T: Ord,
    {
        let i = self.max_idx();
        &mut self[i]
    }

    /// Gets index to the smallest of the components.
    pub fn min_idx(&self) -> usize
    where
        T: Ord,
    {
        if self.z < self.y {
            if self.z < self.x { 2 } else { 0 }
        } else if self.y < self.x {
            1
        } else {
            0
        }
    }

    /// Gets reference to the smallest component.
    pub fn min(&self) -> &T
    where
        T: Ord,
    {
        &self[self.min_idx()]
    }

    /// Gets mutable reference to the smallest component.
    pub fn min_mut(&mut self) -> &mut T
    where
        T: Ord,
    {
        let i = self.min_idx();
        &mut self[i]
    }

    /// Gets index to the middle of the components.
    pub fn mid_idx(&self) -> usize
    where
        T: Ord,
    {
        if self.z < self.y {
            if self.z < self.x {
                if self.y < self.x { 1 } else { 0 }
            } else {
                2
            }
        } else if self.y < self.x {
            if self.z < self.x { 2 } else { 0 }
        } else {
            1
        }
    }

    /// Gets reference to the middle component.
    pub fn mid(&self) -> &T
    where
        T: Ord,
    {
        &self[self.mid_idx()]
    }

    /// Gets mutable reference to the middle component.
    pub fn mid_mut(&mut self) -> &mut T
    where
        T: Ord,
    {
        let i = self.mid_idx();
        &mut self[i]
    }

    /// Checks if all the components match the predicate.
    pub fn are_all(&self, mut f: impl FnMut(&T) -> bool) -> bool {
        f(&self.x) && f(&self.y) && f(&self.z)
    }

    /// Checks if any of the components match the predicate.
    pub fn is_any(&self, mut f: impl FnMut(&T) -> bool) -> bool {
        f(&self.x) || f(&self.y) || f(&self.z)
    }

    /// Checks if all the components don't match the predicate.
    pub fn is_none(&self, mut f: impl FnMut(&T) -> bool) -> bool {
        !f(&self.x) && !f(&self.y) && !f(&self.z)
    }

    /// Checks if at least one doesn't match the predicate.
    pub fn is_any_not(&self, mut f: impl FnMut(&T) -> bool) -> bool {
        !f(&self.x) || !f(&self.y) || !f(&self.z)
    }

    /// Counts how many components match the predicate.
    pub fn get_count(&self, mut f: impl FnMut(&T) -> bool) -> usize {
        f(&self.x) as usize + f(&self.y) as usize + f(&self.z) as usize
    }

    /// Map the components to the given type
    pub fn convert<T2>(self) -> Vec3<T2>
    where
        T: Into<T2>,
    {
        self.map(|a| a.into())
    }

    /// Identity.
    pub fn xyz(self) -> Self {
        self
    }

    /// Reorder the vector.
    pub fn xzy(self) -> Self {
        (self.x, self.z, self.y).into()
    }

    /// Reorder the vector.
    pub fn yxz(self) -> Self {
        (self.y, self.x, self.z).into()
    }

    /// Reorder the vector.
    pub fn yzx(self) -> Self {
        (self.y, self.z, self.x).into()
    }

    /// Reorder the vector.
    pub fn zxy(self) -> Self {
        (self.z, self.x, self.y).into()
    }

    /// Reorder the vector.
    pub fn zyx(self) -> Self {
        (self.z, self.y, self.x).into()
    }

    /// Split the vector.
    pub fn x_yz(self) -> (T, Vec2<T>) {
        (self.x, (self.y, self.z).into())
    }

    /// Split and reorder the vector.
    pub fn x_zy(self) -> (T, Vec2<T>) {
        (self.x, (self.z, self.y).into())
    }

    /// Split and reorder the vector.
    pub fn y_xz(self) -> (T, Vec2<T>) {
        (self.y, (self.x, self.z).into())
    }

    /// Split and reorder the vector.
    pub fn y_zx(self) -> (T, Vec2<T>) {
        (self.y, (self.z, self.x).into())
    }

    /// Split and reorder the vector.
    pub fn z_xy(self) -> (T, Vec2<T>) {
        (self.z, (self.x, self.y).into())
    }

    /// Split and reorder the vector.
    pub fn z_yx(self) -> (T, Vec2<T>) {
        (self.z, (self.y, self.x).into())
    }

    /// Split the vector.
    pub fn yz(self) -> Vec2<T> {
        (self.y, self.z).into()
    }

    /// Get two components of the vector.
    pub fn zy(self) -> Vec2<T> {
        (self.z, self.y).into()
    }

    /// Get two components of the vector.
    pub fn xz(self) -> (T, Vec2<T>) {
        (self.y, (self.x, self.z).into())
    }

    /// Get two components of the vector.
    pub fn zx(self) -> Vec2<T> {
        (self.z, self.x).into()
    }

    /// Get two components of the vector.
    pub fn xy(self) -> Vec2<T> {
        (self.x, self.y).into()
    }

    /// Get two components of the vector.
    pub fn yx(self) -> Vec2<T> {
        (self.y, self.x).into()
    }

    /// Creates sorted version of the vector.
    pub fn sorted(self) -> Self
    where
        T: Ord,
    {
        if self.z < self.y {
            if self.z < self.x {
                if self.y < self.x {
                    self.zyx()
                } else {
                    self.zxy()
                }
            } else {
                self.xzy()
            }
        } else if self.y < self.x {
            if self.z < self.x {
                self.yzx()
            } else {
                self.yxz()
            }
        } else {
            self.xyz()
        }
    }

    /// Sorts values in the vector.
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        if self.z < self.y {
            if self.z < self.x {
                mem::swap(&mut self.z, &mut self.x);
                if self.y >= self.x {
                    mem::swap(&mut self.z, &mut self.y);
                }
            } else {
                mem::swap(&mut self.z, &mut self.y);
            }
        } else if self.y < self.x {
            mem::swap(&mut self.y, &mut self.x);
            if self.z < self.x {
                mem::swap(&mut self.z, &mut self.y);
            }
        }
    }

    /// Cost components to a smaller type and ignore overflows.
    pub fn cast<O>(self) -> Vec3<O>
    where
        T: Cast<O>,
    {
        self.map(|a| a.cast())
    }

    /// Get 3D position in 3D space with the size of self represented by 1D
    /// container from index into the 1D container.
    ///
    /// This is inverse opration to [`Self::idx_of_pos`].
    ///
    /// E.g. if we have [`Vec`] representing 3D space with dimesions given in
    /// this [`Vec3`], we can give index into the [`Vec`], and this will return
    /// position of that element within the 3D space of size given by this
    /// [`Vec3`].
    pub fn pos_of_idx<I, R>(self, i: I) -> Vec3<R>
    where
        T: Copy + Mul,
        T::Output: Copy,
        I: Copy + Div<T::Output, Output = R> + Rem<T::Output>,
        <I as Rem<T::Output>>::Output:
            Copy + Rem<T, Output = R> + Div<T, Output = R>,
    {
        let xy = self.x * self.y;
        let i2 = i % xy;
        (i2 % self.y, i2 / self.y, i / xy).into()
    }

    /// Get index corresponding to pos to 1D container that represents 3D space
    /// with size of this.
    ///
    /// This is inverse opration to [`Self::pos_of_idx`].
    ///
    /// E.g. if we have [`Vec`] representing 3D space with dimentions given in
    /// this [`Vec3`], we give position within the 3D space, and this will
    /// return index into the [`Vec`].
    #[allow(clippy::type_complexity)]
    pub fn idx_of_pos<R>(
        self,
        pos: impl Into<Vec3<R>>
    ) -> <<<<T as Mul>::Output as Mul<R>>::Output as Add<
            <T as Mul<R>>::Output>
        >::Output as Add<R>>::Output
    where
        T: Copy + Mul + Mul<R>,
        <T as Mul>::Output: Mul<R>,
        <<T as Mul>::Output as Mul<R>>::Output: Add<<T as Mul<R>>::Output>,
        <<<T as Mul>::Output as Mul<R>>::Output as Add<
            <T as Mul<R>>::Output
        >>::Output: Add<R>,
    {
        let p = pos.into();
        self.x * self.y * p.z + self.x * p.y + p.x
    }

    /// Calculate vector in 2D plane on which the x coordinate stays the same.
    ///
    /// The x coordinate will be the x coordinate in the new vector.
    pub fn plane_x(self) -> (T, <<T::Output as Add>::Output as Sqrt>::Output)
    where
        T: Copy + Mul<T>,
        T::Output: Add<T::Output>,
        <T::Output as Add>::Output: Sqrt,
    {
        (self.x, Vec2::new(self.y, self.z).len())
    }

    /// Calculate vector in 2D plane on which the x coordinate stays the same.
    ///
    /// The y coordinate will be the x coordinate in the new vector.
    pub fn plane_y(self) -> (T, <<T::Output as Add>::Output as Sqrt>::Output)
    where
        T: Copy + Mul<T>,
        T::Output: Add<T::Output>,
        <T::Output as Add>::Output: Sqrt,
    {
        (self.y, Vec2::new(self.z, self.x).len())
    }

    /// Calculate vector in 2D plane on which the x coordinate stays the same.
    ///
    /// The z coordinate will be the y coordinate in the new vector.
    pub fn plane_z(self) -> (T, <<T::Output as Add>::Output as Sqrt>::Output)
    where
        T: Copy + Mul<T>,
        T::Output: Add<T::Output>,
        <T::Output as Add>::Output: Sqrt,
    {
        (self.z, Vec2::new(self.x, self.y).len())
    }

    /// Calculate angle to the X axis.
    pub fn angle_x(self) -> <T as Goniometric>::Output
    where
        T: Copy + Mul + Goniometric,
        <T as Mul>::Output: Add<<T as Mul>::Output>,
        <<T as Mul>::Output as Add>::Output: Sqrt<Output = T>,
    {
        Vec2::from(self.plane_x()).angle()
    }

    /// Calculate angle to the Y axis.
    pub fn angle_y(self) -> <T as Goniometric>::Output
    where
        T: Copy + Mul + Goniometric,
        <T as Mul>::Output: Add<<T as Mul>::Output>,
        <<T as Mul>::Output as Add>::Output: Sqrt<Output = T>,
    {
        Vec2::from(self.plane_y()).angle()
    }

    /// Calculate angle to the Z axis.
    pub fn angle_z(self) -> <T as Goniometric>::Output
    where
        T: Copy + Mul + Goniometric,
        <T as Mul>::Output: Add<<T as Mul>::Output>,
        <<T as Mul>::Output as Add>::Output: Sqrt<Output = T>,
    {
        Vec2::from(self.plane_z()).angle()
    }

    /// Calculate polar coordinates with X being the polar axis.
    #[allow(clippy::type_complexity)]
    pub fn polar_x(
        self
    ) -> (
            <<<<T as Mul>::Output as Add>::Output as Add<
                <T as Mul>::Output>
            >::Output as Sqrt>::Output,
            <T as Goniometric>::Output,
            <T as Goniometric>::Output,
        )
    where
        T: Copy + Mul + Goniometric,
        <T as Mul>::Output: Add<<T as Mul>::Output>,
        <<T as Mul>::Output as Add>::Output:
            Add<<T as Mul>::Output> + Sqrt<Output = T>,
        <<<T as Mul>::Output as Add>::Output as Add<
            <T as Mul>::Output>
        >::Output: Sqrt,
    {
        (self.len(), self.angle_x(), self.yz().angle())
    }

    /// Calculate polar coordinates with Y being the polar axis.
    #[allow(clippy::type_complexity)]
    pub fn polar_y(
        self
    ) -> (
            <<<<T as Mul>::Output as Add>::Output as Add<
                <T as Mul>::Output>
            >::Output as Sqrt>::Output,
            <T as Goniometric>::Output,
            <T as Goniometric>::Output,
        )
    where
        T: Copy + Mul + Goniometric,
        <T as Mul>::Output: Add<<T as Mul>::Output>,
        <<T as Mul>::Output as Add>::Output:
            Add<<T as Mul>::Output> + Sqrt<Output = T>,
        <<<T as Mul>::Output as Add>::Output as Add<
            <T as Mul>::Output
        >>::Output: Sqrt,
    {
        (self.len(), self.angle_y(), self.zx().angle())
    }

    /// Calculate polar coordinates with Z being the polar axis.
    #[allow(clippy::type_complexity)]
    pub fn polar_z(
        self
    ) -> (
            <<<<T as Mul>::Output as Add>::Output as Add<
                <T as Mul>::Output>
            >::Output as Sqrt>::Output,
            <T as Goniometric>::Output,
            <T as Goniometric>::Output,
        )
    where
        T: Copy + Mul + Goniometric,
        <T as Mul>::Output: Add<<T as Mul>::Output>,
        <<T as Mul>::Output as Add>::Output:
            Add<<T as Mul>::Output> + Sqrt<Output = T>,
        <<<T as Mul>::Output as Add>::Output as Add<
            <T as Mul>::Output>
        >::Output: Sqrt,
    {
        (self.len(), self.angle_z(), self.xy().angle())
    }

    /// Get normalized version of the vector.
    #[allow(clippy::type_complexity)]
    pub fn normalized(
        self,
    ) -> Vec2<
        <T::Float as Div<
            <<<<T::Float as Mul>::Output as Add>::Output as Add<
                <T::Float as Mul>::Output,
            >>::Output as Sqrt>::Output,
        >>::Output,
    >
    where
        T: IntoFloat,
        T::Float: Copy + Mul,
        <T::Float as Mul>::Output: Add,
        <<T::Float as Mul>::Output as Add>::Output:
            Add<<T::Float as Mul>::Output>,
        <<<T::Float as Mul>::Output as Add>::Output as Add<
            <T::Float as Mul>::Output,
        >>::Output: Sqrt,
        <<<<T::Float as Mul>::Output as Add>::Output as Add<
            <T::Float as Mul>::Output,
        >>::Output as Sqrt>::Output: Copy,
        T::Float: Div<
            <<<<T::Float as Mul>::Output as Add>::Output as Add<
                <T::Float as Mul>::Output,
            >>::Output as Sqrt>::Output,
        >,
    {
        let v = self.map(|a| a.into_float());
        let len = v.len();
        (v.x / len, v.y / len).into()
    }

    /// Normalize the vector.
    pub fn normalize(&mut self)
    where
        T: Copy + Float + Mul,
        T::Output: Add,
        <T::Output as Add>::Output: Add<T::Output>,
        <<T::Output as Add>::Output as Add<T::Output>>::Output: Sqrt,
        <<<T::Output as Add>::Output as Add<
            T::Output
        >>::Output as Sqrt>::Output: Copy,
        T: DivAssign<<<<T::Output as Add>::Output as Add<
            T::Output
        >>::Output as Sqrt>::Output>,
    {
        *self /= self.len();
    }

    /// Creates vector from polar coodinate with polar axis X.
    pub fn from_polar_x<L, P, A>(length: L, polar: P, azimuth: A) -> Self
    where
        P: Copy + Goniometric,
        P::Output: Copy,
        A: Copy + Goniometric,
        L: Copy + Mul<P::Output, Output = T>,
        T: Mul<A::Output, Output = T>,
    {
        let ps = polar.sin();
        (
            length * polar.cos(),
            length * ps * azimuth.cos(),
            length * ps * azimuth.sin(),
        )
            .into()
    }

    /// Creates vector from polar coodinate with polar axis Y.
    pub fn from_polar_y<L, P, A>(length: L, polar: P, azimuth: A) -> Self
    where
        P: Copy + Goniometric,
        P::Output: Copy,
        A: Copy + Goniometric,
        L: Copy + Mul<P::Output, Output = T>,
        T: Mul<A::Output, Output = T>,
    {
        let ps = polar.sin();
        (
            length * ps * azimuth.sin(),
            length * polar.cos(),
            length * ps * azimuth.cos(),
        )
            .into()
    }

    /// Creates vector from polar coodinate with polar axis X.
    pub fn from_polar_z<L, P, A>(length: L, polar: P, azimuth: A) -> Self
    where
        P: Copy + Goniometric,
        P::Output: Copy,
        A: Copy + Goniometric,
        L: Copy + Mul<P::Output, Output = T>,
        T: Mul<A::Output, Output = T>,
    {
        let ps = polar.sin();
        (
            length * ps * azimuth.cos(),
            length * ps * azimuth.sin(),
            length * polar.cos(),
        )
            .into()
    }

    /// Scales the vector to different type. Floating point types are in range
    /// 0..=1 where integer types are in range MIN..=MAX.
    pub fn scale<S>(self) -> Vec3<S>
    where
        T: Scale<S>,
    {
        self.map(|a| a.scale())
    }

    /// Calculates the cross product of two vectors.
    pub fn cross<R>(
        self,
        other: impl Into<Vec3<R>>,
    ) -> Vec3<<T::Output as Sub>::Output>
    where
        T: Copy + Mul<R>,
        T::Output: Sub,
        R: Copy,
    {
        let Vec3 { x, y, z } = other.into();
        (
            self.y * z - self.z * y,
            self.z * x - self.x * z,
            self.x * y - self.y * x,
        )
            .into()
    }

    /// Creates RGB color for 332 format.
    pub fn from_332(c: u8) -> Self
    where
        u8: Scale<T>,
    {
        let mut r = c >> 5;
        r = (r << 5) | (r << 2) | (r >> 1);

        let mut g = (c >> 2) & 7;
        g = (g << 5) | (g << 2) | (g >> 1);

        let mut b = c & 3;
        b |= b << 2;
        b |= b << 4;

        Vec3::new(r, g, b).scale()
    }

    /// Converts this color to rgb 332 color.
    pub fn to_332(self) -> u8
    where
        T: Scale<u8>,
    {
        let (r, g, b) = self.scale().into();
        (r & 0b11100000) | ((g >> 3) & 0b11100) | (b >> 6)
    }

    /// Change the range of values from `ss..=se` to `ds..=de`.
    pub fn change_range(self, ss: T, se: T, ds: T, de: T) -> Vec3<T>
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
    pub fn norm_to_range(self, s: T, e: T) -> Vec3<T>
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
    pub fn to_norm_range(self, s: T, e: T) -> Vec3<T>
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
    pub fn cabs(self) -> Vec3<T>
    where
        T: PartialOrd + Zero + Neg<Output = T>,
    {
        self.map(|a| if a < T::ZERO { -a } else { a })
    }
}

impl Vec3<bool> {
    /// Checks if all values are true.
    pub fn all(self) -> bool {
        self.x & self.y & self.z
    }

    /// Checks if any value is true.
    pub fn any(self) -> bool {
        self.x | self.y | self.z
    }

    /// Checks if no value is true.
    pub fn none(self) -> bool {
        !self.any()
    }

    /// Checks if at least one value is not true.
    pub fn not_all(self) -> bool {
        !self.all()
    }

    /// Counts the number of true values.
    pub fn count(self) -> usize {
        self.map(|a| a as usize).sum()
    }
}

impl<T> Vec3<&T> {
    /// Clones the components.
    pub fn cloned(self) -> Vec3<T>
    where
        T: Clone,
    {
        self.map(|a| a.clone())
    }

    /// Copies the components.
    pub fn copied(self) -> Vec3<T>
    where
        T: Copy,
    {
        self.map(|a| *a)
    }
}

impl<T: Zero> Vec3<T> {
    /// 3D vectors with all components set to zero.
    pub const ZERO: Vec3<T> = Vec3 {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
    };
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

impl<T> From<(Vec2<T>, T)> for Vec3<T> {
    fn from((Vec2 { x, y }, z): (Vec2<T>, T)) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> From<((T, T), T)> for Vec3<T> {
    fn from(((x, y), z): ((T, T), T)) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> From<([T; 2], T)> for Vec3<T> {
    fn from(([x, y], z): ([T; 2], T)) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> From<(T, Vec2<T>)> for Vec3<T> {
    fn from((x, Vec2 { x: y, y: z }): (T, Vec2<T>)) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> From<(T, (T, T))> for Vec3<T> {
    fn from((x, (y, z)): (T, (T, T))) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> From<(T, [T; 2])> for Vec3<T> {
    fn from((x, [y, z]): (T, [T; 2])) -> Self {
        Vec3 { x, y, z }
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
