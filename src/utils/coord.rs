use std::ops::Add;

use super::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

impl Coord {
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, rhs: Direction) -> Self::Output {
        let tmp = rhs.lookahead();
        self + tmp
    }
}
