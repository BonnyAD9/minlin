use std::ops::{Add, Div, Mul, Range, Sub};

use crate::{CompArithm, Padding, RangeExt, Two, Vec2, Zero};

pub trait RectExt: Sized {
    type Val: Copy
        + Add<Output = Self::Val>
        + Sub<Output = Self::Val>
        + PartialOrd;

    /// Create new rectangle from position of top left corner and size.
    fn from_pos_size(
        pos: impl Into<Vec2<Self::Val>>,
        size: impl Into<Vec2<Self::Val>>,
    ) -> Self;

    /// Get the width of the rectangle.
    fn width(&self) -> Self::Val;

    /// Get the height of the rectangle.
    fn height(&self) -> Self::Val;

    /// Get the x coordinate of the rectangle.
    fn x(&self) -> Self::Val;

    /// Get the y coordinate of the rectangle.
    fn y(&self) -> Self::Val;

    /// Set the width of the rectangle.
    fn set_width(&mut self, w: Self::Val);

    /// Set the height of the rectangle.
    fn set_height(&mut self, h: Self::Val);

    /// Set the x coordinate of the rectangle.
    fn set_x(&mut self, x: Self::Val);

    /// Set the y coordinate of the rectangle.
    fn set_y(&mut self, y: Self::Val);

    /// Create the rectangle from ranges of coordinates.
    fn from_ranges(
        lr: impl Into<Range<Self::Val>>,
        tb: impl Into<Range<Self::Val>>,
    ) -> Self {
        let lr = lr.into();
        let tb = tb.into();
        Self::from_pos_size((lr.start, tb.start), (lr.size(), tb.size()))
    }

    /// Create the rectangle from two points.
    fn from_points(
        a: impl Into<Vec2<Self::Val>>,
        b: impl Into<Vec2<Self::Val>>,
    ) -> Self {
        let a = a.into();
        let b = b.into();
        Self::from_ranges(
            Vec2::new(a.x, b.x).sorted(),
            Vec2::new(a.y, b.y).sorted(),
        )
    }

    /// Get the position of the rectangle.
    fn pos(&self) -> Vec2<Self::Val> {
        Vec2::new(self.x(), self.y())
    }

    /// Get the size of the rectangle.
    fn size(&self) -> Vec2<Self::Val> {
        Vec2::new(self.width(), self.height())
    }

    /// Get the left coordinate of the rectangle.
    fn left(&self) -> Self::Val {
        self.x()
    }

    /// Get the top coordinate of the rectangle.
    fn top(&self) -> Self::Val {
        self.y()
    }

    /// Get the right coordinate of the rectangle.
    fn right(&self) -> Self::Val {
        self.x() + self.width()
    }

    /// Get the bottom coordinate of the rectangle.
    fn bottom(&self) -> Self::Val {
        self.y() + self.height()
    }

    /// Get the top left corner position.
    fn top_left(&self) -> Vec2<Self::Val> {
        self.pos()
    }

    /// Get the top right corner position.
    fn top_right(&self) -> Vec2<Self::Val> {
        Vec2::new(self.right(), self.top())
    }

    /// Get the bottom right corner position.
    fn bot_right(&self) -> Vec2<Self::Val> {
        self.pos() + self.size()
    }

    /// Get the bottom left corner position.
    fn bot_left(&self) -> Vec2<Self::Val> {
        Vec2::new(self.left(), self.bottom())
    }

    /// Get the range in x coordinate.
    fn xrange(&self) -> Range<Self::Val> {
        self.left()..self.right()
    }

    /// Get the range in y coordinate.
    fn yrange(&self) -> Range<Self::Val> {
        self.top()..self.bottom()
    }

    /// Get the center of the rectangle.
    fn center(&self) -> Vec2<Self::Val>
    where
        Self::Val: Div<Output = Self::Val> + Two,
    {
        self.pos() + self.size() / Self::Val::TWO
    }

    /// Shrink the rectangle with padding.
    fn pad_rect(&self, padding: impl Into<Padding<Self::Val>>) -> Self
    where
        Self::Val: PartialOrd + Zero,
    {
        let p = padding.into();
        Self::from_pos_size(
            self.pos() + p.offset(),
            self.size().cjoin(p.size(), |s, p| {
                if p >= s { Self::Val::ZERO } else { s - p }
            }),
        )
    }

    /// Enlarge the rectangle with the given padding.
    fn extend_rect(&self, margin: impl Into<Padding<Self::Val>>) -> Self {
        let m = margin.into();
        Self::from_pos_size(self.pos() - m.offset(), self.size() + m.size())
    }

    /// Create rectangle that encloses both of the rectangles.
    fn bound_join(&self, other: impl Into<Self>) -> Self {
        let o = other.into();
        Self::from_ranges(
            self.xrange().join_gap(o.xrange()),
            self.yrange().join_gap(o.yrange()),
        )
    }

    /// Create rectangle that is the intersection of two rectangles.
    fn intersect(&self, other: impl Into<Self>) -> Self {
        let o = other.into();
        Self::from_ranges(
            self.xrange().intersect(o.xrange()),
            self.yrange().intersect(o.yrange()),
        )
    }

    /// Change the position of the rectangle.
    fn move_to(&mut self, pos: impl Into<Vec2<Self::Val>>) -> &mut Self {
        let pos = pos.into();
        self.set_x(pos.x);
        self.set_y(pos.y);
        self
    }

    /// Check whether this rectangle fully encloses the other rectangle.
    fn encloses(&self, other: &Self) -> bool {
        self.xrange().encloses(other.xrange())
            && self.yrange().encloses(other.yrange())
    }

    /// Check whether this rectangle contains the given point.
    fn contains(&self, point: impl Into<Vec2<Self::Val>>) -> bool {
        let p = point.into();
        self.xrange().contains(&p.x) && self.yrange().contains(&p.y)
    }

    /// Check whether two rectangles intersect.
    fn intersects(&self, other: &Self) -> bool {
        self.xrange().intersects(other.xrange())
            && self.yrange().intersects(other.yrange())
    }

    /// Get the area of the rectangle.
    fn area(&self) -> Self::Val
    where
        Self::Val: Mul<Output = Self::Val>,
    {
        self.size().prod()
    }

    /// Check whether the rectangle is empty.
    fn is_empty(&self) -> bool
    where
        Self::Val: PartialEq + Zero,
    {
        self.width() == Self::Val::ZERO || self.height() == Self::Val::ZERO
    }

    /// Clamp the point to be contained in the rectangle.
    fn clamp(&self, pt: impl Into<Vec2<Self::Val>>) -> Vec2<Self::Val> {
        let pt = pt.into();
        let x = self.xrange().clamp(pt.x);
        let y = self.yrange().clamp(pt.y);
        Vec2::new(x, y)
    }

    /// Sets the position to the given value.
    fn set_pos(&mut self, pos: impl Into<Vec2<Self::Val>>) {
        let s = pos.into();
        self.set_x(s.x);
        self.set_y(s.y);
    }

    /// Sets the size of the rectangle.
    fn set_size(&mut self, size: impl Into<Vec2<Self::Val>>) {
        let s = size.into();
        self.set_width(s.x);
        self.set_height(s.y);
    }

    /// Extend the rectangle from bottom by the given amount.
    fn extend_bot(&mut self, amt: Self::Val) {
        self.set_height(self.height() + amt);
    }

    /// Extend the rectangle from right by the given amount.
    fn extend_right(&mut self, amt: Self::Val) {
        self.set_width(self.width() + amt);
    }

    /// Extend the rectangle from left by the given amount.
    fn extend_left(&mut self, amt: Self::Val) {
        self.set_width(self.width() + amt);
        self.set_x(self.x() - amt);
    }

    /// Extend the rectangle from left by the given amount.
    fn extend_top(&mut self, amt: Self::Val) {
        self.set_height(self.height() + amt);
        self.set_y(self.y() - amt);
    }
}
