use std::ops::{self, Add, AddAssign, Mul};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub type Point2<T> = Vec2<T>;

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        return Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Mul<i32, Output = T>> ops::Mul<i32> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: i32) -> Self::Output {
        return Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl<T> std::convert::From<(T, T)> for Vec2<T> {
    fn from(tuple: (T, T)) -> Self {
        return Vec2 {
            x: tuple.0,
            y: tuple.1,
        };
    }
}
