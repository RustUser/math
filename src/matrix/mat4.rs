use std::ffi::NulError;
use std::ops::Mul;

use crate::angle::Angle;
use crate::matrix::Matrix;
use crate::matrix::matrix_conversion::MatrixConversion;
use crate::matrix::square_matrix::SquareMatrix;
use crate::pointer::Pointer;
use crate::scalar::Scalar;
use crate::vector::Vector3;

pub type Mat4<S> = Matrix<4, 4, S>;

impl<S: Scalar> Mat4<S> {
    pub fn rotation_x(angle: Angle<S>) -> Mat4<S> {
        crate::functions::rotation_x(angle.to_radians().to_inner()).mat4()
    }
    pub fn rotation_y(angle: Angle<S>) -> Mat4<S> {
        crate::functions::rotation_y(angle.to_radians().to_inner()).mat4()
    }
    pub fn rotation_z(angle: Angle<S>) -> Mat4<S> {
        crate::functions::rotation_z(angle.to_radians().to_inner()).mat4()
    }
    pub fn perspective(aspect_ratio: S, fov: Angle<S>, near: S, far: S) -> Mat4<S> {
        crate::functions::perspective(aspect_ratio, fov.to_radians().to_inner(), near, far)
    }
    pub fn orthographic(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Mat4<S> {
        crate::functions::orthographic(left, right, bottom, top, near, far)
    }
    pub fn look_at(eye: Vector3<S>, center: Vector3<S>, up: Vector3<S>) -> Self {
        crate::functions::look_at(eye, center, up)
    }
    pub fn translation(translation: Vector3<S>) -> Mat4<S> {
        crate::functions::translation(translation)
    }
}

impl<S: Scalar> SquareMatrix<4, S> for Mat4<S> {
    const IDENTITY: Self = Matrix([
                                      [S::ONE, S::ZERO, S::ZERO, S::ZERO],
                                      [S::ZERO, S::ONE, S::ZERO, S::ZERO],
                                      [S::ZERO, S::ZERO, S::ONE, S::ZERO],
                                      [S::ZERO, S::ZERO, S::ZERO, S::ONE]
                                  ], 4);

    fn identity_fill(value: S) -> Self {
        crate::functions::mat_identity_fill(value)
    }
}

impl<S: Scalar> Mul<Self> for Mat4<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        crate::functions::mat4_mul_mat4(self, rhs)
    }
}

#[cfg(feature = "gfx")]
impl crate::gfx::GfxBind for Mat4<f32> {
    fn bind(&self, name: &dyn ToString, program: u32) -> Result<(), NulError> {
        unsafe {
            let loc = self.uniform_location(name, program)?;
            gl::UniformMatrix4fv(
                loc,
                1,
                gl::FALSE,
                self.as_ptr(),
            );
            Ok(())
        }
    }
}