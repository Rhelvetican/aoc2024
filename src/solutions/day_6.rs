use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::Path,
};

use super::AocSolution;
use crate::utils::{coord::Coord, direction::Direction, Result};

#[derive(Debug, Clone)]
struct Guard {
    dir: Direction,
    cur: Coord,
    grid: HashMap<Coord, bool>,
}

impl Guard {
    fn new(src: &str) -> Self {
        let (grid, cur) = src
            .lines()
            .zip(0..)
            .flat_map(|(l, y)| {
                l.chars()
                    .zip(0..)
                    .map(move |(ch, x)| (Coord::new(x, y), ch))
            })
            .fold(
                (HashMap::new(), Coord::new(0, 0)),
                |(mut map, coord), (crd, obs)| {
                    map.insert(crd, obs == '#');
                    let coord = if obs == '^' { crd } else { coord };
                    (map, coord)
                },
            );

        Self {
            dir: Direction::default(),
            cur,
            grid,
        }
    }

    fn patrol(&mut self) -> (usize, bool) {
        let mut repetition = 0u16;
        let mut flag = false;
        let mut patrolled = HashSet::new();

        loop {
            if repetition == 65535 {
                flag = true;
                break;
            }

            let lookahead = self.cur + self.dir;
            if let Some(&obs) = self.grid.get(&lookahead) {
                if obs {
                    self.dir.turn_right();
                } else {
                    if !patrolled.insert(self.cur) {
                        repetition += 1;
                    };
                    self.cur = lookahead;
                }
            } else {
                break;
            }
        }

        (patrolled.len() + 1, flag)
    }

    fn loop_prank(&mut self) -> u64 {
        let mut total = 0;
        for &key in self.grid.keys() {
            let mut tmp = self.clone();
            if tmp.grid.get(&key).is_some_and(|cond| !*cond) {
                tmp.grid.insert(key, true);
                if tmp.patrol().1 {
                    total += 1;
                }
            };
        }

        total
    }
}

#[derive(Clone, Copy)]
pub struct AocDaySixSolution;

impl AocSolution for AocDaySixSolution {
    type Output = u64;
    fn get_input(&self, path: Option<&Path>) -> Result<String> {
        match path {
            Some(p) => Ok(read_to_string(p)?),
            None => Ok(read_to_string("./input/day_6.txt")?),
        }
    }

    fn part_one(&self, input: &str) -> Result<Self::Output> {
        let mut g = Guard::new(input);
        Ok(g.patrol().0 as Self::Output)
    }

    fn part_two(&self, input: &str) -> Result<Self::Output> {
        let mut g = Guard::new(input);
        Ok(g.loop_prank())
    }
}
