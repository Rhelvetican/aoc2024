use std::{fs::read_to_string, path::Path};

use super::AocSolution;
use crate::utils::Result;

fn compute(nums: &[u64], total: u64) -> bool {
    let mut equations = vec![false; nums.len() - 1];

    fn _inc_xf(flags: &mut [bool]) {
        let mut carry = true;
        for flag in flags.iter_mut().rev() {
            if !carry {
                break;
            }

            let tmp = *flag;
            *flag = !tmp;
            carry = tmp;
        }
    }

    loop {
        if equations.iter().zip(1..).fold(
            nums[0],
            |n, (&x, idx)| {
                if x {
                    n * nums[idx]
                } else {
                    n + nums[idx]
                }
            },
        ) == total
        {
            return true;
        } else {
            if equations.iter().all(|b| *b) {
                break;
            }
            _inc_xf(&mut equations);
        }
    }

    false
}

fn concat(n1: u64, n2: u64) -> u64 {
    format!("{n1}{n2}").parse().unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn inc(self) -> Self {
        match self {
            Self::Add => Self::Mul,
            Self::Mul => Self::Concat,
            Self::Concat => Self::Add,
        }
    }
}

fn compute_ver_two(nums: &[u64], total: u64) -> bool {
    let mut equations = vec![Operation::Add; nums.len() - 1];

    fn _inc_xf(flags: &mut [Operation]) {
        let mut carry = true;
        for flag in flags.iter_mut().rev() {
            if !carry {
                break;
            }

            let tmp = *flag;
            *flag = tmp.inc();
            carry = tmp == Operation::Concat;
        }
    }

    loop {
        if equations
            .iter()
            .zip(1..)
            .fold(nums[0], |n, (&x, idx)| match x {
                Operation::Add => n + nums[idx],
                Operation::Mul => n * nums[idx],
                Operation::Concat => concat(n, nums[idx]),
            })
            == total
        {
            return true;
        } else {
            if equations.iter().all(|b| *b == Operation::Concat) {
                break;
            }
            _inc_xf(&mut equations);
        }
    }

    false
}

#[derive(Clone, Copy)]
pub struct AocDaySevenSolution;

impl AocSolution for AocDaySevenSolution {
    type Output = u64;

    fn get_input(&self, path: Option<&Path>) -> Result<String> {
        Ok(match path {
            Some(p) => read_to_string(p)?,
            None => read_to_string("./input/day_7.txt")?,
        })
    }

    fn part_one(&self, input: &str) -> Result<Self::Output> {
        let mut sum = 0;
        for line in input.lines() {
            if let Some((total, nums)) = line.split_once(':') {
                let total = total.parse::<u64>()?;
                let nums = nums
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().map_err(|e| e.into()))
                    .collect::<Result<Vec<u64>>>()?;

                if compute(&nums, total) {
                    sum += total;
                }
            }
        }

        Ok(sum)
    }

    fn part_two(&self, input: &str) -> Result<Self::Output> {
        let mut sum = 0;
        for line in input.lines() {
            if let Some((total, nums)) = line.split_once(':') {
                let total = total.parse::<u64>()?;
                let nums = nums
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().map_err(|e| e.into()))
                    .collect::<Result<Vec<u64>>>()?;

                if compute_ver_two(&nums, total) {
                    sum += total;
                }
            }
        }

        Ok(sum)
    }
}
