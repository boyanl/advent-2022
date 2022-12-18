use std::ops::{self, Add, AddAssign, Mul, Neg, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Point3<T> = Vec3<T>;

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        return Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        return Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Mul<i32, Output = T>> ops::Mul<i32> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: i32) -> Self::Output {
        return Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl<T: Neg<Output = T>> ops::Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        return Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl<T> std::convert::From<(T, T, T)> for Vec3<T> {
    fn from(tuple: (T, T, T)) -> Self {
        return Vec3 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        };
    }
}
