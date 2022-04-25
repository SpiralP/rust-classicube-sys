mod ivec;
mod matrix;
mod vec;

pub use self::{ivec::*, matrix::*, vec::*};
use crate::{bindings::*, std_types::c_double};

/// Sets the X, Y, and Z components of a 3D vector
#[macro_export]
macro_rules! Vec3_Set {
    ($v:expr, $x:expr, $y:expr, $z:expr) => {
        $v.X = $x;
        $v.Y = $y;
        $v.Z = $z;
    };
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

pub fn Tan_Simple(x: c_double) -> c_double {
    unsafe { Math_Sin(x) / Math_Cos(x) }
}
