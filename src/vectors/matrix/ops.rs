use std::ops::Mul;

use crate::{bindings::*, Matrix_Identity};

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
