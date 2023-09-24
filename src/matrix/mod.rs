use std::fmt::{Display, Formatter};

use crate::pointer::Pointer;
use crate::scalar::Scalar;

pub mod f32;
pub mod f64;
pub mod square_matrix;
pub mod mat2;
pub mod mat3;
pub mod mat4;

pub mod matrix_conversion;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize, S: Scalar>(pub [[S; N]; M], pub usize);

impl<const M: usize, const N: usize, S: Scalar> Matrix<M, N, S> {
    pub const ZERO: Self = Matrix([[S::ZERO; N]; M], 4);
    pub const ONE: Self = Matrix([[S::ONE; N]; M], 4);

    pub fn transpose(self) -> Matrix<N, M, S> {
        crate::functions::mat_transpose(self)
    }

    pub fn set_decimal_places(&mut self, places: usize) {
        self.1 = places;
    }
}

impl<const M: usize, const N: usize, S: Scalar> Display for Matrix<M, N, S> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        let t = self.0.iter().map(|s| format!("{:?}", s)).collect::<Vec<String>>().join("\n");
        _f.write_str(&t)
    }
}

impl<const M: usize, const N: usize, S: Scalar> Pointer for Matrix<M, N, S> {
    type Ptr = *const S;

    fn as_ptr(&self) -> Self::Ptr {
        &self.0[0][0] as *const S
    }
}