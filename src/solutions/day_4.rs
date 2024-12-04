use std::collections::HashMap;

use super::AocSolution;

const DIRS: [(i16, i16); 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

struct XmasGrid {
    pub grid: HashMap<(i16, i16), char>,
}

impl XmasGrid {
    fn new(src: &str) -> Self {
        Self {
            grid: src
                .lines()
                .zip(0..)
                .flat_map(|(l, y)| l.chars().zip(0..).map(move |(c, x)| ((x, y), c)))
                .collect(),
        }
    }

    fn find_xmas(&self, pos: (i16, i16)) -> usize {
        let mut tmp = 0;
        for (dx, dy) in DIRS {
            let mut found = true;
            let (mut x, mut y) = pos;

            for nxt in ['M', 'A', 'S'] {
                x += dx;
                y += dy;

                if self.grid.get(&(x, y)) != Some(&nxt) {
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

    fn find_x_mas(&self, pos: (i16, i16)) -> usize {
        let (x, y) = pos;

        let tr = self.grid.get(&(x + 1, y + 1));
        let br = self.grid.get(&(x + 1, y - 1));
        let bl = self.grid.get(&(x - 1, y - 1));
        let tl = self.grid.get(&(x - 1, y + 1));

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
    type Output = usize;
    const INPUT: &str = include_str!("../../input/day4.txt");

    fn part_one(&self) -> Self::Output {
        let xmas = XmasGrid::new(Self::INPUT);
        let mut tmp = 0;

        for (&pos, &ch) in &xmas.grid {
            if ch == 'X' {
                tmp += xmas.find_xmas(pos)
            }
        }

        tmp as Self::Output
    }

    fn part_two(&self) -> Self::Output {
        let xmas = XmasGrid::new(Self::INPUT);
        let mut tmp = 0;

        for (&pos, &ch) in &xmas.grid {
            if ch == 'A' {
                tmp += xmas.find_x_mas(pos)
            }
        }

        tmp as Self::Output
    }
}
