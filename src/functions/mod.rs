use crate::angle::Angle;
use crate::matrix::mat2::Mat2;
use crate::matrix::mat3::Mat3;
use crate::matrix::mat4::Mat4;
use crate::matrix::Matrix;
use crate::matrix::matrix_conversion::MatrixConversion;
use crate::matrix::square_matrix::SquareMatrix;
use crate::quaternion::Quaternion;
use crate::scalar::Scalar;
use crate::vector::{Vector, Vector2, Vector3, Vector4};
use crate::vector::f32::VectorF32;
use crate::vector::f64::VectorF64;

pub fn atan2<S: Scalar>(a: S, b: S) -> S {
    a.inv_tangent2(b)
}

pub fn sqrt<S: Scalar>(s: S) -> S {
    s.square_root()
}

pub fn sin<S: Scalar>(s: S) -> S {
    s.sine()
}

pub fn cos<S: Scalar>(s: S) -> S {
    s.cosine()
}

pub fn tan<S: Scalar>(s: S) -> S {
    s.tangent()
}

pub fn rad<S: Scalar>(radians: S) -> Angle<S> {
    Angle::Radians(radians)
}

pub fn deg<S: Scalar>(degrees: S) -> Angle<S> {
    Angle::Degrees(degrees)
}

pub fn vec2<S: Scalar>(x: S, y: S) -> Vector2<S> {
    Vector([x, y])
}

pub fn vec3<S: Scalar>(x: S, y: S, z: S) -> Vector3<S> {
    Vector([x, y, z])
}

pub fn vec4<S: Scalar>(x: S, y: S, z: S, w: S) -> Vector4<S> {
    Vector([x, y, z, w])
}

pub fn vec2f32(x: f32, y: f32) -> VectorF32<2> {
    Vector([x, y])
}

pub fn vec3f32(x: f32, y: f32, z: f32) -> VectorF32<3> {
    Vector([x, y, z])
}

pub fn vec4f32(x: f32, y: f32, z: f32, w: f32) -> VectorF32<4> {
    Vector([x, y, z, w])
}

pub fn vec2f64(x: f64, y: f64) -> VectorF64<2> {
    Vector([x, y])
}

pub fn vec3f64(x: f64, y: f64, z: f64) -> VectorF64<3> {
    Vector([x, y, z])
}

pub fn vec4f64(x: f64, y: f64, z: f64, w: f64) -> VectorF64<4> {
    Vector([x, y, z, w])
}

pub fn mat_transpose<const M: usize, const N: usize, S: Scalar>(matrix: Matrix<M, N, S>) -> Matrix<N, M, S> {
    let mut out: Matrix<N, M, S> = Matrix::ZERO;
    for j in 0..N {
        for i in 0..M {
            out.0[j][i] = matrix.0[i][j];
        }
    }
    out
}

pub fn max<C: Ord>(a: C, b: C) -> C {
    std::cmp::max(a, b)
}

pub fn min<C: Ord>(a: C, b: C) -> C {
    std::cmp::min(a, b)
}

pub fn mat2<S: Scalar>(a: S, b: S, c: S, d: S) -> Mat2<S> {
    Matrix([
               [a, b],
               [c, d]
           ], 4)
}

pub fn mat3<S: Scalar>(a: S, b: S, c: S, d: S, e: S, f: S, g: S, h: S, i: S) -> Mat3<S> {
    Matrix([
               [a, b, c],
               [d, e, f],
               [g, h, i]
           ], 4)
}

pub fn mat4<S: Scalar>(a: S, b: S, c: S, d: S, e: S, f: S, g: S, h: S, i: S, j: S, k: S, l: S, m: S, n: S, o: S, p: S) -> Mat4<S> {
    Matrix([
               [a, b, c, d],
               [e, f, g, h],
               [i, j, k, l],
               [m, n, o, p]
           ], 4)
}

pub fn mat_fill<const M: usize, const N: usize, S: Scalar>(fill: S) -> Matrix<M, N, S> {
    Matrix([[fill; N]; M], 4)
}

pub fn mat_identity_fill<const L: usize, S: Scalar>(fill: S) -> Matrix<L, L, S> {
    let mut out = Matrix::ZERO;
    for i in 0..L {
        out.0[i][i] = fill;
    }
    out
}

pub fn translation<S: Scalar>(translation: Vector3<S>) -> Mat4<S> {
    let mut t = Mat4::IDENTITY;
    for i in 0..3 {
        t.0[i][3] = translation.0[i];
    }
    t
}

pub fn scale<S: Scalar>(scale: Vector3<S>) -> Mat4<S> {
    let (zero, one) = S::zero_one();
    Matrix([
               [scale.x(), zero, zero, zero],
               [zero, scale.y(), zero, zero],
               [zero, zero, scale.z(), zero],
               [zero, zero, zero, one]
           ], 4)
}

pub fn look_at<S: Scalar>(eye: Vector3<S>, center: Vector3<S>, up: Vector3<S>) -> Mat4<S> {
    let f = (center - eye).normalize();
    let s = f.cross_product(up);
    let u = s.cross_product(f);

    Matrix([
               [s.x(), u.x(), -f.x(), S::ZERO],
               [s.y(), u.y(), -f.z(), S::ZERO],
               [s.z(), u.z(), -f.z(), S::ZERO],
               [-s.dot_product(eye), -u.dot_product(eye), f.dot_product(eye), S::ONE]
           ], 4)
}

pub fn perspective<S: Scalar>(aspect_ratio: S, fov: S, near: S, far: S) -> Mat4<S> {
    let (zero, one, two): (S, S, S) = (S::ZERO, S::ONE, S::from_f32(2.0));
    let fov = fov / two;

    let a = one / (aspect_ratio * fov.tangent());
    let b = one / fov.tangent();
    let c = -(far + near) / (far - near);
    let d = -(two * far * near) / (far - near);
    let e = -one;

    Matrix([
               [a, zero, zero, zero],
               [zero, b, zero, zero],
               [zero, zero, c, e],
               [zero, zero, d, zero]
           ], 4)
}

pub fn orthographic<S: Scalar>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Mat4<S> {
    let (zero, one) = S::zero_one();
    let two = S::from_f32(2.0);
    Matrix([
               [two / (right - left), zero, zero, -(right + left) / (right - left)],
               [zero, two / (top - bottom), zero, -(top + bottom) / (top - bottom)],
               [zero, zero, -two / (far - near), -(far + near) / (far - near)],
               [zero, zero, zero, one]
           ], 4)
}

pub fn rotation_x<S: Scalar>(theta: S) -> Mat3<S> {
    let (sin, cos) = theta.sine_cosine();
    let (zero, one) = S::zero_one();
    Matrix([
               [one, zero, zero],
               [zero, cos, -sin],
               [zero, sin, cos]
           ], 4)
}

pub fn rotation_y<S: Scalar>(theta: S) -> Mat3<S> {
    let (sin, cos) = theta.sine_cosine();
    let (zero, one) = S::zero_one();
    Matrix([
               [cos, zero, sin],
               [zero, one, zero],
               [-sin, zero, cos],
           ], 4)
}

pub fn rotation_z<S: Scalar>(theta: S) -> Mat3<S> {
    let (sin, cos) = theta.sine_cosine();
    let (zero, one) = S::zero_one();
    Matrix([
               [cos, -sin, zero],
               [sin, cos, zero],
               [zero, zero, one],
           ], 4)
}

pub fn mat_to_mat2<S: Scalar, M: MatrixConversion<S>>(matrix: M) -> Mat2<S> {
    matrix.mat2()
}

pub fn mat_to_mat3<S: Scalar, M: MatrixConversion<S>>(matrix: M) -> Mat3<S> {
    matrix.mat3()
}

pub fn mat_to_mat4<S: Scalar, M: MatrixConversion<S>>(matrix: M) -> Mat4<S> {
    matrix.mat4()
}

pub fn mat2_mul_mat2<S: Scalar>(a: Mat2<S>, b: Mat2<S>) -> Mat2<S> {
    let (a1, a2, a3, a4) = (a.0[0][0], a.0[0][1], a.0[1][0], a.0[1][1]);
    let (b1, b2, b3, b4) = (b.0[0][0], b.0[1][0], b.0[0][1], b.0[1][1]);

    let ab11 = a1 * b1 + a2 * b2;
    let ab12 = a1 * b3 + a2 * b4;
    let ab21 = a3 * b1 + a4 * b2;
    let ab22 = a3 * b3 + a4 * b4;

    Matrix([
               [ab11, ab12],
               [ab21, ab22]
           ], 4)
}

pub fn mat3_mul_mat3<S: Scalar>(a: Mat3<S>, b: Mat3<S>) -> Mat3<S> {
    let (a1, a2, a3, a4, a5, a6, a7, a8, a9) = (
        a.0[0][0], a.0[0][1], a.0[0][2],
        a.0[1][0], a.0[1][1], a.0[1][2],
        a.0[2][0], a.0[2][1], a.0[2][2]
    );
    let (b1, b2, b3, b4, b5, b6, b7, b8, b9) = (
        b.0[0][0], b.0[1][0], b.0[2][0],
        b.0[0][1], b.0[1][1], b.0[2][1],
        b.0[0][2], b.0[1][2], b.0[2][2],
    );

    let ab11 = a1 * b1 + a2 * b2 + a3 * b3;
    let ab12 = a1 * b4 + a2 * b5 + a3 * b6;
    let ab13 = a1 * b7 + a2 * b8 + a3 * b9;
    let ab21 = a4 * b1 + a5 * b2 + a6 * b3;
    let ab22 = a4 * b4 + a5 * b5 + a6 * b6;
    let ab23 = a4 * b7 + a5 * b8 + a6 * b9;
    let ab31 = a7 * b1 + a8 * b2 + a9 * b3;
    let ab32 = a7 * b4 + a8 * b5 + a9 * b6;
    let ab33 = a7 * b7 + a8 * b8 + a9 * b9;

    Matrix([
               [ab11, ab12, ab13],
               [ab21, ab22, ab23],
               [ab31, ab32, ab33]
           ], 4)
}

pub fn mat4_mul_mat4<S: Scalar>(a: Mat4<S>, b: Mat4<S>) -> Mat4<S> {
    let mut out = Mat4::IDENTITY;

    for i in 0..4 {
        for j in 0..4 {
            out.0[i][j] = S::ZERO;
            for k in 0..4 {
                out.0[i][j] += a.0[i][k] * b.0[k][j];
            }
        }
    }
    out
}

pub fn quat_mul_quat<S: Scalar>(a: Quaternion<S>, b: Quaternion<S>) -> Quaternion<S> {
    let (a_w, a_x, a_y, a_z) = a.w_xyz();
    let (b_w, b_x, b_y, b_z) = b.w_xyz();
    Quaternion::new(
        vec3(
            a_w * b_x + a_x * b_w + a_y * b_z - a_z * b_y,
            a_w * b_y - a_x * b_z + a_y * b_w + a_z * b_x,
            a_w * b_z + a_x * b_y - a_y * b_x + a_z * b_w,
        ),
        a_w * b_w - a_x * b_x - a_y * b_y - a_z * b_z,
    )
}