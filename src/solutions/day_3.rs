use std::sync::LazyLock;

use regex::Regex;

use super::AocSolution;

static MUL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap());

static COND_MUL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap());

fn as_mul(s: &str) -> u64 {
    let (a, b) = s[4..(s.len() - 1)]
        .split_once(',')
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .unwrap();

    a * b
}

#[derive(Clone, Copy)]
pub struct AocDayThreeSolution;

impl AocSolution for AocDayThreeSolution {
    type Output = u64;
    const INPUT: &str = include_str!("../../input/day3.txt");

    fn part_one(&self) -> Self::Output {
        MUL.find_iter(Self::INPUT)
            .map(|x| x.as_str())
            .map(as_mul)
            .sum()
    }

    fn part_two(&self) -> Self::Output {
        COND_MUL
            .find_iter(Self::INPUT)
            .map(|x| x.as_str())
            .fold((0, true), |(sum, flag), cut| {
                if cut.starts_with("mul") && flag {
                    (sum + as_mul(cut), flag)
                } else if cut == "do()" {
                    (sum, true)
                } else {
                    (sum, false)
                }
            })
            .0
    }
}
