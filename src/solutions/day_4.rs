use std::{collections::HashMap, fs::read_to_string, path::Path};

use super::AocSolution;
use crate::utils::{coord::Coord, Result};

const DIRS: [Coord<i16>; 8] = [
    Coord::new(1, 0),
    Coord::new(1, -1),
    Coord::new(0, -1),
    Coord::new(-1, -1),
    Coord::new(-1, 0),
    Coord::new(-1, 1),
    Coord::new(0, 1),
    Coord::new(1, 1),
];

struct XmasGrid {
    pub grid: HashMap<Coord<i16>, char>,
}

impl XmasGrid {
    fn new(src: &str) -> Self {
        Self {
            grid: src
                .lines()
                .zip(0..)
                .flat_map(|(l, y)| l.chars().zip(0..).map(move |(c, x)| (Coord::new(x, y), c)))
                .collect(),
        }
    }

    fn find_xmas(&self, pos: Coord<i16>) -> usize {
        let mut tmp = 0;
        for dir in DIRS {
            let mut found = true;

            for nxt in ['M', 'A', 'S'] {
                let nxt_pos = pos + dir;

                if self.grid.get(&nxt_pos) != Some(&nxt) {
                    found = false;
                    break;
                }
            }

            if found {
                tmp += 1;
            }
        }

        tmp
    }

    fn find_x_mas(&self, pos: Coord<i16>) -> usize {
        let (x, y) = (pos.x, pos.y);

        let tr = self.grid.get(&Coord::new(x + 1, y + 1));
        let br = self.grid.get(&Coord::new(x + 1, y - 1));
        let bl = self.grid.get(&Coord::new(x - 1, y - 1));
        let tl = self.grid.get(&Coord::new(x - 1, y + 1));

        match (tr, bl, tl, br) {
            (Some('S'), Some('M'), Some('M'), Some('S')) => 1,
            (Some('M'), Some('S'), Some('S'), Some('M')) => 1,
            (Some('S'), Some('M'), Some('S'), Some('M')) => 1,
            (Some('M'), Some('S'), Some('M'), Some('S')) => 1,
            _ => 0,
        }
    }
}

pub struct AocDayFourSolution;

impl AocSolution for AocDayFourSolution {
    type Output = u64;

    fn get_input(&self, path: Option<&Path>) -> Result<String> {
        Ok(match path {
            Some(p) => read_to_string(p)?,
            None => read_to_string("./input/day_4.txt")?,
        })
    }

    fn part_one(&self, input: &str) -> Result<Self::Output> {
        let xmas = XmasGrid::new(input);
        let mut tmp = 0;

        xmas.grid
            .iter()
            .filter(|(_, &ch)| ch == 'X')
            .for_each(|(&pt, _)| {
                tmp += xmas.find_xmas(pt);
            });

        Ok(tmp as Self::Output)
    }

    fn part_two(&self, input: &str) -> Result<Self::Output> {
        let xmas = XmasGrid::new(input);
        let mut tmp = 0;

        xmas.grid
            .iter()
            .filter(|(_, &ch)| ch == 'A')
            .for_each(|(&pt, _)| {
                tmp += xmas.find_x_mas(pt);
            });

        Ok(tmp as Self::Output)
    }
}
