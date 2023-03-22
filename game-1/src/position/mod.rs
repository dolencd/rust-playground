use std::{
    ops::{Add, Mul},
};

#[derive(Default, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position(pub i32, pub i32);

impl Add for Position {
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
        }
    }
    type Output = Position;
}

impl Mul<i32> for Position {
    type Output = Position;
    fn mul(self, rhs: i32) -> Self::Output {
        Position(self.0 * rhs, self.1 * rhs)
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Down => Position(0, -1),
            Direction::Left => Position(-1, 0),
            Direction::Up => Position(0, 1),
            Direction::Right => Position(1, 0),
        }
    }
}
