use std::fmt::{Formatter, Write};
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Side {
    Front,
    Back,
    Up,
    Down,
    Left,
    Right,
}

pub use Side::*;

impl Side {
    pub fn opposite(self) -> Self {
        match self {
            Front => Back,
            Back => Front,
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl std::convert::TryFrom<u8> for Side {
    type Error = ();

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x01 => Ok(Back),
            0x02 => Ok(Front),
            0x04 => Ok(Down),
            0x08 => Ok(Up),
            0x10 => Ok(Right),
            0x20 => Ok(Left),
            _ => Err(()),
        }
    }
}

impl Into<char> for Side {
    fn into(self) -> char {
        match self {
            Front => 'f',
            Back => 'b',
            Up => 'u',
            Down => 'd',
            Left => 'l',
            Right => 'r',
        }
    }
}

#[derive(Copy, Clone)]
pub struct Sides {
    bits: u8,
}

impl Sides {
    pub const MAX_VALUE: u8 = 0x20;

    pub fn contains(self, side: Side) -> bool {
        let sides: Sides = side.into();
        self.bits & sides.bits != 0
    }
}

impl PartialEq for Sides {
    fn eq(&self, other: &Self) -> bool { self.bits & 0b111111 == other.bits & 0b111111 }
}

impl Eq for Sides {}

impl std::fmt::Debug for Sides {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;

        for side in self.into_iter() {
            if self.contains(side) {
                f.write_char(side.into())?
            }
        }

        f.write_char(']')?;

        Ok(())
    }
}

impl Default for Sides {
    fn default() -> Self { Sides { bits: 0 } }
}

impl From<u8> for Sides {
    fn from(val: u8) -> Self { Sides { bits: val } }
}

impl Into<Sides> for Side {
    fn into(self) -> Sides {
        let sides = match self {
            Front => 0x01,
            Back => 0x02,
            Up => 0x04,
            Down => 0x08,
            Left => 0x10,
            Right => 0x20,
        };

        Sides { bits: sides }
    }
}

impl std::ops::BitOr<Side> for Side {
    type Output = Sides;

    fn bitor(self, rhs: Side) -> Self::Output {
        let l: Sides = self.into();
        let r: Sides = rhs.into();
        Sides { bits: l.bits | r.bits }
    }
}

impl std::ops::BitOr<Side> for Sides {
    type Output = Sides;

    fn bitor(self, rhs: Side) -> Self::Output {
        let r: Sides = rhs.into();
        Sides { bits: self.bits | r.bits }
    }
}

impl std::ops::BitOr<Sides> for Sides {
    type Output = Sides;

    fn bitor(self, rhs: Sides) -> Self::Output {
        Sides { bits: self.bits | rhs.bits }
    }
}

impl std::ops::BitAnd<Side> for Side {
    type Output = Sides;

    fn bitand(self, rhs: Side) -> Self::Output {
        let l: Sides = self.into();
        let r: Sides = rhs.into();
        Sides { bits: l.bits & r.bits }
    }
}

impl std::ops::BitAnd<Side> for Sides {
    type Output = Sides;

    fn bitand(self, rhs: Side) -> Self::Output {
        let r: Sides = rhs.into();
        Sides { bits: self.bits & r.bits }
    }
}

impl std::ops::BitAnd<Sides> for Sides {
    type Output = Sides;

    fn bitand(self, rhs: Sides) -> Self::Output {
        Sides { bits: self.bits & rhs.bits }
    }
}

impl std::ops::Not for Side {
    type Output = Sides;

    fn not(self) -> Self::Output {
        let sides: Sides = self.into();
        Sides { bits: !sides.bits }
    }
}

impl std::ops::Not for Sides {
    type Output = Sides;

    fn not(self) -> Self::Output { Sides { bits: !self.bits } }
}

pub struct SidesIterator {
    sides: Sides,
    bit: u8,
}

impl SidesIterator {
    pub fn new(sides: Sides) -> Self {
        SidesIterator { sides, bit: 1 }
    }
}

impl Iterator for SidesIterator {
    type Item = Side;

    fn next(&mut self) -> Option<Self::Item> {
        while self.bit <= Sides::MAX_VALUE {
            let side: Side = self.bit.try_into().unwrap();
            self.bit <<= 1;

            if self.sides.contains(side) { return Some(side); }
        }

        None
    }
}

impl IntoIterator for Sides {
    type Item = Side;
    type IntoIter = SidesIterator;

    fn into_iter(self) -> Self::IntoIter { SidesIterator::new(self) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        let bdr: Sides = 0b101010.into();
        assert_eq!(format!("{:?}", bdr), "[bdr]");

        let ful: Sides = 0b010101.into();
        assert_eq!(format!("{:?}", ful), "[ful]");

        let empty: Sides = 0.into();
        assert_eq!(format!("{:?}", empty), "[]");
    }

    #[test]
    fn contains() {
        let bdr: Sides = 0b101010.into();
        assert!(!bdr.contains(Front));
        assert!(bdr.contains(Back));
        assert!(!bdr.contains(Up));
        assert!(bdr.contains(Down));
        assert!(!bdr.contains(Left));
        assert!(bdr.contains(Right));
    }

    #[test]
    fn bit_ops() {
        let fb = Front | Back;
        let bu = Back | Up;
        let fbu = fb | bu;
        assert!(fbu
            .into_iter()
            .all(|s| s == Front || s == Back || s == Up)
        );

        let lrd = Left | Right | Down;
        let urd = Up | Right | Down;
        let rd = lrd & urd;
        assert_eq!(rd, Right | Down);

        let udr = Up | Down | Right;
        assert_eq!(!udr, Front | Back | Left);
        assert_eq!(!Up, Front | Back | Left | Right | Down);
    }
}
