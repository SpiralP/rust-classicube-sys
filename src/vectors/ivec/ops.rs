use crate::bindings::*;
use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    os::raw::c_int,
};

impl Add<Self> for IVec3 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.X += other.X;
        self.Y += other.Y;
        self.Z += other.Z;
        self
    }
}

impl Add<c_int> for IVec3 {
    type Output = Self;

    fn add(mut self, other: c_int) -> Self {
        self.X += other;
        self.Y += other;
        self.Z += other;
        self
    }
}

impl Sub<Self> for IVec3 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self.X -= other.X;
        self.Y -= other.Y;
        self.Z -= other.Z;
        self
    }
}

impl Sub<c_int> for IVec3 {
    type Output = Self;

    fn sub(mut self, other: c_int) -> Self {
        self.X -= other;
        self.Y -= other;
        self.Z -= other;
        self
    }
}

impl Mul<Self> for IVec3 {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self.X *= other.X;
        self.Y *= other.Y;
        self.Z *= other.Z;
        self
    }
}

impl Mul<c_int> for IVec3 {
    type Output = Self;

    fn mul(mut self, other: c_int) -> Self {
        self.X *= other;
        self.Y *= other;
        self.Z *= other;
        self
    }
}

impl Div<Self> for IVec3 {
    type Output = Self;

    fn div(mut self, other: Self) -> Self {
        self.X /= other.X;
        self.Y /= other.Y;
        self.Z /= other.Z;
        self
    }
}

impl Div<c_int> for IVec3 {
    type Output = Self;

    fn div(mut self, other: c_int) -> Self {
        self.X /= other;
        self.Y /= other;
        self.Z /= other;
        self
    }
}

impl Neg for IVec3 {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.X = -self.X;
        self.Y = -self.Y;
        self.Z = -self.Z;
        self
    }
}
