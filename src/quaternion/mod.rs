use std::ops::{Div, Mul};

use crate::angle::Angle;
use crate::euler_angles::EulerAngles;
use crate::functions::{atan2, cos, sin, sqrt, vec3};
use crate::matrix::mat3::Mat3;
use crate::matrix::Matrix;
use crate::scalar::Scalar;
use crate::vector::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion<S: Scalar> {
    xyz: Vector3<S>,
    w: S,
}

impl<S: Scalar> Quaternion<S> {
    pub fn new(xyz: Vector3<S>, w: S) -> Self {
        Self {
            xyz,
            w,
        }
    }

    pub fn pure(xyz: Vector3<S>) -> Self {
        Self {
            xyz,
            w: S::ZERO,
        }
    }

    ///The <i>conjugate</i> of the quaternion is denoted <i>q*</i> is denoted as:
    ///<i>q*=s-v</i>
    pub fn conjugate(&self) -> Self {
        Self {
            xyz: -self.xyz,
            w: self.w,
        }
    }

    ///The 'norm' acts as the magnitude of the Quaternion. |q| = sqrt(s^2 + a^2 + b^2 + c^2) such that <i>q = s+a<b>i</b>+b<b>j</b>+c<b>k</b></i>
    pub fn norm(&self) -> S {
        let mut norm = S::ZERO;
        let two = S::from_f32(2.0);

        for i in 0..3 {
            norm += self.xyz.0[i].pow(two);
        }

        norm += self.w.pow(two);

        norm.square_root()
    }

    pub fn unit(&self) -> Self {
        let norm = self.norm();
        Self {
            xyz: self.xyz / norm,
            w: self.w / norm,
        }
    }

    pub fn inverse(&self) -> Self {
        self.conjugate() / self.norm().pow(S::from_f32(2.0))
    }

    pub fn xyz(&self) -> &Vector3<S> {
        &self.xyz
    }

    pub fn w(&self) -> &S {
        &self.w
    }

    pub fn v(&self) -> &Vector3<S> {
        &self.xyz
    }
    pub fn s(&self) -> &S {
        &self.w
    }

    pub fn x(&self) -> &S { &self.xyz.0[0] }
    pub fn y(&self) -> &S {
        &self.xyz.0[1]
    }
    pub fn z(&self) -> &S {
        &self.xyz.0[2]
    }

    pub fn w_xyz(&self) -> (S, S, S, S) {
        (self.w, *self.x(), *self.y(), *self.z())
    }

    pub fn rotation_matrix(self) -> Mat3<S> {
        self.into()
    }
}

impl<S: Scalar> Into<Mat3<S>> for Quaternion<S> {
    fn into(self) -> Mat3<S> {
        let (w, x, y, z) = self.w_xyz();
        let (one, two): (S, S) = (S::ONE, S::from_f32(2.0));
        Matrix([
                   [one - two * y * y - two * z * z, two * x * y - two * w * z, two * x * z + two * w * y],
                   [two * x * y + two * w * z, one - two * x * x - two * z * z, two * y * z - two * w * x],
                   [two * x * z - two * w * y, two * y * z + two * w * x, one - two * x * x - two * y * y],
               ], 4)
    }
}

impl<S: Scalar> From<EulerAngles<S>> for Quaternion<S> {
    fn from(value: EulerAngles<S>) -> Self {
        let two = S::from_f32(2.0);

        let (roll, pitch, yaw) = (
            value.roll.to_radians().to_inner(),
            value.pitch.to_radians().to_inner(),
            value.yaw.to_radians().to_inner(),
        );

        let (cr, sr) = (
            cos(roll / two),
            sin(roll / two)
        );
        let (cp, sp) = (
            cos(pitch / two),
            sin(pitch / two)
        );
        let (cy, sy) = (
            cos(yaw / two),
            sin(yaw / two)
        );

        let w = cr * cp * cy + sr * sp * sy;
        let x = sr * cp * cy - cr * sp * sy;
        let y = cr * sp * cy + sr * cp * sy;
        let z = cr * cp * sy - sr * sp * cy;

        Quaternion::new(
            vec3(
                x,
                y,
                z,
            ),
            w,
        )
    }
}

impl<S: Scalar> Into<EulerAngles<S>> for Quaternion<S> {
    fn into(self) -> EulerAngles<S> {
        let one = S::ONE;
        let two = S::from_f32(2.0);
        let pi = S::PI;

        let q_w = self.w;
        let q_x = *self.x();
        let q_y = *self.y();
        let q_z = *self.z();

        let sinr_cosp = two * (
            q_w * q_x + q_y * q_z
        );
        let cosr_cosp = one - two * (q_x * q_x + q_y * q_y);

        let sin_p = sqrt(one + two * (q_w * q_y - q_x * q_z));
        let cos_p = sqrt(one - two * (q_w * q_y - q_x * q_z));

        let sin_y_cosp = two * (q_w * q_z + q_x * q_y);
        let cos_y_cosp = one - two * (q_y * q_y + q_z * q_z);

        EulerAngles {
            roll: Angle::rad(
                atan2(sinr_cosp, cosr_cosp)
            ),
            pitch: Angle::rad(
                two * atan2(sin_p, cos_p) - pi / two
            ),
            yaw: Angle::rad(
                atan2(sin_y_cosp, cos_y_cosp)
            ),
        }
    }
}

impl<S: Scalar> Mul<Self> for Quaternion<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        crate::functions::quat_mul_quat(self, rhs)
    }
}

impl<S: Scalar> Div<S> for Quaternion<S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self {
            xyz: self.xyz / rhs,
            w: self.w / rhs,
        }
    }
}