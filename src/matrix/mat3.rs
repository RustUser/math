use std::ops::Mul;

use crate::angle::Angle;
use crate::matrix::mat2::Mat2;
use crate::matrix::mat4::Mat4;
use crate::matrix::Matrix;
use crate::matrix::matrix_conversion::MatrixConversion;
use crate::matrix::square_matrix::SquareMatrix;
use crate::scalar::Scalar;

pub type Mat3<S> = Matrix<3, 3, S>;

impl<S: Scalar> Mul<Self> for Mat3<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        crate::functions::mat3_mul_mat3(self, rhs)
    }
}

impl<S: Scalar> Mat3<S> {
    pub fn rotation_x(angle: Angle<S>) -> Mat3<S> {
        crate::functions::rotation_x(angle.to_radians().to_inner())
    }
    pub fn rotation_y(angle: Angle<S>) -> Mat3<S> {
        crate::functions::rotation_y(angle.to_radians().to_inner())
    }
    pub fn rotation_z(angle: Angle<S>) -> Mat3<S> {
        crate::functions::rotation_z(angle.to_radians().to_inner())
    }
}

impl<S: Scalar> MatrixConversion<S> for Mat3<S> {
    fn mat2(&self) -> Mat2<S> {
        let mut out: Mat2<S> = Matrix::IDENTITY;

        for i in 0..2 {
            for j in 0..2 {
                out.0[i][j] = self.0[i][j];
            }
        }

        out
    }

    fn mat3(&self) -> Mat3<S> {
        *self
    }

    fn mat4(&self) -> Mat4<S> {
        let mut out: Mat4<S> = Matrix::IDENTITY;

        for i in 0..3 {
            for j in 0..3 {
                out.0[i][j] = self.0[i][j];
            }
        }

        out
    }
}

impl<S: Scalar> SquareMatrix<3, S> for Mat3<S> {
    const IDENTITY: Self = Matrix([
                                      [S::ONE, S::ZERO, S::ZERO],
                                      [S::ZERO, S::ONE, S::ZERO],
                                      [S::ZERO, S::ZERO, S::ONE]
                                  ], 4);

    fn identity_fill(value: S) -> Self {
        crate::functions::mat_fill(value)
    }
}