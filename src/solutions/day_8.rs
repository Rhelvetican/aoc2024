use std::{collections::HashMap, fs::read_to_string, path::Path};

use itertools::Itertools;

use super::AocSolution;
use crate::utils::{coord::Coord, Result};

fn anticoord(a: Coord<i8>, b: Coord<i8>) -> (Coord<i8>, Coord<i8>) {
    let delta = a - b;
    (a + delta, b - delta)
}

struct Map {
    pub antennas: HashMap<char, Vec<Coord<i8>>>,
    pub grid: HashMap<Coord<i8>, bool>,
}

impl Map {
    fn new(src: &str) -> Self {
        let (antennas, grid) = src
            .lines()
            .zip(0..)
            .flat_map(|(line, y)| {
                line.chars()
                    .zip(0..)
                    .map(move |(ch, x)| (Coord::new(x, y), ch))
            })
            .fold(
                (HashMap::new(), HashMap::new()),
                |(mut antennas, mut map), (coord, ch)| {
                    if ch != '.' {
                        antennas.entry(ch).or_insert(Vec::new()).push(coord);
                    }

                    map.insert(coord, false);

                    (antennas, map)
                },
            );

        Self { antennas, grid }
    }

    fn locations(&mut self) -> u64 {
        for freq in self.antennas.values() {
            freq.iter()
                .tuple_combinations::<(_, _)>()
                .map(|(&a, &b)| anticoord(a, b))
                .for_each(|(a, b)| {
                    if let Some(flag) = self.grid.get_mut(&a) {
                        *flag = true;
                    }

                    if let Some(flag) = self.grid.get_mut(&b) {
                        *flag = true;
                    }
                });
        }

        self.grid.values().filter(|&&x| x).count() as u64
    }

    fn resonance(&mut self) -> u64 {
        for freq in self.antennas.values() {
            freq.iter()
                .permutations(2)
                .map(|v| (v[0], v[1]))
                .filter(|(a, b)| a.ne(b))
                .for_each(|(&a, &b)| {
                    let dist = b - a;
                    let mut anti = b;

                    while let Some(flag) = self.grid.get_mut(&anti) {
                        *flag = true;
                        anti = anti + dist;
                    }
                });
        }

        self.grid.values().filter(|&&x| x).count() as u64
    }
}

#[derive(Clone, Copy)]
pub struct AocDayEightSolution;

impl AocSolution for AocDayEightSolution {
    type Output = u64;

    fn get_input(&self, path: Option<&Path>) -> Result<String> {
        Ok(match path {
            Some(p) => read_to_string(p)?,
            None => read_to_string("./input/day_8.txt")?,
        })
    }

    fn part_one(&self, input: &str) -> Result<Self::Output> {
        let mut map = Map::new(input);
        Ok(map.locations())
    }

    fn part_two(&self, input: &str) -> Result<Self::Output> {
        let mut map = Map::new(input);
        Ok(map.resonance())
    }
}
