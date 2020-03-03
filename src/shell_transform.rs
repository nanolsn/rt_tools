use super::{
    sides::*,
    axis::Axis,
};

pub trait ShellTransform
    where
        Self: Sized,
{
    fn flip(self, axis: Axis) -> Self;

    fn turn_counter_clockwise(self, axis: Axis) -> Self;

    fn turn_clockwise(self, axis: Axis) -> Self;

    fn turn(self, axis: Axis, counter_clockwise: bool) -> Self {
        if counter_clockwise {
            self.turn_counter_clockwise(axis)
        } else {
            self.turn_clockwise(axis)
        }
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
    fn flip(self, axis: Axis) -> Self {
        let Shell { front, back, up, down, left, right } = self;

        match axis {
            Axis::X => Shell { right: left, left: right, ..self },
            Axis::Y => Shell { up: down, down: up, ..self },
            Axis::Z => Shell { back: front, front: back, ..self },
        }
    }

    fn turn_counter_clockwise(self, axis: Axis) -> Self {
        let Shell { front, back, up, down, left, right } = self;

        match axis {
            Axis::X => Shell { front: up, down: front, back: down, up: back, ..self },
            Axis::Y => Shell { left: front, back: left, right: back, front: right, ..self },
            Axis::Z => Shell { right: up, down: right, left: down, up: left, ..self },
        }
    }

    fn turn_clockwise(self, axis: Axis) -> Self {
        let Shell { front, back, up, down, left, right } = self;

        match axis {
            Axis::X => Shell { front: down, down: back, back: up, up: front, ..self },
            Axis::Y => Shell { left: back, back: right, right: front, front: left, ..self },
            Axis::Z => Shell { right: down, down: left, left: up, up: right, ..self },
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip() {
        let s = Shell::new()
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

        let s = Shell::new()
            .flip(Axis::X)
            .flip(Axis::Y)
            .flip(Axis::Z);

        assert_eq!(s.front, Back);
        assert_eq!(s.back, Front);
        assert_eq!(s.up, Down);
        assert_eq!(s.down, Up);
        assert_eq!(s.left, Right);
        assert_eq!(s.right, Left);

        let s = Shell::new()
            .flip(Axis::X)
            .flip(Axis::Y)
            .flip(Axis::Z)
            .flip(Axis::X)
            .flip(Axis::Y)
            .flip(Axis::Z);

        assert_eq!(s, Shell::new());
    }

    #[test]
    fn turn() {
        let s = Shell::new()
            .turn_counter_clockwise(Axis::X)
            .turn_clockwise(Axis::X);

        assert_eq!(s, Shell::new());

        let s = Shell::new()
            .turn_clockwise(Axis::X)
            .turn_clockwise(Axis::X)
            .turn_clockwise(Axis::X)
            .turn_clockwise(Axis::X);

        assert_eq!(s, Shell::new());

        let s = Shell::new()
            .turn_counter_clockwise(Axis::Y)
            .turn_counter_clockwise(Axis::X);

        assert_eq!(s.front, Up);
        assert_eq!(s.back, Down);
        assert_eq!(s.up, Left);
        assert_eq!(s.down, Right);
        assert_eq!(s.left, Front);
        assert_eq!(s.right, Back);

        let s = Shell::new()
            .turn_counter_clockwise(Axis::X)
            .turn_counter_clockwise(Axis::Z);

        assert_eq!(s.front, Up);
        assert_eq!(s.back, Down);
        assert_eq!(s.up, Left);
        assert_eq!(s.down, Right);
        assert_eq!(s.left, Front);
        assert_eq!(s.right, Back);

        let s = Shell::new()
            .turn_counter_clockwise(Axis::Z)
            .turn_counter_clockwise(Axis::Z)
            .turn_counter_clockwise(Axis::Z);

        assert_eq!(s, Shell::new().turn_clockwise(Axis::Z));
    }
}
