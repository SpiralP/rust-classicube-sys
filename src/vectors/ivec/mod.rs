mod ops;

pub use self::ops::*;
use crate::{
    bindings::{IVec3, Vec3},
    Int32_MaxValue, Vec3_IsZero, Vec3_Set,
};
use std::os::raw::c_int;

impl IVec3 {
    pub const fn new(x: c_int, y: c_int, z: c_int) -> Self {
        Self { X: x, Y: y, Z: z }
    }

    pub const fn zero() -> Self {
        Self { X: 0, Y: 0, Z: 0 }
    }

    pub fn set(&mut self, x: c_int, y: c_int, z: c_int) {
        Vec3_Set!(self, x, y, z);
    }

    pub fn is_zero(&self) -> bool {
        Vec3_IsZero!(self)
    }

    pub const fn max_value() -> Self {
        IVec3_MaxValue()
    }

    pub fn to_vec3(&self) -> Vec3 {
        let mut result = Vec3::zero();
        IVec3_ToVec3(&mut result, self);
        result
    }

    #[must_use]
    pub fn min(&self, b: IVec3) -> Self {
        let mut result = Self::zero();
        IVec3_Min(&mut result, self, &b);
        result
    }

    #[must_use]
    pub fn max(&self, b: IVec3) -> Self {
        let mut result = Self::zero();
        IVec3_Max(&mut result, self, &b);
        result
    }
}

impl From<IVec3> for Vec3 {
    fn from(other: IVec3) -> Self {
        other.to_vec3()
    }
}

/// Returns a vector with all components set to Int32_MaxValue.
pub const fn IVec3_MaxValue() -> IVec3 {
    IVec3 {
        X: Int32_MaxValue,
        Y: Int32_MaxValue,
        Z: Int32_MaxValue,
    }
}

pub fn IVec3_ToVec3(result: &mut Vec3, a: &IVec3) {
    result.X = a.X as _;
    result.Y = a.Y as _;
    result.Z = a.Z as _;
}

pub fn IVec3_Min(result: &mut IVec3, a: &IVec3, b: &IVec3) {
    result.X = a.X.min(b.X);
    result.Y = a.Y.min(b.Y);
    result.Z = a.Z.min(b.Z);
}

pub fn IVec3_Max(result: &mut IVec3, a: &IVec3, b: &IVec3) {
    result.X = a.X.max(b.X);
    result.Y = a.Y.max(b.Y);
    result.Z = a.Z.max(b.Z);
}
