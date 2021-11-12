mod ops;

use crate::{bindings::*, Tan_Simple};
use std::os::raw::{c_double, c_float};

impl Matrix {
    pub const IDENTITY: Self = Matrix_IdentityValue();

    pub fn identity_value() -> Self {
        Matrix_IdentityValue()
    }

    /// Returns a matrix representing a counter-clockwise rotation around X axis.
    pub fn rotate_x(angle: c_float) -> Self {
        let mut result = Self::IDENTITY;
        unsafe {
            Matrix_RotateX(&mut result, angle);
        }
        result
    }

    /// Returns a matrix representing a counter-clockwise rotation around Y axis.
    pub fn rotate_y(angle: c_float) -> Self {
        let mut result = Self::IDENTITY;
        unsafe {
            Matrix_RotateY(&mut result, angle);
        }
        result
    }

    /// Returns a matrix representing a counter-clockwise rotation around Z axis.
    pub fn rotate_z(angle: c_float) -> Self {
        let mut result = Self::IDENTITY;
        unsafe {
            Matrix_RotateZ(&mut result, angle);
        }
        result
    }

    /// Returns a matrix representing a translation to the given coordinates.
    pub fn translate(x: c_float, y: c_float, z: c_float) -> Self {
        let mut result = Self::IDENTITY;
        unsafe { Matrix_Translate(&mut result, x, y, z) }
        result
    }

    /// Returns a matrix representing a scaling by the given factors.
    pub fn scale(x: c_float, y: c_float, z: c_float) -> Self {
        let mut result = Self::IDENTITY;
        unsafe { Matrix_Scale(&mut result, x, y, z) }
        result
    }

    pub fn orthographic(
        left: c_float,
        right: c_float,
        top: c_float,
        bottom: c_float,
        zNear: c_float,
        zFar: c_float,
    ) -> Self {
        let mut result = Self::IDENTITY;
        Matrix_Orthographic(&mut result, left, right, top, bottom, zNear, zFar);
        result
    }

    pub fn perspective_field_of_view(
        fovy: c_float,
        aspect: c_float,
        z_near: c_float,
        z_far: c_float,
    ) -> Self {
        let mut result = Self::IDENTITY;
        Matrix_PerspectiveFieldOfView(&mut result, fovy, aspect, z_near, z_far);
        result
    }

    pub fn look_rot(pos: Vec3, rot: Vec2) -> Self {
        let mut result = Self::IDENTITY;
        Matrix_LookRot(&mut result, pos, rot);
        result
    }
}

pub const fn Matrix_IdentityValue() -> Matrix {
    Matrix {
        row1: Vec4 {
            X: 1.0,
            Y: 0.0,
            Z: 0.0,
            W: 0.0,
        },
        row2: Vec4 {
            X: 0.0,
            Y: 1.0,
            Z: 0.0,
            W: 0.0,
        },
        row3: Vec4 {
            X: 0.0,
            Y: 0.0,
            Z: 1.0,
            W: 0.0,
        },
        row4: Vec4 {
            X: 0.0,
            Y: 0.0,
            Z: 0.0,
            W: 1.0,
        },
    }
}

/// Identity matrix. (A * Identity = A)
pub const Matrix_Identity: Matrix = Matrix_IdentityValue();

pub fn Matrix_Orthographic(
    result: &mut Matrix,
    left: c_float,
    right: c_float,
    top: c_float,
    bottom: c_float,
    zNear: c_float,
    zFar: c_float,
) {
    /* Transposed, source https://msdn.microsoft.com/en-us/library/dd373965(v=vs.85).aspx */
    *result = Matrix_Identity;

    result.row1.X = 2.0 / (right - left);
    result.row2.Y = 2.0 / (top - bottom);
    result.row3.Z = -2.0 / (zFar - zNear);

    result.row4.X = -(right + left) / (right - left);
    result.row4.Y = -(top + bottom) / (top - bottom);
    result.row4.Z = -(zFar + zNear) / (zFar - zNear);
}

pub fn Matrix_PerspectiveFieldOfView(
    result: &mut Matrix,
    fovy: c_float,
    aspect: c_float,
    zNear: c_float,
    zFar: c_float,
) {
    let c = zNear * Tan_Simple(0.5 * fovy as c_double) as c_float;

    /* Transposed, source https://msdn.microsoft.com/en-us/library/dd373537(v=vs.85).aspx */
    /* For a FOV based perspective matrix, left/right/top/bottom are calculated as: */
    /* left = -c * aspect, right = c * aspect, bottom = -c, top = c */
    /* Calculations are simplified because of left/right and top/bottom symmetry */
    *result = Matrix_Identity;
    result.row4.W = 0.0;

    result.row1.X = zNear / (c * aspect);
    result.row2.Y = zNear / c;
    result.row4.Z = -(2.0 * zFar * zNear) / (zFar - zNear);
    result.row3.Z = -(zFar + zNear) / (zFar - zNear);
    result.row3.W = -1.0;
}

pub fn Matrix_LookRot(result: &mut Matrix, pos: Vec3, rot: Vec2) {
    let mut rotX = Matrix_Identity;
    let mut rotY = Matrix_Identity;
    let mut trans = Matrix_Identity;

    unsafe {
        Matrix_RotateX(&mut rotX, rot.Y);
        Matrix_RotateY(&mut rotY, rot.X);
        Matrix_Translate(&mut trans, -pos.X, -pos.Y, -pos.Z);

        Matrix_Mul(result, &rotY, &rotX);
        Matrix_Mul(result, &trans, result);
    }
}
