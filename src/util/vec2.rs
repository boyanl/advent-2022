use std::ops;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

pub type Point2 = Vec2;

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        return Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        };
    }
}
impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul<i32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: i32) -> Self::Output {
        return Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Vec2_64 {
    pub x: i64,
    pub y: i64,
}

pub type Point2_64 = Vec2_64;

impl ops::Add<Vec2_64> for Vec2_64 {
    type Output = Vec2_64;

    fn add(self, _rhs: Vec2_64) -> Vec2_64 {
        return Vec2_64 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        };
    }
}
impl ops::AddAssign<Vec2_64> for Vec2_64 {
    fn add_assign(&mut self, rhs: Vec2_64) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Add<Vec2> for Vec2_64 {
    type Output = Vec2_64;

    fn add(self, rhs: Vec2) -> Vec2_64 {
        return Vec2_64 {
            x: self.x + rhs.x as i64,
            y: self.y + rhs.y as i64,
        };
    }
}
impl ops::AddAssign<Vec2> for Vec2_64 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x as i64;
        self.y += rhs.y as i64;
    }
}

impl ops::Mul<i64> for Vec2_64 {
    type Output = Vec2_64;
    fn mul(self, rhs: i64) -> Self::Output {
        return Vec2_64 {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}
