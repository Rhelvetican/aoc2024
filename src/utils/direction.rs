use std::ops::{Neg, Not};

use super::{coord::Coord, Unit};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    pub fn turn_right(&mut self) {
        match self {
            Self::Up => *self = Self::Right,
            Self::Right => *self = Self::Down,
            Self::Down => *self = Self::Left,
            Self::Left => *self = Self::Up,
        }
    }

    pub fn turn_left(&mut self) {
        match self {
            Self::Up => *self = Self::Left,
            Self::Left => *self = Self::Down,
            Self::Down => *self = Self::Right,
            Self::Right => *self = Self::Up,
        }
    }

    pub fn turn_back(&mut self) {
        *self = !*self
    }

    pub fn lookahead<U: Unit>(&self) -> Coord<U> {
        match self {
            Self::Up => Coord::new(0.into(), (-1).into()),
            Self::Right => Coord::new(1.into(), 0.into()),
            Self::Down => Coord::new(0.into(), 1.into()),
            Self::Left => Coord::new((-1).into(), 0.into()),
        }
    }
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
        }
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        !self
    }
}
