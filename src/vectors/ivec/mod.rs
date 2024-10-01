mod ops;

use crate::{
    bindings::{IVec3, Vec3},
    std_types::c_int,
    Int32_MaxValue, Vec3_IsZero, Vec3_Set,
};

impl IVec3 {
    pub const fn new(x: c_int, y: c_int, z: c_int) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
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
        x: Int32_MaxValue,
        y: Int32_MaxValue,
        z: Int32_MaxValue,
    }
}

pub fn IVec3_ToVec3(result: &mut Vec3, a: &IVec3) {
    result.x = a.x as _;
    result.y = a.y as _;
    result.z = a.z as _;
}

pub fn IVec3_Min(result: &mut IVec3, a: &IVec3, b: &IVec3) {
    result.x = a.x.min(b.x);
    result.y = a.y.min(b.y);
    result.z = a.z.min(b.z);
}

pub fn IVec3_Max(result: &mut IVec3, a: &IVec3, b: &IVec3) {
    result.x = a.x.max(b.x);
    result.y = a.y.max(b.y);
    result.z = a.z.max(b.z);
}
