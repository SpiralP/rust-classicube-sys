use crate::{IVec3, Vec3};
use std::{
  ops::{Add, Div, Mul, Sub},
  os::raw::c_int,
};

impl IVec3 {
  #[inline]
  pub const fn max_value() -> Self {
    Self {
      X: c_int::max_value(),
      Y: c_int::max_value(),
      Z: c_int::max_value(),
    }
  }

  #[inline]
  pub fn min(&self, other: &Self) -> Self {
    Self {
      X: self.X.min(other.X),
      Y: self.Y.min(other.Y),
      Z: self.Z.min(other.Z),
    }
  }

  #[inline]
  pub fn max(&self, other: &Self) -> Self {
    Self {
      X: self.X.max(other.X),
      Y: self.Y.max(other.Y),
      Z: self.Z.max(other.Z),
    }
  }
}

impl Add<Self> for IVec3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    Self {
      X: self.X + other.X,
      Y: self.Y + other.Y,
      Z: self.Z + other.Z,
    }
  }
}

impl Add<c_int> for IVec3 {
  type Output = Self;

  #[inline]
  fn add(self, other: c_int) -> Self {
    Self {
      X: self.X + other,
      Y: self.Y + other,
      Z: self.Z + other,
    }
  }
}

impl Sub<Self> for IVec3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    Self {
      X: self.X - other.X,
      Y: self.Y - other.Y,
      Z: self.Z - other.Z,
    }
  }
}

impl Sub<c_int> for IVec3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: c_int) -> Self {
    Self {
      X: self.X - other,
      Y: self.Y - other,
      Z: self.Z - other,
    }
  }
}

impl Mul<Self> for IVec3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    Self {
      X: self.X * other.X,
      Y: self.Y * other.Y,
      Z: self.Z * other.Z,
    }
  }
}

impl Mul<c_int> for IVec3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: c_int) -> Self {
    Self {
      X: self.X * other,
      Y: self.Y * other,
      Z: self.Z * other,
    }
  }
}

impl Div<Self> for IVec3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    Self {
      X: self.X / other.X,
      Y: self.Y / other.Y,
      Z: self.Z / other.Z,
    }
  }
}

impl Div<c_int> for IVec3 {
  type Output = Self;

  #[inline]
  fn div(self, other: c_int) -> Self {
    Self {
      X: self.X / other,
      Y: self.Y / other,
      Z: self.Z / other,
    }
  }
}

impl From<Vec3> for IVec3 {
  #[inline]
  fn from(other: Vec3) -> Self {
    Self {
      X: other.X as _,
      Y: other.Y as _,
      Z: other.Z as _,
    }
  }
}
