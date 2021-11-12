mod ops;

use crate::bindings::*;
use std::os::raw::{c_double, c_float};

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

pub fn Tan_Simple(x: c_double) -> c_double {
    unsafe { Math_Sin(x) / Math_Cos(x) }
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
