mod ops;

pub use self::ops::*;
use crate::{bindings::*, Vec3_IsZero, Vec3_Set};
use std::os::raw::c_float;

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

    pub const fn big_pos() -> Self {
        Vec3_BigPos()
    }

    pub const fn create(x: c_float, y: c_float, z: c_float) -> Self {
        Vec3_Create3(x, y, z)
    }

    pub fn set(&mut self, x: c_float, y: c_float, z: c_float) {
        Vec3_Set!(self, x, y, z);
    }

    pub fn is_zero(&self) -> bool {
        Vec3_IsZero!(self)
    }

    pub fn length_squared(&self) -> c_float {
        Vec3_LengthSquared(self)
    }

    pub fn lerp(&self, b: &Vec3, blend: c_float) -> Self {
        let mut result = Self::zero();
        Vec3_Lerp(&mut result, self, b, blend);
        result
    }

    pub fn normalize(&self) -> Self {
        let mut result = Self::zero();
        Vec3_Normalize(&mut result, self);
        result
    }

    pub fn transform(&self, mat: &Matrix) -> Self {
        let mut result = Self::zero();
        Vec3_Transform(&mut result, self, mat);
        result
    }

    pub fn transform_y(y: c_float, mat: &Matrix) -> Self {
        let mut result = Self::zero();
        Vec3_TransformY(&mut result, y, mat);
        result
    }

    pub fn rotate_x(v: Vec3, angle: c_float) -> Self {
        Vec3_RotateX(v, angle)
    }

    pub fn rotate_y(v: Vec3, angle: c_float) -> Self {
        Vec3_RotateY(v, angle)
    }

    pub fn rotate_y3(x: c_float, y: c_float, z: c_float, angle: c_float) -> Self {
        Vec3_RotateY3(x, y, z, angle)
    }

    pub fn rotate_z(v: Vec3, angle: c_float) -> Self {
        Vec3_RotateZ(v, angle)
    }

    pub fn floor(&self) -> IVec3 {
        let mut result = IVec3::zero();
        IVec3_Floor(&mut result, self);
        result
    }

    pub fn get_dir_vector(yawRad: c_float, pitchRad: c_float) -> Self {
        Vec3_GetDirVector(yawRad, pitchRad)
    }
}

pub const fn Vec3_BigPos() -> Vec3 {
    Vec3 {
        X: 1e25_f32,
        Y: 1e25_f32,
        Z: 1e25_f32,
    }
}

pub const fn Vec3_Create3(x: c_float, y: c_float, z: c_float) -> Vec3 {
    Vec3 { X: x, Y: y, Z: z }
}

/// Returns the squared length of the vector.
/// Squared length can be used for comparison, to avoid a costly sqrt()
/// However, you must sqrt() this when adding lengths.
pub fn Vec3_LengthSquared(v: &Vec3) -> c_float {
    v.X * v.X + v.Y * v.Y + v.Z * v.Z
}

/// Linearly interpolates components of two vectors.
pub fn Vec3_Lerp(result: &mut Vec3, a: &Vec3, b: &Vec3, blend: c_float) {
    result.X = blend * (b.X - a.X) + a.X;
    result.Y = blend * (b.Y - a.Y) + a.Y;
    result.Z = blend * (b.Z - a.Z) + a.Z;
}

/// Scales all components of a vector to lie in [-1, 1]
pub fn Vec3_Normalize(result: &mut Vec3, a: &Vec3) {
    let lenSquared = a.X * a.X + a.Y * a.Y + a.Z * a.Z;
    let scale = 1.0 / lenSquared.sqrt();
    result.X = a.X * scale;
    result.Y = a.Y * scale;
    result.Z = a.Z * scale;
}

/// Transforms a vector by the given matrix.
pub fn Vec3_Transform(result: &mut Vec3, a: &Vec3, mat: &Matrix) {
    // a could be pointing to result - can't directly assign X/Y/Z therefore
    let x = a.X * mat.row1.X + a.Y * mat.row2.X + a.Z * mat.row3.X + mat.row4.X;
    let y = a.X * mat.row1.Y + a.Y * mat.row2.Y + a.Z * mat.row3.Y + mat.row4.Y;
    let z = a.X * mat.row1.Z + a.Y * mat.row2.Z + a.Z * mat.row3.Z + mat.row4.Z;
    result.X = x;
    result.Y = y;
    result.Z = z;
}

/// Same as Vec3_Transform, but faster since X and Z are assumed as 0.
pub fn Vec3_TransformY(result: &mut Vec3, y: c_float, mat: &Matrix) {
    result.X = y * mat.row2.X + mat.row4.X;
    result.Y = y * mat.row2.Y + mat.row4.Y;
    result.Z = y * mat.row2.Z + mat.row4.Z;
}

pub fn Vec3_RotateX(v: Vec3, angle: c_float) -> Vec3 {
    let cosA = angle.cos();
    let sinA = angle.sin();
    Vec3_Create3(v.X, cosA * v.Y + sinA * v.Z, -sinA * v.Y + cosA * v.Z)
}

pub fn Vec3_RotateY(v: Vec3, angle: c_float) -> Vec3 {
    let cosA = angle.cos();
    let sinA = angle.sin();
    Vec3_Create3(cosA * v.X - sinA * v.Z, v.Y, sinA * v.X + cosA * v.Z)
}

pub fn Vec3_RotateY3(x: c_float, y: c_float, z: c_float, angle: c_float) -> Vec3 {
    let cosA = angle.cos();
    let sinA = angle.sin();
    Vec3_Create3(cosA * x - sinA * z, y, sinA * x + cosA * z)
}

pub fn Vec3_RotateZ(v: Vec3, angle: c_float) -> Vec3 {
    let cosA = angle.cos();
    let sinA = angle.sin();
    Vec3_Create3(cosA * v.X + sinA * v.Y, -sinA * v.X + cosA * v.Y, v.Z)
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

/// Returns a normalised vector facing in the direction described by the given yaw and pitch.
pub fn Vec3_GetDirVector(yawRad: c_float, pitchRad: c_float) -> Vec3 {
    let x = -pitchRad.cos() * -yawRad.sin();
    let y = -pitchRad.sin();
    let z = -pitchRad.cos() * yawRad.cos();
    Vec3_Create3(x, y, z)
}
