pub mod scalar;
pub mod angle;
pub mod vector;
pub mod matrix;
pub mod axis_angle;
pub mod euler_angles;
pub mod quaternion;
pub mod functions;
pub mod pointer;
pub mod types;

//#[cfg(feature = "gfx")]
pub mod gfx;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::angle::Angle;
    use crate::euler_angles::EulerAngles;
    use crate::functions::vec3;
    use crate::matrix::mat3::Mat3;
    use crate::matrix::mat4::Mat4;
    use crate::matrix::square_matrix::SquareMatrix;
    use crate::quaternion::Quaternion;
    use crate::scalar::Scalar;
    use crate::vector::f32::vec3::Vec3F32;

    #[test]
    fn it_works() {
        let e = EulerAngles {
            roll: 180_f32.angle_deg(),
            pitch: 90_f32.angle_deg(),
            yaw: 45_f32.angle_deg(),
        };

        let q: Quaternion<f32> = e.into();
        println!("{:?}", q);
        let e: EulerAngles<f32> = q.into();
        println!("{}", e);
    }

    #[test]
    fn test_transpose() {
        let mut m = Mat4::identity_fill(5.0);
        m.0[0][1] = 15.0;
        let n = m.transpose();
        println!("{}\n", m);
        println!("{}", n);
    }

    #[test]
    fn test_perspective() {
        let aspect_ratio = 1.0;
        let near = 0.01_f32;
        let far = 1000.0;
        let p = crate::functions::perspective(aspect_ratio, 90.0_f32.to_radians(), near, far);
        println!("{:?}", p);
    }

    #[test]
    fn test_look_at() {
        let eye = vec3(0.0_f32, 0.0, -3.0);
        let center = vec3(0.0, 0.0, 0.0);
        let up = Vec3F32::UP;

        let look_at = crate::functions::look_at(eye, center, up);
    }

    #[test]
    fn mat2_mul() {
        let i = Instant::now();
        let a = crate::functions::mat2(3.0, 7.0, 4.0, 9.0);
        let b = crate::functions::mat2(6.0, 2.0, 5.0, 8.0);

        let c = a * b;

        //let c = crate::functions::mat2_mul_mat2(a, b);
        println!("{}", c);
        println!("{:?}", i.elapsed());
    }

    #[test]
    fn mat3_mul() {
        let i = Instant::now();
        let a = crate::functions::mat3(
            12.0, 8.0, 4.0,
            3.0, 17.0, 14.0,
            9.0, 8.0, 10.0,
        );
        let b = crate::functions::mat3(
            5.0, 19.0, 3.0,
            6.0, 15.0, 9.0,
            7.0, 8.0, 16.0,
        );

        let c = a * b;
        //let c = crate::functions::mat3_mul_mat3(a, b);
        println!("{}", c);
        println!("{:?}", i.elapsed());
    }

    #[test]
    fn mat4_mul() {
        let i = Instant::now();
        let a = crate::functions::mat4(
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            10.0, 11.0, 12.0, 13.0,
            14.0, 15.0, 16.0, 17.0,
        );
        let b = crate::functions::mat4(
            3.0, 4.0, 5.0, 6.0,
            7.0, 8.0, 9.0, 10.0,
            11.0, 12.0, 13.0, 14.0,
            15.0, 16.0, 17.0, 18.0,
        );
        let c = crate::functions::mat4_mul_mat4(a, b);
        println!("{}", c);
        println!("{:?}", i.elapsed());
    }

    #[test]
    fn quat_to_mat4() {
        let e = EulerAngles {
            roll: Angle::Degrees(180.0),
            pitch: Angle::Degrees(90.0),
            yaw: Angle::Degrees(45.0),
        };
        let q: Quaternion<f32> = e.into();
        println!("{:?}", q);
        let m: Mat3<f32> = q.into();

        println!("{}", m);
    }
}