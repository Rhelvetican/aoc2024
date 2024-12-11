use std::{collections::HashMap, fs::read_to_string, path::Path};

use super::AocSolution;
use crate::utils::Result;

struct Stones {
    stones: Vec<u64>,
}

impl Stones {
    fn new(src: &str) -> Self {
        Self {
            stones: src
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect(),
        }
    }

    fn blinks(&self, cnt: u8) -> u64 {
        fn _inner(cache: &mut HashMap<(u64, u8), u64>, stone: u64, cnt: u8) -> u64 {
            if cnt == 0 {
                return 1;
            }

            if let Some(&val) = cache.get(&(stone, cnt)) {
                return val;
            }

            let next = match stone {
                0 => _inner(cache, 1, cnt - 1),
                n => {
                    let digits = n.ilog10() + 1;
                    if digits % 2 == 0 {
                        let pow = 10u64.pow(digits / 2);
                        _inner(cache, stone / pow, cnt - 1) + _inner(cache, stone % pow, cnt - 1)
                    } else {
                        _inner(cache, stone * 2024, cnt - 1)
                    }
                }
            };

            cache.insert((stone, cnt), next);
            next
        }

        let mut cache = HashMap::new();
        self.stones
            .iter()
            .map(|&x| _inner(&mut cache, x, cnt))
            .sum()
    }
}

#[derive(Clone, Copy)]
pub struct AocDayElevenSolution;

impl AocSolution for AocDayElevenSolution {
    type Output = u64;

    fn get_input(&self, path: Option<&Path>) -> Result<String> {
        Ok(match path {
            Some(p) => read_to_string(p)?,
            None => read_to_string("./input/day_11.txt")?,
        })
    }

    fn part_one(&self, input: &str) -> Result<Self::Output> {
        let stones = Stones::new(input);
        Ok(stones.blinks(25))
    }

    fn part_two(&self, input: &str) -> Result<Self::Output> {
        let stones = Stones::new(input);
        Ok(stones.blinks(75))
    }
}
