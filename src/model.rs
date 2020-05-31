use crate::bindings::*;
use std::os::raw::c_float;

// pub const MODEL_QUAD_VERTICES: u32 = 4;
pub const MODEL_BOX_VERTICES: u32 = FACE_CONSTS_FACE_COUNT as u32 * MODEL_QUAD_VERTICES as u32;

#[macro_export]
macro_rules! BoxDesc_Dim {
    ($p1:expr, $p2:expr) => {
        {
            if $p1 < $p2 {
                $p2 - $p1
            } else {
                $p1 - $p2
            }
        } as u8
    };
}

// Macros for making initialising a BoxDesc easier to understand. See Model.c for how these get used.

#[macro_export]
macro_rules! BoxDesc_Tex {
    ($x:expr, $y:expr) => {
        ($x, $y)
    };
}

/// gives (x, y, z)
#[macro_export]
macro_rules! BoxDesc_Dims {
    ($x1:expr, $y1:expr, $z1:expr, $x2:expr, $y2:expr, $z2:expr) => {
        (
            $crate::BoxDesc_Dim!($x1, $x2),
            $crate::BoxDesc_Dim!($y1, $y2),
            $crate::BoxDesc_Dim!($z1, $z2),
        )
    };
}

/// gives (x1, y1, z1, x2, y2, z2)
#[macro_export]
macro_rules! BoxDesc_Bounds {
    ($x1:expr, $y1:expr, $z1:expr, $x2:expr, $y2:expr, $z2:expr) => {
        (
            $x1 as ::std::os::raw::c_float / 16.0,
            $y1 as ::std::os::raw::c_float / 16.0,
            $z1 as ::std::os::raw::c_float / 16.0,
            $x2 as ::std::os::raw::c_float / 16.0,
            $y2 as ::std::os::raw::c_float / 16.0,
            $z2 as ::std::os::raw::c_float / 16.0,
        )
    };
}

/// gives (x, y, z)
#[macro_export]
macro_rules! BoxDesc_Rot {
    ($x:expr, $y:expr, $z:expr) => {
        ($x / 16.0, $y / 16.0, $z / 16.0)
    };
}

/// gives ((x, y, z), (x1, y1, z1, x2, y2, z2))
#[macro_export]
macro_rules! BoxDesc_Box {
    ($x1:expr, $y1:expr, $z1:expr, $x2:expr, $y2:expr, $z2:expr) => {
        (
            $crate::BoxDesc_Dims!($x1, $y1, $z1, $x2, $y2, $z2),
            $crate::BoxDesc_Bounds!($x1, $y1, $z1, $x2, $y2, $z2),
        )
    };
}

type BoxDesc_Dims_Return = (u8, u8, u8);
type BoxDesc_Bounds_Return = (c_float, c_float, c_float, c_float, c_float, c_float);
impl BoxDesc {
    pub fn from_macros(
        (texX, texY): (u16, u16),
        ((sizeX, sizeY, sizeZ), (x1, y1, z1, x2, y2, z2)): (
            BoxDesc_Dims_Return,
            BoxDesc_Bounds_Return,
        ),
    ) -> Self {
        Self {
            texX,
            texY,
            sizeX,
            sizeY,
            sizeZ,
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
            rotX: 0.0,
            rotY: 0.0,
            rotZ: 0.0,
        }
    }
}

#[test]
fn test_model_macros() {
    fn BoxDesc_BuildBox(_part: *mut ModelPart, desc: *const BoxDesc) {
        unsafe {
            println!("{:#?}", *desc);
        }
    }

    let mut part: ModelPart = unsafe { std::mem::zeroed() };
    BoxDesc_BuildBox(
        &mut part,
        &BoxDesc::from_macros(BoxDesc_Tex!(0, 16), BoxDesc_Box!(-3, 1, -3, 3, 7, 3)),
    );
}
