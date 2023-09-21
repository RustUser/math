pub mod scalar;
pub mod angle;
pub mod vector;
pub mod axis_angle;
pub mod euler_angles;
pub mod quaternion;
pub mod functions;


#[cfg(test)]
mod tests {
    use crate::euler_angles::EulerAngles;
    use crate::quaternion::Quaternion;
    use crate::scalar::Scalar;

    #[test]
    fn it_works() {

        let e = EulerAngles {
            roll: 180_f32.angle_deg(),
            pitch: 90_f32.angle_deg(),
            yaw: 45_f32.angle_deg(),
        };
        let q = Quaternion::from(e);
        let e2: EulerAngles<f32> = q.into();
        println!("{}", e);
        println!("{:?}", q);
        println!("{}", e2.to_degrees());
    }
}