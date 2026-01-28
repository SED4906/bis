
use std::ops::{Mul, Neg};

use crate::math::{
    Vector,
    vector::{X, Y, Z},
};

pub type Vector3 = Vector<3, f32>;

#[derive(Clone, Copy)]
pub struct Quaternion {
    pub r: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn rotate(&self, q: Quaternion) -> Vector3 {
        (-q * (*self).into() * q).into()
    }
}

impl From<Vector3> for Quaternion {
    fn from(v: Vector3) -> Self {
        Quaternion {
            r: 0.0,
            x: v.x(),
            y: v.y(),
            z: v.z(),
        }
    }
}

impl From<Quaternion> for Vector3 {
    fn from(q: Quaternion) -> Self {
        Vector3::from([q.x, q.y, q.z])
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.r * rhs.x + self.x * rhs.r - self.y * rhs.z + self.z * rhs.y,
            y: self.r * rhs.y + self.x * rhs.z + self.y * rhs.r - self.z * rhs.x,
            z: self.r * rhs.z - self.x * rhs.y + self.y * rhs.x + self.z * rhs.r,
        }
    }
}

impl Neg for Quaternion {
    type Output = Quaternion;

    fn neg(self) -> Self::Output {
        let Quaternion { r, x, y, z } = self;
        Self {
            r,
            x: -x,
            y: -y,
            z: -z,
        }
    }
}
