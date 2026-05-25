use core::ops::Mul;

use crate::{
    Matrix_Identity,
    bindings::{Matrix, Matrix_Mul},
};

impl Mul<Self> for Matrix {
    type Output = Self;

    fn mul(self, right: Self) -> Self {
        let mut result = Matrix_Identity;
        unsafe {
            Matrix_Mul(&raw mut result, &raw const self, &raw const right);
        }
        result
    }
}
