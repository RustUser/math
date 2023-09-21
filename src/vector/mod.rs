use std::ops::{Div, Neg};
use crate::scalar::Scalar;

pub mod f32;
pub mod f64;

pub type Vector2<S> = Vector<2, S>;
pub type Vector3<S> = Vector<3, S>;
pub type Vector4<S> = Vector<4, S>;

#[derive(Debug, Clone, Copy)]
pub struct Vector<const L: usize, S: Scalar>(pub [S; L]);

impl<const L: usize, S: Scalar> Vector<L, S> {
    pub fn magnitude(&self) -> S {
        self.0.iter().map(|s| s.pow(S::from_f32(2.0))).sum::<S>().square_root()
    }

    pub fn dot_product(&self, b: &Self) -> S {
        let mut dot = S::ZERO;
        for i in 0..L {
            dot += self.0[i] * b.0[i];
        }
        dot
    }
}

impl <const L: usize, S: Scalar> Neg for Vector<L, S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let neg_one = -S::ONE;
        let mut o = self.0.clone();
        for i in 0..L {
            o[i] *= neg_one;
        }
        Self(o)
    }
}

impl <const L: usize, S: Scalar> Div<S> for Vector<L, S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        let mut o = self.0.clone();
        for j in &mut o {
            *j /= rhs;
        }
        Self(o)
    }
}