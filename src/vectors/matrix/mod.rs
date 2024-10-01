mod ops;

use crate::{
    bindings::*,
    std_types::{c_double, c_float},
    Tan_Simple,
};

impl Matrix {
    pub const IDENTITY: Self = Matrix_IdentityValue();

    pub const fn identity_value() -> Self {
        Matrix_IdentityValue()
    }

    /// Returns a matrix representing a counter-clockwise rotation around x axis.
    pub fn rotate_x(angle: c_float) -> Self {
        let mut result = Self::IDENTITY;
        unsafe {
            Matrix_RotateX(&mut result, angle);
        }
        result
    }

    /// Returns a matrix representing a counter-clockwise rotation around y axis.
    pub fn rotate_y(angle: c_float) -> Self {
        let mut result = Self::IDENTITY;
        unsafe {
            Matrix_RotateY(&mut result, angle);
        }
        result
    }

    /// Returns a matrix representing a counter-clockwise rotation around z axis.
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
        unsafe {
            Matrix_Translate(&mut result, x, y, z);
        }
        result
    }

    /// Returns a matrix representing a scaling by the given factors.
    pub fn scale(x: c_float, y: c_float, z: c_float) -> Self {
        let mut result = Self::IDENTITY;
        unsafe {
            Matrix_Scale(&mut result, x, y, z);
        }
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
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        row2: Vec4 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        },
        row3: Vec4 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        },
        row4: Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
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

    result.row1.x = 2.0 / (right - left);
    result.row2.y = 2.0 / (top - bottom);
    result.row3.z = -2.0 / (zFar - zNear);

    result.row4.x = -(right + left) / (right - left);
    result.row4.y = -(top + bottom) / (top - bottom);
    result.row4.z = -(zFar + zNear) / (zFar - zNear);
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
    result.row4.w = 0.0;

    result.row1.x = zNear / (c * aspect);
    result.row2.y = zNear / c;
    result.row4.z = -(2.0 * zFar * zNear) / (zFar - zNear);
    result.row3.z = -(zFar + zNear) / (zFar - zNear);
    result.row3.w = -1.0;
}

pub fn Matrix_LookRot(result: &mut Matrix, pos: Vec3, rot: Vec2) {
    let mut rotX = Matrix_Identity;
    let mut rotY = Matrix_Identity;
    let mut trans = Matrix_Identity;

    unsafe {
        Matrix_RotateX(&mut rotX, rot.y);
        Matrix_RotateY(&mut rotY, rot.x);
        Matrix_Translate(&mut trans, -pos.x, -pos.y, -pos.z);

        Matrix_Mul(result, &rotY, &rotX);
        Matrix_Mul(result, &trans, result);
    }
}
