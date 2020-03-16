#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl std::convert::TryFrom<char> for Axis {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'x' => Ok(Axis::X),
            'y' => Ok(Axis::Y),
            'z' => Ok(Axis::Z),
            _ => Err(()),
        }
    }
}

impl Into<char> for Axis {
    fn into(self) -> char {
        match self {
            Axis::X => 'x',
            Axis::Y => 'y',
            Axis::Z => 'z',
        }
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char((*self).into())
    }
}
