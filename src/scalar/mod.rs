use std::fmt::{Debug, Display};
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::angle::Angle;

crate::scalar!(f32, f64);

pub trait Scalar:
Clone + Copy +
Debug + Display +
Sum + Product +
Neg<Output=Self> +
Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
AddAssign + SubAssign + MulAssign + DivAssign {
    const ZERO: Self;
    const ONE: Self;

    const PI: Self;

    fn rad(self) -> Self;
    fn deg(self) -> Self;

    fn pow(&self, f: Self) -> Self;
    fn square_root(&self) -> Self;

    fn angle_rad(self) -> Angle<Self>;
    fn angle_deg(self) -> Angle<Self>;

    fn sine(self) -> Self;
    fn cosine(self) -> Self;
    fn tangent(self) -> Self;

    fn inv_tangent2(self, b: Self) -> Self;

    fn from_f32(f: f32) -> Self;
}

#[macro_export]
macro_rules! scalar {
    ($($s:ty),*) => {
        $(
            impl Scalar for $s {
                const ZERO: Self = 0.0_f64 as Self;
                const ONE: Self = 1.0_f64 as Self;
                const PI: Self = std::f64::consts::PI as Self;

                fn rad(self) -> Self {
                    self.to_radians()
                }
                fn deg(self) -> Self {
                    self.to_degrees()
                }

                fn pow(&self, f: Self) -> Self {
                    self.powf(f)
                }
                fn square_root(&self) -> Self {
                    self.sqrt()
                }
                fn angle_rad(self) -> Angle<Self> {
                    Angle::Radians(self)
                }
                fn angle_deg(self) -> Angle<Self> {
                    Angle::Degrees(self)
                }
                fn sine(self) -> Self {
                    self.sin()
                }
                fn cosine(self) -> Self {
                    self.cos()
                }
                fn tangent(self) -> Self {
                    self.tan()
                }

                fn inv_tangent2(self, b: Self) -> Self {
                    self.atan2(b)
                }

                fn from_f32(f: f32) -> Self {
                    f as Self
                }
            }
        )*
    };
}