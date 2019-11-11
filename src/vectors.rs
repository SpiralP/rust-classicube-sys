use crate::{IVec3, Vec3};

#[inline]
pub fn IVec3_Floor(a: &Vec3) -> IVec3 {
  IVec3 {
    X: a.X as _,
    Y: a.Y as _,
    Z: a.Z as _,
  }
}

#[inline]
pub fn IVec3_ToVec3(a: &IVec3) -> Vec3 {
  Vec3 {
    X: a.X as _,
    Y: a.Y as _,
    Z: a.Z as _,
  }
}

#[inline]
pub fn IVec3_Min(a: &IVec3, b: &IVec3) -> IVec3 {
  IVec3 {
    X: a.X.min(b.X),
    Y: a.Y.min(b.Y),
    Z: a.Z.min(b.Z),
  }
}

#[inline]
pub fn IVec3_Max(a: &IVec3, b: &IVec3) -> IVec3 {
  IVec3 {
    X: a.X.max(b.X),
    Y: a.Y.max(b.Y),
    Z: a.Z.max(b.Z),
  }
}

#[inline]
pub fn Vec3_GetDirVector(yaw_rad: f32, pitch_rad: f32) -> Vec3 {
  let x = -(pitch_rad.cos()) * -(yaw_rad.sin());
  let y = -(pitch_rad.sin());
  let z = -(pitch_rad.cos()) * (yaw_rad.cos());
  Vec3_Create3(x, y, z)
}

#[inline]
pub fn Vec3_Create3(x: f32, y: f32, z: f32) -> Vec3 {
  Vec3 { X: x, Y: y, Z: z }
}
