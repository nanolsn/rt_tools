use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(pub i32, pub i32, pub i32);

impl Default for Point {
    fn default() -> Self { Point(0, 0, 0) }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs }
}
