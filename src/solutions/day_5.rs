use std::{cmp::Ordering, collections::HashMap};

use super::AocSolution;

#[derive(Debug)]
struct RuleChecker {
    rules: HashMap<u8, Vec<u8>>,
    prints: Vec<Vec<u8>>,
}

impl RuleChecker {
    fn new(src: &str, prints_src: &str) -> Self {
        Self {
            rules: src
                .lines()
                .filter_map(|l| l.split_once('|'))
                .filter_map(
                    |(a, b)| match (a.parse::<u8>().ok(), b.parse::<u8>().ok()) {
                        (Some(a), Some(b)) => Some((a, b)),
                        _ => None,
                    },
                )
                .fold(HashMap::new(), |mut map, (v, k)| {
                    let entry = map.entry(k).or_default();
                    entry.push(v);
                    map
                }),
            prints: prints_src
                .lines()
                .map(|l| l.split(',').filter_map(|n| n.parse::<u8>().ok()).collect())
                .collect(),
        }
    }

    fn validate_print(&self, print: &[u8]) -> u8 {
        if print.is_sorted_by(|a, b| self.rules.get(b).is_none_or(|v| v.contains(a))) {
            print[print.len() / 2]
        } else {
            0
        }
    }

    fn compute_validated_print(&self) -> u64 {
        let mut tmp = 0;
        for print in &self.prints {
            tmp += self.validate_print(print) as u64;
        }

        tmp
    }

    fn compute_invalid_print(&mut self) -> u64 {
        let mut tmp = 0;
        for print in self.prints.iter_mut() {
            if !print.is_sorted_by(|a, b| self.rules.get(b).is_none_or(|v| v.contains(a))) {
                print.sort_by(|a, b| {
                    if self.rules.get(a).is_none_or(|v| v.contains(b)) {
                        Ordering::Less
                    } else if self.rules.get(b).is_none_or(|v| v.contains(a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });

                tmp += print[print.len() / 2] as u64;
            }
        }

        tmp
    }
}

#[derive(Clone, Copy)]
pub struct AocDayFiveSolution;

impl AocSolution for AocDayFiveSolution {
    type Output = u64;
    const INPUT: &str = include_str!("../../input/day5.txt");

    fn part_one(&self) -> Self::Output {
        let (rules, prints) = Self::INPUT.split_once("\r\n\r\n").unwrap_or(("", ""));
        let checker = RuleChecker::new(rules, prints);
        checker.compute_validated_print()
    }

    fn part_two(&self) -> Self::Output {
        let (rules, prints) = Self::INPUT.split_once("\r\n\r\n").unwrap_or(("", ""));
        let mut checker = RuleChecker::new(rules, prints);
        checker.compute_invalid_print()
    }
}
