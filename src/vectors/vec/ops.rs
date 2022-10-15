use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::{bindings::*, std_types::c_float};

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.X += other.X;
        self.Y += other.Y;
        self.Z += other.Z;
        self
    }
}

impl Add<c_float> for Vec3 {
    type Output = Self;

    fn add(mut self, other: c_float) -> Self {
        self.X += other;
        self.Y += other;
        self.Z += other;
        self
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self.X -= other.X;
        self.Y -= other.Y;
        self.Z -= other.Z;
        self
    }
}

impl Sub<c_float> for Vec3 {
    type Output = Self;

    fn sub(mut self, other: c_float) -> Self {
        self.X -= other;
        self.Y -= other;
        self.Z -= other;
        self
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self.X *= other.X;
        self.Y *= other.Y;
        self.Z *= other.Z;
        self
    }
}

impl Mul<c_float> for Vec3 {
    type Output = Self;

    fn mul(mut self, other: c_float) -> Self {
        self.X *= other;
        self.Y *= other;
        self.Z *= other;
        self
    }
}

impl Div<Self> for Vec3 {
    type Output = Self;

    fn div(mut self, other: Self) -> Self {
        self.X /= other.X;
        self.Y /= other.Y;
        self.Z /= other.Z;
        self
    }
}

impl Div<c_float> for Vec3 {
    type Output = Self;

    fn div(mut self, other: c_float) -> Self {
        self.X /= other;
        self.Y /= other;
        self.Z /= other;
        self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.X = -self.X;
        self.Y = -self.Y;
        self.Z = -self.Z;
        self
    }
}
