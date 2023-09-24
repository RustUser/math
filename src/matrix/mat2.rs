use std::ops::Mul;

use crate::matrix::mat3::Mat3;
use crate::matrix::mat4::Mat4;
use crate::matrix::Matrix;
use crate::matrix::matrix_conversion::MatrixConversion;
use crate::matrix::square_matrix::SquareMatrix;
use crate::scalar::Scalar;

pub type Mat2<S> = Matrix<2, 2, S>;

impl<S: Scalar> Mat2<S> {}

impl<S: Scalar> Mul<Self> for Mat2<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        crate::functions::mat2_mul_mat2(self, rhs)
    }
}

impl<S: Scalar> MatrixConversion<S> for Mat2<S> {
    fn mat2(&self) -> Mat2<S> {
        *self
    }

    fn mat3(&self) -> Mat3<S> {
        let mut out: Mat3<S> = Matrix::IDENTITY;

        for i in 0..2 {
            for j in 0..2 {
                out.0[i][j] = self.0[i][j];
            }
        }

        out
    }

    fn mat4(&self) -> Mat4<S> {
        let mut out: Mat4<S> = Matrix::IDENTITY;

        for i in 0..2 {
            for j in 0..2 {
                out.0[i][j] = self.0[i][j];
            }
        }

        out
    }
}

impl<S: Scalar> SquareMatrix<2, S> for Mat2<S> {
    const IDENTITY: Self = Matrix([
                                      [S::ONE, S::ZERO],
                                      [S::ZERO, S::ONE],
                                  ], 4);

    fn identity_fill(value: S) -> Self {
        let mut output = Self::IDENTITY;
        for i in 0..2 {
            output.0[i][i] = value;
        }
        output
    }
}