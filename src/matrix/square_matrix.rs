use crate::scalar::Scalar;

pub trait SquareMatrix<const L: usize, S: Scalar> {
    const IDENTITY: Self;

    fn identity_fill(value: S) -> Self;
}