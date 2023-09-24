use crate::{
    matrix::{
        mat2::Mat2,
        mat3::Mat3,
        mat4::Mat4,
    },
    scalar::Scalar,
};

pub trait MatrixConversion<S: Scalar> {
    fn mat2(&self) -> Mat2<S>;
    fn mat3(&self) -> Mat3<S>;
    fn mat4(&self) -> Mat4<S>;
}