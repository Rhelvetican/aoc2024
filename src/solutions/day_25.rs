use std::{fs::read_to_string, path::Path};

use super::AocSolution;
use crate::utils::Result;

pub struct AocDayTwentyFiveSolution;

impl AocSolution for AocDayTwentyFiveSolution {
    type Output = u64;

    fn get_input(&self, path: Option<&Path>) -> Result<String> {
        Ok(match path {
            Some(p) => read_to_string(p)?,
            None => read_to_string("./input/day_day_25.txt")?,
        })
    }

    fn part_one(&self, input: &str) -> Result<Self::Output> {
        todo!()
    }

    fn part_two(&self, input: &str) -> Result<Self::Output> {
        todo!()
    }
}
