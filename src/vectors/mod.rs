mod ops;

pub use self::ops::*;
use crate::{bindings::*, Int32_MaxValue};
use std::os::raw::{c_float, c_int};

impl Vec3 {
    pub const fn new(x: c_float, y: c_float, z: c_float) -> Self {
        Self { X: x, Y: y, Z: z }
    }

    pub const fn zero() -> Self {
        Self {
            X: 0.0,
            Y: 0.0,
            Z: 0.0,
        }
    }
}

impl IVec3 {
    pub const fn new(x: c_int, y: c_int, z: c_int) -> Self {
        Self { X: x, Y: y, Z: z }
    }

    pub const fn zero() -> Self {
        Self { X: 0, Y: 0, Z: 0 }
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
impl IVec3 {
    pub const fn max_value() -> Self {
        IVec3_MaxValue()
    }
}

pub const fn Vec3_BigPos() -> Vec3 {
    Vec3 {
        X: 1e25 as _,
        Y: 1e25 as _,
        Z: 1e25 as _,
    }
}
impl Vec3 {
    pub const fn big_pos() -> Self {
        Vec3_BigPos()
    }
}

pub const fn Vec3_Create3(x: c_float, y: c_float, z: c_float) -> Vec3 {
    Vec3 { X: x, Y: y, Z: z }
}
impl Vec3 {
    pub const fn create(x: c_float, y: c_float, z: c_float) -> Self {
        Vec3_Create3(x, y, z)
    }
}

/// Sets the X, Y, and Z components of a 3D vector
#[macro_export]
macro_rules! Vec3_Set {
    ($v:expr, $x:expr, $y:expr, $z:expr) => {
        $v.X = $x;
        $v.Y = $y;
        $v.Z = $z;
    };
}
impl Vec3 {
    pub fn set(&mut self, x: c_float, y: c_float, z: c_float) {
        Vec3_Set!(self, x, y, z);
    }
}
impl IVec3 {
    pub fn set(&mut self, x: c_int, y: c_int, z: c_int) {
        Vec3_Set!(self, x, y, z);
    }
}

/// Whether all components of a 3D vector are 0
#[macro_export]
macro_rules! Vec3_IsZero {
    ($v:expr) => {
        #[allow(clippy::float_cmp)]
        {
            $v.X == 0 as _ && $v.Y == 0 as _ && $v.Z == 0 as _
        }
    };
}
impl Vec3 {
    pub fn is_zero(&self) -> bool {
        Vec3_IsZero!(self)
    }
}
impl IVec3 {
    pub fn is_zero(&self) -> bool {
        Vec3_IsZero!(self)
    }
}

/// Returns the squared length of the vector.
/// Squared length can be used for comparison, to avoid a costly sqrt()
/// However, you must sqrt() this when adding lengths.
pub fn Vec3_LengthSquared(v: &Vec3) -> c_float {
    v.X * v.X + v.Y * v.Y + v.Z * v.Z
}
impl Vec3 {
    pub fn length_squared(&self) -> c_float {
        Vec3_LengthSquared(self)
    }
}

#[macro_export]
macro_rules! Vec3_AddBy {
    ($dst:expr, $value:expr) => {
        $crate::Vec3_Add($dst, $dst, $value)
    };
}

#[macro_export]
macro_rules! Vec3_SubBy {
    ($dst:expr, $value:expr) => {
        $crate::Vec3_Sub($dst, $dst, $value)
    };
}

#[macro_export]
macro_rules! Vec3_Mul1By {
    ($dst:expr, $value:expr) => {
        $crate::Vec3_Mul1($dst, $dst, $value)
    };
}

#[macro_export]
macro_rules! Vec3_Mul3By {
    ($dst:expr, $value:expr) => {
        $crate::Vec3_Mul3($dst, $dst, $value)
    };
}

/// Linearly interpolates components of two vectors.
pub fn Vec3_Lerp(result: &mut Vec3, a: &Vec3, b: &Vec3, blend: c_float) {
    result.X = blend * (b.X - a.X) + a.X;
    result.Y = blend * (b.Y - a.Y) + a.Y;
    result.Z = blend * (b.Z - a.Z) + a.Z;
}
impl Vec3 {
    pub fn lerp(&self, b: &Vec3, blend: c_float) -> Self {
        let mut result = Self::zero();
        Vec3_Lerp(&mut result, self, b, blend);
        result
    }
}

/// Scales all components of a vector to lie in [-1, 1]
pub fn Vec3_Normalize(result: &mut Vec3, a: &Vec3) {
    let lenSquared = a.X * a.X + a.Y * a.Y + a.Z * a.Z;
    let scale = 1.0 / lenSquared.sqrt();
    result.X = a.X * scale;
    result.Y = a.Y * scale;
    result.Z = a.Z * scale;
}
impl Vec3 {
    pub fn normalize(&self) -> Self {
        let mut result = Self::zero();
        Vec3_Normalize(&mut result, self);
        result
    }
}

/// Transforms a vector by the given matrix.
pub fn Vec3_Transform(result: &mut Vec3, a: &Vec3, mat: &Matrix) {
    // a could be pointing to result - can't directly assign X/Y/Z therefore
    let x = a.X * mat.Row0.X + a.Y * mat.Row1.X + a.Z * mat.Row2.X + mat.Row3.X;
    let y = a.X * mat.Row0.Y + a.Y * mat.Row1.Y + a.Z * mat.Row2.Y + mat.Row3.Y;
    let z = a.X * mat.Row0.Z + a.Y * mat.Row1.Z + a.Z * mat.Row2.Z + mat.Row3.Z;
    result.X = x;
    result.Y = y;
    result.Z = z;
}
impl Vec3 {
    pub fn transform(&self, mat: &Matrix) -> Self {
        let mut result = Self::zero();
        Vec3_Transform(&mut result, self, mat);
        result
    }
}

/// Same as Vec3_Transform, but faster since X and Z are assumed as 0.
pub fn Vec3_TransformY(result: &mut Vec3, y: c_float, mat: &Matrix) {
    result.X = y * mat.Row1.X + mat.Row3.X;
    result.Y = y * mat.Row1.Y + mat.Row3.Y;
    result.Z = y * mat.Row1.Z + mat.Row3.Z;
}
impl Vec3 {
    pub fn transform_y(y: c_float, mat: &Matrix) -> Self {
        let mut result = Self::zero();
        Vec3_TransformY(&mut result, y, mat);
        result
    }
}

pub fn Vec3_RotateX(v: Vec3, angle: c_float) -> Vec3 {
    let cosA = angle.cos();
    let sinA = angle.sin();
    Vec3_Create3(v.X, cosA * v.Y + sinA * v.Z, -sinA * v.Y + cosA * v.Z)
}
impl Vec3 {
    pub fn rotate_x(v: Vec3, angle: c_float) -> Self {
        Vec3_RotateX(v, angle)
    }
}

pub fn Vec3_RotateY(v: Vec3, angle: c_float) -> Vec3 {
    let cosA = angle.cos();
    let sinA = angle.sin();
    Vec3_Create3(cosA * v.X - sinA * v.Z, v.Y, sinA * v.X + cosA * v.Z)
}
impl Vec3 {
    pub fn rotate_y(v: Vec3, angle: c_float) -> Self {
        Vec3_RotateY(v, angle)
    }
}

pub fn Vec3_RotateY3(x: c_float, y: c_float, z: c_float, angle: c_float) -> Vec3 {
    let cosA = angle.cos();
    let sinA = angle.sin();
    Vec3_Create3(cosA * x - sinA * z, y, sinA * x + cosA * z)
}
impl Vec3 {
    pub fn rotate_y3(x: c_float, y: c_float, z: c_float, angle: c_float) -> Self {
        Vec3_RotateY3(x, y, z, angle)
    }
}

pub fn Vec3_RotateZ(v: Vec3, angle: c_float) -> Vec3 {
    let cosA = angle.cos();
    let sinA = angle.sin();
    Vec3_Create3(cosA * v.X + sinA * v.Y, -sinA * v.X + cosA * v.Y, v.Z)
}
impl Vec3 {
    pub fn rotate_z(v: Vec3, angle: c_float) -> Self {
        Vec3_RotateZ(v, angle)
    }
}

/// Whether all of the components of the two vectors are equal.
pub fn Vec3_Equals(a: &Vec3, b: &Vec3) -> bool {
    #[allow(clippy::float_cmp)]
    {
        a.X == b.X && a.Y == b.Y && a.Z == b.Z
    }
}

pub fn IVec3_Floor(result: &mut IVec3, a: &Vec3) {
    result.X = a.X.floor() as _;
    result.Y = a.Y.floor() as _;
    result.Z = a.Z.floor() as _;
}
impl Vec3 {
    pub fn floor(&self) -> IVec3 {
        let mut result = IVec3::zero();
        IVec3_Floor(&mut result, self);
        result
    }
}

pub fn IVec3_ToVec3(result: &mut Vec3, a: &IVec3) {
    result.X = a.X as _;
    result.Y = a.Y as _;
    result.Z = a.Z as _;
}
impl IVec3 {
    pub fn to_vec3(&self) -> Vec3 {
        let mut result = Vec3::zero();
        IVec3_ToVec3(&mut result, self);
        result
    }
}
impl From<IVec3> for Vec3 {
    fn from(other: IVec3) -> Self {
        other.to_vec3()
    }
}

pub fn IVec3_Min(result: &mut IVec3, a: &IVec3, b: &IVec3) {
    result.X = a.X.min(b.X);
    result.Y = a.Y.min(b.Y);
    result.Z = a.Z.min(b.Z);
}
impl IVec3 {
    pub fn min(&self, b: &IVec3) -> Self {
        let mut result = Self::zero();
        IVec3_Min(&mut result, self, b);
        result
    }
}

pub fn IVec3_Max(result: &mut IVec3, a: &IVec3, b: &IVec3) {
    result.X = a.X.max(b.X);
    result.Y = a.Y.max(b.Y);
    result.Z = a.Z.max(b.Z);
}
impl IVec3 {
    pub fn max(&self, b: &IVec3) -> Self {
        let mut result = Self::zero();
        IVec3_Max(&mut result, self, b);
        result
    }
}

/// Returns a normalised vector facing in the direction described by the given yaw and pitch.
pub fn Vec3_GetDirVector(yawRad: c_float, pitchRad: c_float) -> Vec3 {
    let x = -pitchRad.cos() * -yawRad.sin();
    let y = -pitchRad.sin();
    let z = -pitchRad.cos() * yawRad.cos();
    Vec3_Create3(x, y, z)
}
impl Vec3 {
    pub fn get_dir_vector(yawRad: c_float, pitchRad: c_float) -> Self {
        Vec3_GetDirVector(yawRad, pitchRad)
    }
}
