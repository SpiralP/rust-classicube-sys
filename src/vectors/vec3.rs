use crate::{IVec3, Vec3};
use std::{
    ops::{Add, Div, Mul, Sub},
    os::raw::c_float,
};

impl Vec3 {
    #[inline]
    pub const fn new(x: c_float, y: c_float, z: c_float) -> Self {
        Self { X: x, Y: y, Z: z }
    }

    #[inline]
    pub const fn big_pos() -> Self {
        Self {
            X: 1e25 as _,
            Y: 1e25 as _,
            Z: 1e25 as _,
        }
    }

    #[inline]
    pub fn get_dir_vector(yaw_rad: c_float, pitch_rad: c_float) -> Self {
        let x = -(pitch_rad.cos()) * -(yaw_rad.sin());
        let y = -(pitch_rad.sin());
        let z = -(pitch_rad.cos()) * (yaw_rad.cos());
        Self::new(x, y, z)
    }

    /// Sets the X, Y, and Z components of a 3D vector
    #[inline]
    pub fn set(&mut self, x: c_float, y: c_float, z: c_float) {
        self.X = x;
        self.Y = y;
        self.Z = z;
    }

    /// Whether all components of a 3D vector are 0
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.X == 0.0 && self.Y == 0.0 && self.Z == 0.0
    }

    /// Returns the squared length of the vector.
    ///
    /// Squared length can be used for comparison, to avoid a costly sqrt()
    ///
    /// However, you must sqrt() this when adding lengths.
    #[inline]
    pub fn length_squared(&self) -> c_float {
        self.X * self.X + self.Y * self.Y + self.Z * self.Z
    }

    #[inline]
    pub const fn floor(&self) -> IVec3 {
        IVec3 {
            X: self.X as _,
            Y: self.Y as _,
            Z: self.Z as _,
        }
    }

    /// Negates the components of a vector.
    #[inline]
    pub fn negate(&self) -> Self {
        Self {
            X: -self.X,
            Y: -self.Y,
            Z: -self.Z,
        }
    }
}

impl Add<Self> for Vec3 {
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

impl Add<c_float> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: c_float) -> Self {
        Self {
            X: self.X + other,
            Y: self.Y + other,
            Z: self.Z + other,
        }
    }
}

impl Sub<Self> for Vec3 {
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

impl Sub<c_float> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: c_float) -> Self {
        Self {
            X: self.X - other,
            Y: self.Y - other,
            Z: self.Z - other,
        }
    }
}

impl Mul<Self> for Vec3 {
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

impl Mul<c_float> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: c_float) -> Self {
        Self {
            X: self.X * other,
            Y: self.Y * other,
            Z: self.Z * other,
        }
    }
}

impl Div<Self> for Vec3 {
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

impl Div<c_float> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, other: c_float) -> Self {
        Self {
            X: self.X / other,
            Y: self.Y / other,
            Z: self.Z / other,
        }
    }
}

impl From<IVec3> for Vec3 {
    #[inline]
    fn from(other: IVec3) -> Self {
        Self {
            X: other.X as _,
            Y: other.Y as _,
            Z: other.Z as _,
        }
    }
}
