use super::OwnedString;
use crate::bindings::*;
use std::{mem, os::raw::c_float};

#[allow(clippy::missing_safety_doc)]
pub unsafe fn Entity_Init(e: &mut Entity) {
    let model = OwnedString::new("humanoid");
    e.ModelScale.set(1.0, 1.0, 1.0);
    e.uScale = 1.0;
    e.vScale = 1.0;
    e._skinReqID = 0;
    e.SkinRaw[0] = 0;
    e.NameRaw[0] = 0;
    Entity_SetModel(e, model.as_cc_string());
}

/// Clamps the given angle so it lies between [0, 360).
pub fn LocationUpdate_Clamp(mut degrees: c_float) -> c_float {
    while degrees >= 360.0 {
        degrees -= 360.0;
    }
    while degrees < 0.0 {
        degrees += 360.0;
    }
    degrees
}

/// Makes a location update only containing yaw and pitch.
pub fn LocationUpdate_MakeOri(update: &mut LocationUpdate, yaw: c_float, pitch: c_float) {
    *update = unsafe { mem::zeroed() };
    update.Flags = LOCATIONUPDATE_PITCH as u8 | LOCATIONUPDATE_YAW as u8;
    update.Pitch = LocationUpdate_Clamp(pitch);
    update.Yaw = LocationUpdate_Clamp(yaw);
}

/// Makes a location update only containing position
pub fn LocationUpdate_MakePos(update: &mut LocationUpdate, pos: Vec3, rel: cc_bool) {
    *update = unsafe { mem::zeroed() };
    update.Flags = LOCATIONUPDATE_POS as u8;
    update.Pos = pos;
    update.RelativePos = rel;
}

/// Makes a location update containing position, yaw and pitch.
pub fn LocationUpdate_MakePosAndOri(
    update: &mut LocationUpdate,
    pos: Vec3,
    yaw: c_float,
    pitch: c_float,
    rel: cc_bool,
) {
    *update = unsafe { mem::zeroed() };
    update.Flags = LOCATIONUPDATE_POS as u8 | LOCATIONUPDATE_PITCH as u8 | LOCATIONUPDATE_YAW as u8;
    update.Pitch = LocationUpdate_Clamp(pitch);
    update.Yaw = LocationUpdate_Clamp(yaw);
    update.Pos = pos;
    update.RelativePos = rel;
}
