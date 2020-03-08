use std::iter::FromIterator;
use super::{
    sides::*,
    parse::{Parse, point::yaml_to_point},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(pub i32, pub i32, pub i32);

impl Point {
    pub fn distance(self) -> f32 {
        let s = (self.0.pow(2) + self.1.pow(2) + self.2.pow(2)) as f32;
        s.sqrt()
    }

    pub fn distance_between(self, other: Self) -> f32 { (self - other).distance() }

    pub fn modulo(self, m: i32) -> Self {
        let x = self.0 % m;
        let y = self.1 % m;
        let z = self.2 % m;

        Point(
            if x < 0 { x + m } else { x },
            if y < 0 { y + m } else { y },
            if z < 0 { z + m } else { z },
        )
    }

    pub fn zero() -> Self { Point(0, 0, 0) }

    pub fn front() -> Self { Point(0, 0, 1) }
    pub fn back() -> Self { Point(0, 0, -1) }
    pub fn up() -> Self { Point(0, 1, 0) }
    pub fn down() -> Self { Point(0, -1, 0) }
    pub fn left() -> Self { Point(1, 0, 0) }
    pub fn right() -> Self { Point(-1, 0, 0) }

    pub fn to_front(self) -> Self { self + Self::front() }
    pub fn to_back(self) -> Self { self + Self::back() }
    pub fn to_up(self) -> Self { self + Self::up() }
    pub fn to_down(self) -> Self { self + Self::down() }
    pub fn to_left(self) -> Self { self + Self::left() }
    pub fn to_right(self) -> Self { self + Self::right() }

    pub fn to(self, side: Side) -> Self { self + side.into() }
}

impl Default for Point {
    fn default() -> Self { Self::zero() }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl From<[f32; 3]> for Point {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Point(
            if x < 0. { x - 1. } else { x } as i32,
            if y < 0. { y - 1. } else { y } as i32,
            if z < 0. { z - 1. } else { z } as i32,
        )
    }
}

impl From<(f32, f32, f32)> for Point {
    fn from((x, y, z): (f32, f32, f32)) -> Self { Point::from([x, y, z]) }
}

impl From<glm::Vec3> for Point {
    fn from(vec: glm::Vec3) -> Self { Point::from([vec.x, vec.y, vec.z]) }
}

impl From<Side> for Point {
    fn from(side: Side) -> Self {
        match side {
            Front => Self::front(),
            Back => Self::back(),
            Up => Self::up(),
            Down => Self::down(),
            Left => Self::left(),
            Right => Self::right(),
        }
    }
}

impl FromIterator<i32> for Point {
    fn from_iter<T: IntoIterator<Item=i32>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().unwrap_or(0);
        let y = iter.next().unwrap_or(0);
        let z = iter.next().unwrap_or(0);
        Point(x, y, z)
    }
}

impl Parse for Point {
    type DataError = ();

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> { yaml_to_point(yml) }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, Point(x, y, z): Self) -> Self { Point(self.0 + x, self.1 + y, self.2 + z) }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, Point(x, y, z): Self) -> Self { Point(self.0 - x, self.1 - y, self.2 - z) }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let p = Point(-1, 0, 0);

        assert_eq!(p.distance(), 1.0);
        assert_eq!(p.distance_between(p), 0.0);
        assert_eq!(p.distance_between(Point(1, 0, 0)), 2.0);
    }

    #[test]
    fn modulo() {
        let p = Point(12, 5, -1);
        assert_eq!(p.modulo(5), Point(2, 0, 4));
    }

    #[test]
    fn to() {
        let p = Point::zero()
            .to_up()
            .to_down()
            .to_front()
            .to_right();

        assert_eq!(p, Point(-1, 0, 1));
        assert_eq!(p, Point::zero().to(Front).to(Right));
    }

    #[test]
    fn from() {
        let p = Point::from((1., 0.5, 0.1));
        assert_eq!(p, Point(1, 0, 0));

        let p = Point::from((-0.1, 0., 0.1));
        assert_eq!(p, Point(-1, 0, 0));

        let p = Point::from((-12.1, 3., 5.1));
        assert_eq!(p, Point(-13, 3, 5));

        let p = Point::from(Up);
        assert_eq!(p, Point(0, 1, 0));
    }
}
