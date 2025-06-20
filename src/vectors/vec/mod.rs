mod ops;

use crate::{
    bindings::{IVec3, Matrix, Vec3},
    std_types::{c_float, cosf, floorf, sinf, sqrtf},
    Vec3_IsZero, Vec3_Set,
};

impl Vec3 {
    #[must_use]
    pub const fn new(x: c_float, y: c_float, z: c_float) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[must_use]
    pub const fn big_pos() -> Self {
        Vec3_BigPos()
    }

    #[must_use]
    pub const fn create(x: c_float, y: c_float, z: c_float) -> Self {
        Vec3_Create3(x, y, z)
    }

    pub fn set(&mut self, x: c_float, y: c_float, z: c_float) {
        Vec3_Set!(self, x, y, z);
    }

    #[must_use]
    pub fn is_zero(&self) -> bool {
        Vec3_IsZero!(self)
    }

    #[must_use]
    pub fn length_squared(&self) -> c_float {
        Vec3_LengthSquared(self)
    }

    #[must_use]
    pub fn lerp(&self, b: Vec3, blend: c_float) -> Self {
        let mut result = Self::zero();
        Vec3_Lerp(&mut result, self, &b, blend);
        result
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        let mut result = Self::zero();
        Vec3_Normalize(&mut result, self);
        result
    }

    #[must_use]
    pub fn transform(&self, mat: Matrix) -> Self {
        let mut result = Self::zero();
        Vec3_Transform(&mut result, self, &mat);
        result
    }

    #[must_use]
    pub fn transform_y(y: c_float, mat: Matrix) -> Self {
        let mut result = Self::zero();
        Vec3_TransformY(&mut result, y, &mat);
        result
    }

    #[must_use]
    pub fn rotate_x(v: Vec3, angle: c_float) -> Self {
        Vec3_RotateX(v, angle)
    }

    #[must_use]
    pub fn rotate_y(v: Vec3, angle: c_float) -> Self {
        Vec3_RotateY(v, angle)
    }

    #[must_use]
    pub fn rotate_y3(x: c_float, y: c_float, z: c_float, angle: c_float) -> Self {
        Vec3_RotateY3(x, y, z, angle)
    }

    #[must_use]
    pub fn rotate_z(v: Vec3, angle: c_float) -> Self {
        Vec3_RotateZ(v, angle)
    }

    #[must_use]
    pub fn floor(&self) -> IVec3 {
        let mut result = IVec3::zero();
        IVec3_Floor(&mut result, self);
        result
    }

    #[must_use]
    pub fn get_dir_vector(yawRad: c_float, pitchRad: c_float) -> Self {
        Vec3_GetDirVector(yawRad, pitchRad)
    }
}

#[must_use]
pub const fn Vec3_BigPos() -> Vec3 {
    Vec3 {
        x: 1e25_f32,
        y: 1e25_f32,
        z: 1e25_f32,
    }
}

#[must_use]
pub const fn Vec3_Create3(x: c_float, y: c_float, z: c_float) -> Vec3 {
    Vec3 { x, y, z }
}

/// Returns the squared length of the vector.
/// Squared length can be used for comparison, to avoid a costly `sqrt()`
/// However, you must `sqrt()` this when adding lengths.
#[must_use]
pub fn Vec3_LengthSquared(v: &Vec3) -> c_float {
    v.x * v.x + v.y * v.y + v.z * v.z
}

/// Linearly interpolates components of two vectors.
pub fn Vec3_Lerp(result: &mut Vec3, a: &Vec3, b: &Vec3, blend: c_float) {
    result.x = blend * (b.x - a.x) + a.x;
    result.y = blend * (b.y - a.y) + a.y;
    result.z = blend * (b.z - a.z) + a.z;
}

/// Scales all components of a vector to lie in [-1, 1]
pub fn Vec3_Normalize(result: &mut Vec3, a: &Vec3) {
    let lenSquared = a.x * a.x + a.y * a.y + a.z * a.z;
    let scale = 1.0 / sqrtf(lenSquared);
    result.x = a.x * scale;
    result.y = a.y * scale;
    result.z = a.z * scale;
}

/// Transforms a vector by the given matrix.
pub fn Vec3_Transform(result: &mut Vec3, a: &Vec3, mat: &Matrix) {
    // a could be pointing to result - can't directly assign x/y/z therefore
    let x = a.x * mat.row1.x + a.y * mat.row2.x + a.z * mat.row3.x + mat.row4.x;
    let y = a.x * mat.row1.y + a.y * mat.row2.y + a.z * mat.row3.y + mat.row4.y;
    let z = a.x * mat.row1.z + a.y * mat.row2.z + a.z * mat.row3.z + mat.row4.z;
    result.x = x;
    result.y = y;
    result.z = z;
}

/// Same as `Vec3_Transform`, but faster since x and z are assumed as 0.
pub fn Vec3_TransformY(result: &mut Vec3, y: c_float, mat: &Matrix) {
    result.x = y * mat.row2.x + mat.row4.x;
    result.y = y * mat.row2.y + mat.row4.y;
    result.z = y * mat.row2.z + mat.row4.z;
}

#[must_use]
pub fn Vec3_RotateX(v: Vec3, angle: c_float) -> Vec3 {
    let cosA = cosf(angle);
    let sinA = sinf(angle);
    Vec3_Create3(v.x, cosA * v.y + sinA * v.z, -sinA * v.y + cosA * v.z)
}

#[must_use]
pub fn Vec3_RotateY(v: Vec3, angle: c_float) -> Vec3 {
    let cosA = cosf(angle);
    let sinA = sinf(angle);
    Vec3_Create3(cosA * v.x - sinA * v.z, v.y, sinA * v.x + cosA * v.z)
}

#[must_use]
pub fn Vec3_RotateY3(x: c_float, y: c_float, z: c_float, angle: c_float) -> Vec3 {
    let cosA = cosf(angle);
    let sinA = sinf(angle);
    Vec3_Create3(cosA * x - sinA * z, y, sinA * x + cosA * z)
}

#[must_use]
pub fn Vec3_RotateZ(v: Vec3, angle: c_float) -> Vec3 {
    let cosA = cosf(angle);
    let sinA = sinf(angle);
    Vec3_Create3(cosA * v.x + sinA * v.y, -sinA * v.x + cosA * v.y, v.z)
}

/// Whether all of the components of the two vectors are equal.
#[must_use]
pub fn Vec3_Equals(a: &Vec3, b: &Vec3) -> bool {
    #[allow(clippy::float_cmp)]
    {
        a.x == b.x && a.y == b.y && a.z == b.z
    }
}

pub fn IVec3_Floor(result: &mut IVec3, a: &Vec3) {
    result.x = floorf(a.x) as _;
    result.y = floorf(a.y) as _;
    result.z = floorf(a.z) as _;
}

/// Returns a normalised vector facing in the direction described by the given yaw and pitch.
#[must_use]
pub fn Vec3_GetDirVector(yawRad: c_float, pitchRad: c_float) -> Vec3 {
    let x = -cosf(pitchRad) * -sinf(yawRad);
    let y = -sinf(pitchRad);
    let z = -cosf(pitchRad) * cosf(yawRad);
    Vec3_Create3(x, y, z)
}
