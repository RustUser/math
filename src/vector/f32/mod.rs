use crate::vector::Vector;

pub mod vec2;
pub mod vec3;
pub mod vec4;

pub type VectorF32<const L: usize> = Vector<L, f32>;