use std::fmt::{Display, Formatter};
use crate::angle::Angle;
use crate::scalar::Scalar;

#[derive(Debug, Clone, Copy)]
pub struct EulerAngles<S: Scalar> {
    pub roll: Angle<S>,
    pub pitch: Angle<S>,
    pub yaw: Angle<S>
}

impl<S: Scalar> EulerAngles<S> {
    pub fn to_radians(self) -> Self {
        Self {
            roll: self.roll.to_radians(),
            pitch: self.pitch.to_radians(),
            yaw: self.yaw.to_radians()
        }
    }
    pub fn to_degrees(self) -> Self {
        Self {
            roll: self.roll.to_degrees(),
            pitch: self.pitch.to_degrees(),
            yaw: self.yaw.to_degrees()
        }
    }

}

impl <S: Scalar> Display for EulerAngles<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{} {} {}]", self.roll, self.pitch, self.yaw))
    }
}