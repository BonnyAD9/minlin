//! # minlin
//! 
//! Crate with minimal linear algebra constructs for convinience.
//! 
//! This crate contains implementation for `Vec2`, `Vec3` and `Vec4` with many
//! helpful methods for various use cases. It also provides more specialized types
//! such as `Rect`, `Padding` and `Rgba` for more specialized use cases, extensions
//! for `Range` and various numeric traits.

mod mat2;
mod num_traits;
mod padding;
mod rect;
mod rgba;
mod traits;
mod vec2;
mod vec2_range_iter;
mod vec3;
mod vec3_range_iter;
mod vec4;

pub use self::{
    mat2::*, num_traits::*, padding::*, rect::*, rgba::*, traits::*, vec2::*,
    vec2_range_iter::*, vec3::*, vec3_range_iter::*, vec4::*,
};
