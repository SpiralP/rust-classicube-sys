use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::{bindings::*, std_types::c_float};

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }
}

impl Add<c_float> for Vec3 {
    type Output = Self;

    fn add(mut self, other: c_float) -> Self {
        self.x += other;
        self.y += other;
        self.z += other;
        self
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self
    }
}

impl Sub<c_float> for Vec3 {
    type Output = Self;

    fn sub(mut self, other: c_float) -> Self {
        self.x -= other;
        self.y -= other;
        self.z -= other;
        self
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self
    }
}

impl Mul<c_float> for Vec3 {
    type Output = Self;

    fn mul(mut self, other: c_float) -> Self {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self
    }
}

impl Div<Self> for Vec3 {
    type Output = Self;

    fn div(mut self, other: Self) -> Self {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self
    }
}

impl Div<c_float> for Vec3 {
    type Output = Self;

    fn div(mut self, other: c_float) -> Self {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}
