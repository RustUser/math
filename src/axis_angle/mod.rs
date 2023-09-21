use crate::angle::Angle;
use crate::scalar::Scalar;

#[derive(Debug, Clone, Copy)]
pub struct AxisAngle<S: Scalar> {
    pub alpha: Angle<S>,
    pub beta: Angle<S>,
    pub gamma: Angle<S>
}

impl <S: Scalar> AxisAngle<S> {

}