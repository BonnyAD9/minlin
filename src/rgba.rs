use std::ops::{Deref, DerefMut};

use crate::{Float, MapExt, NormalLimits, Scale, Vec4};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rgba<T = u8>(pub Vec4<T>);

impl<T> Rgba<T> {
    /// Get the red component.
    pub fn r(&self) -> T
    where
        T: Copy,
    {
        self.x
    }

    /// Get the green component.
    pub fn g(&self) -> T
    where
        T: Copy,
    {
        self.y
    }

    /// Get the blue component.
    pub fn b(&self) -> T
    where
        T: Copy,
    {
        self.z
    }

    /// Get the alpha (transparency) component.
    pub fn a(&self) -> T
    where
        T: Copy,
    {
        self.w
    }

    /// Get mutable red component.
    pub fn r_mut(&mut self) -> &mut T {
        &mut self.x
    }

    /// Get mutable green component.
    pub fn g_mut(&mut self) -> &mut T {
        &mut self.y
    }

    /// Get mutable blue component.
    pub fn b_mut(&mut self) -> &mut T {
        &mut self.z
    }

    /// Get mutable alpha (transparency) component.
    pub fn a_mut(&mut self) -> &mut T {
        &mut self.w
    }

    /// Construct from components with transparency.
    #[allow(clippy::self_named_constructors)]
    pub const fn rgba(r: T, g: T, b: T, a: T) -> Self {
        Self(Vec4::new(r, g, b, a))
    }

    /// Construct opaque from components.
    pub const fn rgb(r: T, g: T, b: T) -> Self
    where
        T: NormalLimits,
    {
        Self::rgba(r, g, b, T::NORM_MAX)
    }

    /// Construct from hex value with transparency.
    pub fn xrgba(rgba: u32) -> Self
    where
        u8: Scale<T>,
    {
        let r = ((rgba >> 24) as u8).scale();
        let g = ((rgba >> 16) as u8).scale();
        let b = ((rgba >> 8) as u8).scale();
        let a = (rgba as u8).scale();
        Self::rgba(r, g, b, a)
    }

    /// Construct opaque from hex value.
    pub fn xrgb(rgb: u32) -> Self
    where
        u8: Scale<T>,
        T: NormalLimits,
    {
        let r = ((rgb >> 16) as u8).scale();
        let g = ((rgb >> 8) as u8).scale();
        let b = (rgb as u8).scale();
        Self::rgb(r, g, b)
    }

    /// Construct from rgb with floating point transparency.
    pub fn rgbf<F: Float + Scale<T>>(r: T, g: T, b: T, a: F) -> Self {
        Self::rgba(r, g, b, a.scale())
    }

    /// Construct from hex with floating point transparency.
    pub fn xrgbf<F: Float + Scale<T>>(rgb: u32, a: F) -> Self
    where
        u8: Scale<T>,
    {
        let r = ((rgb >> 16) as u8).scale();
        let g = ((rgb >> 8) as u8).scale();
        let b = (rgb as u8).scale();
        Self::rgba(r, g, b, a.scale())
    }

    /// Construct same color but transparent.
    pub fn transparent(self, a: impl Scale<T>) -> Self {
        Self::rgba(self.0.x, self.0.y, self.0.z, a.scale())
    }

    /// Construct same color but opaque.
    pub fn opaque(self) -> Self
    where
        T: NormalLimits,
    {
        Self::rgb(self.0.x, self.0.y, self.0.z)
    }

    /// Convert to u8 components.
    pub fn to8(self) -> Rgba<u8>
    where
        T: Scale<u8>,
    {
        self.scale()
    }

    /// Convert to float components.
    pub fn tof(self) -> Rgba<f32>
    where
        T: Scale<f32>,
    {
        self.scale()
    }
}

impl<T> Default for Rgba<T>
where
    T: NormalLimits,
{
    fn default() -> Self {
        Self::BLACK
    }
}

impl<T: NormalLimits> Rgba<T> {
    /// Black color.
    pub const WHITE: Self = Self::rgb(T::NORM_MIN, T::NORM_MIN, T::NORM_MIN);

    /// White color.
    pub const BLACK: Self = Self::rgb(T::NORM_MIN, T::NORM_MIN, T::NORM_MIN);

    /// Transparent black.
    pub const TRANSPARENT: Self =
        Self::rgba(T::NORM_MIN, T::NORM_MIN, T::NORM_MIN, T::NORM_MIN);
}

impl<T> Deref for Rgba<T> {
    type Target = Vec4<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Rgba<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> MapExt for Rgba<T> {
    type Val = T;
    type This<R> = Rgba<R>;

    fn map<R>(self, f: impl FnMut(Self::Val) -> R) -> Self::This<R> {
        Rgba(self.0.map(f))
    }
}
