use crate::angle::Angle;
use crate::scalar::Scalar;
use crate::vector::{Vector, Vector2, Vector3, Vector4};
use crate::vector::f32::VectorF32;
use crate::vector::f64::VectorF64;

pub fn sin<S: Scalar>(s: S) -> S {
    s.sine()
}

pub fn cos<S: Scalar>(s: S) -> S {
    s.cosine()
}

pub fn tan<S: Scalar>(s: S) -> S {
    s.tangent()
}

pub fn rad<S: Scalar>(radians: S) -> Angle<S> {
    Angle::Radians(radians)
}

pub fn deg<S: Scalar>(degrees: S) -> Angle<S> {
    Angle::Degrees(degrees)
}

pub fn vec2<S: Scalar>(x: S, y: S) -> Vector2<S> {
    Vector([x, y])
}

pub fn vec3<S: Scalar>(x: S, y: S, z: S) -> Vector3<S> {
    Vector([x, y, z])
}

pub fn vec4<S: Scalar>(x: S, y: S, z: S, w: S) -> Vector4<S> {
    Vector([x, y, z, w])
}

pub fn vec2f32(x: f32, y: f32) -> VectorF32<2> {
    Vector([x, y])
}

pub fn vec3f32(x: f32, y: f32, z: f32) -> VectorF32<3> {
    Vector([x, y, z])
}

pub fn vec4f32(x: f32, y: f32, z: f32, w: f32) -> VectorF32<4> {
    Vector([x, y, z, w])
}

pub fn vec2f64(x: f64, y: f64) -> VectorF64<2> {
    Vector([x, y])
}

pub fn vec3f64(x: f64, y: f64, z: f64) -> VectorF64<3> {
    Vector([x, y, z])
}

pub fn vec4f64(x: f64, y: f64, z: f64, w: f64) -> VectorF64<4> {
    Vector([x, y, z, w])
}