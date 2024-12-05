use std::{collections::HashMap, fs::read_to_string, path::Path};

use super::AocSolution;
use crate::utils::Result;

#[derive(Clone, Copy)]
pub struct AocDayOneSolution;

impl AocSolution for AocDayOneSolution {
    type Output = u64;

    fn get_input(&self, path: Option<&Path>) -> Result<String> {
        Ok(match path {
            Some(p) => read_to_string(p)?,
            None => read_to_string("./input/day_1.txt")?,
        })
    }

    fn part_one(&self, input: &str) -> Result<Self::Output> {
        let (mut left, mut right) = input
            .lines()
            .filter_map(|s| s.split_once(' ').map(|(s, x)| (s.trim(), x.trim())))
            .filter_map(
                |(s, x)| match (s.parse::<u64>().ok(), x.parse::<u64>().ok()) {
                    (Some(x), Some(y)) => Some((x, y)),
                    _ => None,
                },
            )
            .fold((Vec::new(), Vec::new()), |(mut v1, mut v2), (x, y)| {
                v1.push(x);
                v2.push(y);
                (v1, v2)
            });

        left.sort();
        right.sort();

        Ok(left
            .into_iter()
            .zip(right)
            .map(|(l, r)| if l > r { l - r } else { r - l })
            .sum())
    }

    fn part_two(&self, input: &str) -> Result<Self::Output> {
        let mut cache = HashMap::<u64, u64>::new();
        let mut similarity_score = 0;

        let (left, right) = input
            .lines()
            .filter_map(|s| s.split_once(' ').map(|(s, x)| (s.trim(), x.trim())))
            .filter_map(
                |(s, x)| match (s.parse::<u64>().ok(), x.parse::<u64>().ok()) {
                    (Some(x), Some(y)) => Some((x, y)),
                    _ => None,
                },
            )
            .fold((Vec::new(), Vec::new()), |(mut v1, mut v2), (x, y)| {
                v1.push(x);
                v2.push(y);
                (v1, v2)
            });

        for key in left {
            if let Some(&val) = cache.get(&key) {
                similarity_score += key * val;
            } else {
                let val = right.iter().filter(|e| e.eq(&&key)).count() as u64;
                cache.insert(key, val);
                similarity_score += key * val;
            }
        }

        Ok(similarity_score)
    }
}
