use glm::{vec3, Vec3};

use super::{
    axis::Axis,
    sides::*,
    parse::ParseYaml,
};

pub trait ShellTransform {
    fn flip(&mut self, axis: Axis) -> &mut Self;

    fn turn_counter_clockwise(&mut self, axis: Axis) -> &mut Self;

    fn turn_clockwise(&mut self, axis: Axis) -> &mut Self;

    fn turn(&mut self, axis: Axis, counter_clockwise: bool) -> &mut Self {
        if counter_clockwise {
            self.turn_counter_clockwise(axis)
        } else {
            self.turn_clockwise(axis)
        }
    }
}

pub fn apply_action<S>(shell: &mut S, action: ShellTransformAction) -> &mut S
    where
        S: ShellTransform,
{
    match action {
        Flip(ax) => shell.flip(ax),
        TurnCounterClockwise(ax) => shell.turn_counter_clockwise(ax),
        TurnClockwise(ax) => shell.turn_clockwise(ax),
    }
}

pub fn apply_actions<S, I>(shell: &mut S, actions: I) -> &mut S
    where
        S: ShellTransform,
        I: IntoIterator<Item=ShellTransformAction>,
{
    for act in actions {
        apply_action(shell, act);
    }

    shell
}

impl ShellTransform for Vec3 {
    fn flip(&mut self, axis: Axis) -> &mut Self {
        *self = match axis {
            Axis::X => vec3(-self.x, self.y, self.z),
            Axis::Y => vec3(self.x, -self.y, self.z),
            Axis::Z => vec3(self.x, self.y, -self.z),
        };

        self
    }

    fn turn_counter_clockwise(&mut self, axis: Axis) -> &mut Self {
        *self = match axis {
            Axis::X => vec3(self.x, -self.z, self.y),
            Axis::Y => vec3(self.z, self.y, -self.x),
            Axis::Z => vec3(-self.y, self.x, self.z),
        };

        self
    }

    fn turn_clockwise(&mut self, axis: Axis) -> &mut Self {
        *self = match axis {
            Axis::X => vec3(self.x, self.z, -self.y),
            Axis::Y => vec3(-self.z, self.y, self.x),
            Axis::Z => vec3(self.y, -self.x, self.z),
        };

        self
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shell {
    front: Side,
    back: Side,
    up: Side,
    down: Side,
    left: Side,
    right: Side,
}

impl Shell {
    pub fn new() -> Self { Default::default() }

    pub fn local_front(&self) -> Side { self.front }
    pub fn local_back(&self) -> Side { self.back }
    pub fn local_up(&self) -> Side { self.up }
    pub fn local_down(&self) -> Side { self.down }
    pub fn local_left(&self) -> Side { self.left }
    pub fn local_right(&self) -> Side { self.right }

    pub fn local_side(&self, side: Side) -> Side {
        match side {
            Front => self.front,
            Back => self.back,
            Up => self.up,
            Down => self.down,
            Left => self.left,
            Right => self.right,
        }
    }
}

impl ShellTransform for Shell {
    fn flip(&mut self, axis: Axis) -> &mut Self {
        let Shell { front, back, up, down, left, right } = *self;

        match axis {
            Axis::X => {
                self.right = left;
                self.left = right;
            }
            Axis::Y => {
                self.up = down;
                self.down = up;
            }
            Axis::Z => {
                self.back = front;
                self.front = back;
            }
        };

        self
    }

    fn turn_counter_clockwise(&mut self, axis: Axis) -> &mut Self {
        let Shell { front, back, up, down, left, right } = *self;

        match axis {
            Axis::X => {
                self.front = up;
                self.down = front;
                self.back = down;
                self.up = back;
            }
            Axis::Y => {
                self.left = front;
                self.back = left;
                self.right = back;
                self.front = right;
            }
            Axis::Z => {
                self.right = up;
                self.down = right;
                self.left = down;
                self.up = left;
            }
        }

        self
    }

    fn turn_clockwise(&mut self, axis: Axis) -> &mut Self {
        let Shell { front, back, up, down, left, right } = *self;

        match axis {
            Axis::X => {
                self.front = down;
                self.down = back;
                self.back = up;
                self.up = front;
            }
            Axis::Y => {
                self.left = back;
                self.back = right;
                self.right = front;
                self.front = left;
            }
            Axis::Z => {
                self.right = down;
                self.down = left;
                self.left = up;
                self.up = right;
            }
        }

        self
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            front: Front,
            back: Back,
            up: Up,
            down: Down,
            left: Left,
            right: Right,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShellTransformAction {
    Flip(Axis),
    TurnCounterClockwise(Axis),
    TurnClockwise(Axis),
}

use ShellTransformAction::*;

impl std::convert::TryFrom<&str> for ShellTransformAction {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use std::convert::TryInto;

        if s.len() != "turn_-?".len() && s.len() != "turn_?".len() { Err(())? }

        let last = s.chars().rev().next().ok_or(())?;

        Ok(match s {
            _ if s.starts_with("flip_") => Flip(last.try_into()?),
            _ if s.starts_with("turn_-") => TurnClockwise(last.try_into()?),
            _ if s.starts_with("turn_") => TurnCounterClockwise(last.try_into()?),
            _ => Err(())?,
        })
    }
}

impl std::fmt::Display for ShellTransformAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Flip(ax) => write!(f, "flip_{}", ax),
            TurnCounterClockwise(ax) => write!(f, "turn_{}", ax),
            TurnClockwise(ax) => write!(f, "turn_-{}", ax),
        }
    }
}

impl ParseYaml for ShellTransformAction {
    type DataError = ();

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        use std::convert::TryInto;
        yml.as_str().ok_or(())?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_vec3() {
        let mut v = vec3(0., 1., -1.);
        v.flip(Axis::Y);
        assert_eq!(v, vec3(0., -1., -1.));
    }

    #[test]
    fn turn_vec3() {
        let mut v = vec3(0., 1., -1.);
        v.turn(Axis::Y, true);
        assert_eq!(v, vec3(-1., 1., 0.));
    }

    #[test]
    fn flip_shell() {
        let mut s = Shell::new();
        s
            .flip(Axis::X)
            .flip(Axis::X)
            .flip(Axis::Y)
            .flip(Axis::Z);

        assert_eq!(s.front, Back);
        assert_eq!(s.back, Front);
        assert_eq!(s.up, Down);
        assert_eq!(s.down, Up);
        assert_eq!(s.left, Left);
        assert_eq!(s.right, Right);

        let mut s = Shell::new();
        s
            .flip(Axis::X)
            .flip(Axis::Y)
            .flip(Axis::Z);

        assert_eq!(s.front, Back);
        assert_eq!(s.back, Front);
        assert_eq!(s.up, Down);
        assert_eq!(s.down, Up);
        assert_eq!(s.left, Right);
        assert_eq!(s.right, Left);

        let mut s = Shell::new();
        s
            .flip(Axis::X)
            .flip(Axis::Y)
            .flip(Axis::Z)
            .flip(Axis::X)
            .flip(Axis::Y)
            .flip(Axis::Z);

        assert_eq!(s, Shell::new());
    }

    #[test]
    fn turn_shell() {
        let mut s = Shell::new();
        s
            .turn_counter_clockwise(Axis::X)
            .turn_clockwise(Axis::X);

        assert_eq!(s, Shell::new());

        let mut s = Shell::new();
        s
            .turn_clockwise(Axis::X)
            .turn_clockwise(Axis::X)
            .turn_clockwise(Axis::X)
            .turn_clockwise(Axis::X);

        assert_eq!(s, Shell::new());

        let mut s = Shell::new();
        s
            .turn_counter_clockwise(Axis::Y)
            .turn_counter_clockwise(Axis::X);

        assert_eq!(s.front, Up);
        assert_eq!(s.back, Down);
        assert_eq!(s.up, Left);
        assert_eq!(s.down, Right);
        assert_eq!(s.left, Front);
        assert_eq!(s.right, Back);

        let mut s = Shell::new();
        s
            .turn_counter_clockwise(Axis::X)
            .turn_counter_clockwise(Axis::Z);

        assert_eq!(s.front, Up);
        assert_eq!(s.back, Down);
        assert_eq!(s.up, Left);
        assert_eq!(s.down, Right);
        assert_eq!(s.left, Front);
        assert_eq!(s.right, Back);

        let mut s = Shell::new();
        s
            .turn_counter_clockwise(Axis::Z)
            .turn_counter_clockwise(Axis::Z)
            .turn_counter_clockwise(Axis::Z);

        assert_eq!(s, *Shell::new().turn_clockwise(Axis::Z));
    }

    #[test]
    fn display() {
        let a = format!("{}", Flip(Axis::X));
        assert_eq!(a, "flip_x");

        let a = format!("{}", TurnCounterClockwise(Axis::Y));
        assert_eq!(a, "turn_y");

        let a = format!("{}", TurnClockwise(Axis::Z));
        assert_eq!(a, "turn_-z");
    }

    #[test]
    fn try_from() {
        use std::convert::TryFrom;

        let a = ShellTransformAction::try_from(".");
        assert_eq!(a, Err(()));

        let a = ShellTransformAction::try_from("wwww_x");
        assert_eq!(a, Err(()));

        let a = ShellTransformAction::try_from("flip_x");
        assert_eq!(a, Ok(Flip(Axis::X)));

        let a = ShellTransformAction::try_from("turn_y");
        assert_eq!(a, Ok(TurnCounterClockwise(Axis::Y)));

        let a = ShellTransformAction::try_from("turn_-z");
        assert_eq!(a, Ok(TurnClockwise(Axis::Z)));
    }
}
