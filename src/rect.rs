use std::ops::{Add, AddAssign, Deref, DerefMut, Range, Sub, SubAssign};

use crate::{One, Padding, RectExt, Vec2, Vec2RangeIter, Vec4, Zero};

/// Rectangle.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Rect<T = usize>(pub Vec4<T>);

impl<T> Rect<T> {
    /// Create new rectangle from position and size.
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self(Vec4::new(x, y, width, height))
    }
}

impl<T> RectExt for Rect<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    type Val = T;

    fn from_pos_size(
        pos: impl Into<Vec2<Self::Val>>,
        size: impl Into<Vec2<Self::Val>>,
    ) -> Self {
        Self(Vec4::from_pos_size(pos, size))
    }

    fn width(&self) -> Self::Val {
        self.0.width()
    }

    fn height(&self) -> Self::Val {
        self.0.height()
    }

    fn x(&self) -> Self::Val {
        self.0.x()
    }

    fn y(&self) -> Self::Val {
        self.0.y()
    }

    fn set_width(&mut self, w: Self::Val) {
        self.0.set_width(w);
    }

    fn set_height(&mut self, h: Self::Val) {
        self.0.set_height(h);
    }

    fn set_x(&mut self, x: Self::Val) {
        self.0.set_x(x);
    }

    fn set_y(&mut self, y: Self::Val) {
        self.0.set_y(y);
    }
}

impl<T> Deref for Rect<T> {
    type Target = Vec4<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Rect<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Range<Vec2<T>>> for Rect<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    fn from(value: Range<Vec2<T>>) -> Self {
        Self::from_points(value.start, value.end)
    }
}

impl<T, V> From<V> for Rect<T>
where
    Vec4<T>: From<V>,
{
    fn from(value: V) -> Self {
        Self(value.into())
    }
}

impl<T> From<(Range<T>, Range<T>)> for Rect<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    fn from((lr, tb): (Range<T>, Range<T>)) -> Self {
        Self::from_ranges(lr, tb)
    }
}

impl<T> IntoIterator for Rect<T>
where
    T: Copy + AddAssign + PartialOrd + One,
{
    type Item = Vec2<T>;

    type IntoIter = Vec2RangeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut bl = self.xy();
        bl += self.zw();
        self.xy().to(bl)
    }
}

impl<T> Add<Padding<T>> for Rect<T>
where
    T: Add<Output = T> + PartialOrd + Zero + Sub<Output = T> + Copy
{
    type Output = Rect<T>;

    fn add(self, rhs: Padding<T>) -> Self::Output {
        self.pad_rect(rhs)
    }
}

impl<T> AddAssign<Padding<T>> for Rect<T>
where
    T: Add<Output = T> + PartialOrd + Zero + Sub<Output = T> + Copy
{
    fn add_assign(&mut self, rhs: Padding<T>) {
        *self = *self + rhs;
    }
}

impl<T> Sub<Padding<T>> for Rect<T>
where
    T: Add<Output = T> + PartialOrd + Sub<Output = T> + Copy
{
    type Output = Rect<T>;

    fn sub(self, rhs: Padding<T>) -> Self::Output {
        self.extend_rect(rhs)
    }
}

impl<T> SubAssign<Padding<T>> for Rect<T>
where
    T: Add<Output = T> + PartialOrd + Sub<Output = T> + Copy
{
    fn sub_assign(&mut self, rhs: Padding<T>) {
        *self = *self - rhs;
    }
}
