use std::fmt::{Display, Formatter};
use std::mem::size_of;
use std::ops::{Div, Neg, Sub};

use crate::pointer::Pointer;
use crate::scalar::Scalar;

pub mod f32;
pub mod f64;

pub type Vector2<S> = Vector<2, S>;
pub type Vector3<S> = Vector<3, S>;
pub type Vector4<S> = Vector<4, S>;

#[derive(Debug, Clone, Copy)]
pub struct Vector<const L: usize, S: Scalar>(pub [S; L]);

impl<const L: usize, S: Scalar> Pointer for Vector<L, S> {
    type Ptr = *const S;

    fn as_ptr(&self) -> Self::Ptr {
        &self.0[0] as *const _
    }
}

impl<const L: usize, S: Scalar> Display for Vector<L, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.0.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "[{}]", s)//as this
    }
}

impl<const L: usize, S: Scalar> Vector<L, S> {
    pub fn magnitude(&self) -> S {
        self.0.iter().map(|s| s.pow(S::from_f32(2.0))).sum::<S>().square_root()
    }

    pub fn dot_product(&self, b: Self) -> S {
        let mut dot = S::ZERO;
        for i in 0..L {
            dot += self.0[i] * b.0[i];
        }
        dot
    }

    pub fn normalize(&mut self) -> Self {
        let m = self.magnitude();
        self.0.iter_mut().for_each(|s| *s /= m);
        self.clone()
    }

    pub fn normalized(&self) -> Self {
        self.clone().normalize()
    }
}

impl<const L: usize, S: Scalar> Neg for Vector<L, S> {
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

impl<const L: usize, S: Scalar> Div<S> for Vector<L, S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        let mut o = self.0.clone();
        for j in &mut o {
            *j /= rhs;
        }
        Self(o)
    }
}

impl<const L: usize, S: Scalar> Sub<Self> for Vector<L, S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out.0[i] -= rhs.0[i];
        }
        out
    }
}

impl<S: Scalar> Vector2<S> {
    pub const UP: Self = Vector([S::ZERO, S::ONE]);
    pub const DOWN: Self = Vector([S::ZERO, S::NEG_ONE]);
    pub const RIGHT: Self = Vector([S::ONE, S::ZERO]);
    pub const LEFT: Self = Vector([S::NEG_ONE, S::ZERO]);

    pub fn x(&self) -> S {
        self.0[0]
    }
    pub fn y(&self) -> S {
        self.0[1]
    }
}

impl<S: Scalar> Vector3<S> {
    pub const UP: Self = Vector([S::ZERO, S::ONE, S::ZERO]);
    pub const DOWN: Self = Vector([S::ZERO, S::NEG_ONE, S::ZERO]);
    pub const RIGHT: Self = Vector([S::ONE, S::ZERO, S::ZERO]);
    pub const LEFT: Self = Vector([S::NEG_ONE, S::ZERO, S::ZERO]);
    pub const FORWARD: Self = Vector([S::ZERO, S::ZERO, S::ONE]);
    pub const BACKWARD: Self = Vector([S::ZERO, S::ZERO, S::NEG_ONE]);

    pub fn x_y_z(&self) -> (S, S, S) {
        (self.0[0], self.0[1], self.0[2])
    }

    pub fn cross_product(&self, b: Self) -> Self {
        let (a_x, a_y, a_z) = self.x_y_z();
        let (b_x, b_y, b_z) = b.x_y_z();

        Self([
            a_y * b_z - a_z * b_y,
            a_z * b_x - a_x * b_z,
            a_x * b_y - a_y * b_x
        ])
    }

    pub fn x(&self) -> S {
        self.0[0]
    }

    pub fn y(&self) -> S {
        self.0[1]
    }

    pub fn z(&self) -> S {
        self.0[2]
    }
}

impl<S: Scalar> Vector4<S> {
    pub const UP: Self = Vector([S::ZERO, S::ONE, S::ZERO, S::ZERO]);
    pub const DOWN: Self = Vector([S::ZERO, S::NEG_ONE, S::ZERO, S::ZERO]);
    pub const RIGHT: Self = Vector([S::ONE, S::ZERO, S::ZERO, S::ZERO]);
    pub const LEFT: Self = Vector([S::NEG_ONE, S::ZERO, S::ZERO, S::ZERO]);
    pub const FORWARD: Self = Vector([S::ZERO, S::ZERO, S::ONE, S::ZERO]);
    pub const BACKWARD: Self = Vector([S::ZERO, S::ZERO, S::NEG_ONE, S::ZERO]);

    pub fn x(&self) -> S {
        self.0[0]
    }
    pub fn y(&self) -> S {
        self.0[1]
    }
    pub fn z(&self) -> S {
        self.0[2]
    }
    pub fn w(&self) -> S {
        self.0[3]
    }
}

impl<const L: usize, S: Scalar> Vector<L, S> {
    pub const STRIDE: usize = size_of::<S>() * L;
    pub const ZERO: Self = Self([S::ZERO; L]);
}