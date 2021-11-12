use crate::{bindings::*, Matrix_Identity};
use std::ops::Mul;

impl Mul<Self> for Matrix {
    type Output = Self;

    fn mul(self, right: Self) -> Self {
        let mut result = Matrix_Identity;
        unsafe {
            Matrix_Mul(&mut result, &self, &right);
        }
        result
    }
}
