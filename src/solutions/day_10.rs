use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::Path,
};

use super::AocSolution;
use crate::utils::{coord::Coord as Crd, Result};

type Coord = Crd<i8>;

struct HikingMap {
    pub starting_points: Vec<Coord>,
    pub mountain: HashMap<Coord, u8>,
}

impl HikingMap {
    fn new(src: &str) -> Self {
        let (starting_points, mountain) = src
            .lines()
            .zip(0..)
            .flat_map(|(line, y)| {
                line.chars().zip(0..).filter_map(move |(ch, x)| match ch {
                    '0' => Some((Coord::new(x, y), 0)),
                    '1' => Some((Coord::new(x, y), 1)),
                    '2' => Some((Coord::new(x, y), 2)),
                    '3' => Some((Coord::new(x, y), 3)),
                    '4' => Some((Coord::new(x, y), 4)),
                    '5' => Some((Coord::new(x, y), 5)),
                    '6' => Some((Coord::new(x, y), 6)),
                    '7' => Some((Coord::new(x, y), 7)),
                    '8' => Some((Coord::new(x, y), 8)),
                    '9' => Some((Coord::new(x, y), 9)),
                    _ => None,
                })
            })
            .fold(
                (Vec::new(), HashMap::new()),
                |(mut spos, mut map), (coord, heigh)| {
                    if heigh == 0 {
                        spos.push(coord);
                    }

                    map.insert(coord, heigh);

                    (spos, map)
                },
            );

        Self {
            starting_points,
            mountain,
        }
    }

    fn hike(&self) -> u64 {
        fn _inner(map: &HashMap<Coord, u8>, start_pos: Coord) -> u64 {
            fn _traverse(map: &HashMap<Coord, u8>, summits: &mut HashSet<Coord>, current: Coord) {
                if map.get(&current).is_some_and(|h| h == &9) {
                    summits.insert(current);
                    return;
                }

                for dir in current.surround() {
                    if map.get(&dir).is_some_and(|&h| map[&current] + 1 == h) {
                        _traverse(map, summits, dir);
                    }
                }
            }

            let mut visited = HashSet::new();
            if map.contains_key(&start_pos) {
                _traverse(map, &mut visited, start_pos);
                visited.len() as u64
            } else {
                0
            }
        }

        self.starting_points
            .iter()
            .map(|pos| _inner(&self.mountain, *pos))
            .sum()
    }

    fn hike_ver_two(&self) -> u64 {
        fn _inner(map: &HashMap<Coord, u8>, start_pos: Coord) -> u64 {
            fn _traverse(map: &HashMap<Crd<i8>, u8>, current: Coord, rate: &mut u64) {
                if map.get(&current).is_some_and(|h| h == &9) {
                    *rate += 1;
                    return;
                }

                for dir in current.surround() {
                    if map.get(&dir).is_some_and(|&h| map[&current] + 1 == h) {
                        _traverse(map, dir, rate);
                    }
                }
            }

            let mut rating = 0;
            if map.contains_key(&start_pos) {
                _traverse(map, start_pos, &mut rating);
                rating
            } else {
                0
            }
        }

        self.starting_points
            .iter()
            .map(|pos| _inner(&self.mountain, *pos))
            .sum()
    }
}

#[derive(Clone, Copy)]
pub struct AocDayTenSolution;

impl AocSolution for AocDayTenSolution {
    type Output = u64;

    fn get_input(&self, path: Option<&Path>) -> Result<String> {
        Ok(match path {
            Some(p) => read_to_string(p)?,
            None => read_to_string("./input/day_10.txt")?,
        })
    }

    fn part_one(&self, input: &str) -> Result<Self::Output> {
        let map = HikingMap::new(input);
        Ok(map.hike())
    }

    fn part_two(&self, input: &str) -> Result<Self::Output> {
        let map = HikingMap::new(input);
        Ok(map.hike_ver_two())
    }
}
