use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::{bindings::*, std_types::c_int};

impl Add<Self> for IVec3 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }
}

impl Add<c_int> for IVec3 {
    type Output = Self;

    fn add(mut self, other: c_int) -> Self {
        self.x += other;
        self.y += other;
        self.z += other;
        self
    }
}

impl Sub<Self> for IVec3 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self
    }
}

impl Sub<c_int> for IVec3 {
    type Output = Self;

    fn sub(mut self, other: c_int) -> Self {
        self.x -= other;
        self.y -= other;
        self.z -= other;
        self
    }
}

impl Mul<Self> for IVec3 {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self
    }
}

impl Mul<c_int> for IVec3 {
    type Output = Self;

    fn mul(mut self, other: c_int) -> Self {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self
    }
}

impl Div<Self> for IVec3 {
    type Output = Self;

    fn div(mut self, other: Self) -> Self {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self
    }
}

impl Div<c_int> for IVec3 {
    type Output = Self;

    fn div(mut self, other: c_int) -> Self {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self
    }
}

impl Neg for IVec3 {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}
