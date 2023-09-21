use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use crate::scalar::Scalar;

#[derive(Debug, Clone, Copy)]
pub enum Angle<S: Scalar> {
    Radians(S),
    Degrees(S),
}

impl<S: Scalar> Add<S> for Angle<S> {
    type Output = Self;

    fn add(self, rhs: S) -> Self::Output {
        match self {
            Angle::Radians(r) => Self::Radians(r + rhs),
            Angle::Degrees(d) => Self::Degrees(d + rhs)
        }
    }
}

impl<S: Scalar> Add<Angle<S>> for Angle<S> {
    type Output = Self;

    fn add(self, rhs: Angle<S>) -> Self::Output {
        match self {
            Angle::Radians(r) => Self::Radians(r + rhs.to_radians().to_inner()),
            Angle::Degrees(d) => Self::Degrees(d + rhs.to_degrees().to_inner())
        }
    }
}

impl<S: Scalar> Sub<S> for Angle<S> {
    type Output = Self;

    fn sub(self, rhs: S) -> Self::Output {
        match self {
            Angle::Radians(r) => Self::Radians(r - rhs),
            Angle::Degrees(d) => Self::Degrees(d - rhs)
        }
    }
}

impl<S: Scalar> Sub<Angle<S>> for Angle<S> {
    type Output = Self;

    fn sub(self, rhs: Angle<S>) -> Self::Output {
        match self {
            Angle::Radians(r) => Self::Radians(r - rhs.to_radians().to_inner()),
            Angle::Degrees(d) => Self::Degrees(d - rhs.to_degrees().to_inner())
        }
    }
}

impl<S: Scalar> Angle<S> {
    pub fn rad(s: S) -> Self {
        Self::Radians(s)
    }
    pub fn deg(s: S) -> Self {
        Self::Degrees(s)
    }

    pub fn to_radians(self) -> Self {
        match self {
            Angle::Radians(_) => self,
            Angle::Degrees(d) => Self::Radians(d.rad())
        }
    }

    pub fn to_degrees(self) -> Self {
        match self {
            Angle::Radians(r) => Self::Degrees(r.deg()),
            Angle::Degrees(_) => self
        }
    }

    pub fn is_radians(&self) -> bool {
        match self {
            Angle::Radians(_) => true,
            Angle::Degrees(_) => false
        }
    }
    pub fn is_degrees(&self) -> bool {
        match self {
            Angle::Radians(_) => false,
            Angle::Degrees(_) => true
        }
    }

    pub fn inner(&self) -> &S {
        match &self {
            Angle::Radians(r) => r,
            Angle::Degrees(d) => d
        }
    }

    pub fn to_inner(self) -> S {
        match self {
            Angle::Radians(r) => r,
            Angle::Degrees(d) => d
        }
    }

    pub fn units(&self) -> &'static str {
        match self {
            Angle::Radians(_) => "",
            Angle::Degrees(_) => "°"
        }
    }
}

impl<S: Scalar> Display for Angle<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.inner(), self.units()))
    }
}

impl <S: Scalar> Default for Angle<S> {
    fn default() -> Self {
        Self::Radians(S::ZERO)
    }
}