use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, Sub},
};

use super::{direction::Direction, Unit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord<U: Unit> {
    pub x: U,
    pub y: U,
}

impl<U: Unit> Coord<U> {
    #[inline(always)]
    pub const fn new(x: U, y: U) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn surround(&self) -> [Self; 4] {
        [
            Self::new(self.x - U::from(1), self.y),
            Self::new(self.x, self.y + U::from(1)),
            Self::new(self.x + U::from(1), self.y),
            Self::new(self.x, self.y - U::from(1)),
        ]
    }
}

impl<U: Unit> Add for Coord<U> {
    type Output = Coord<U>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<U: Unit> Sub for Coord<U> {
    type Output = Coord<U>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<U: Unit> Add<Direction> for Coord<U> {
    type Output = Coord<U>;

    fn add(self, rhs: Direction) -> Self::Output {
        let tmp = rhs.lookahead();
        self + tmp
    }
}
