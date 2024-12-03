use super::AocSolution;

#[derive(Clone, Copy)]
pub struct AocDayTwoSolution;

struct LevelValidator;

impl LevelValidator {
    fn is_safe(&self, levels: &[i32]) -> bool {
        let dec = levels[0] > levels[1];

        for (cur, nxt) in levels.windows(2).map(|w| (w[0], w[1])) {
            let diff = cur - nxt;

            if diff == 0 || diff.abs() > 3 || (diff > 0) != dec {
                return false;
            }
        }

        true
    }

    fn is_safe_with_rem(&self, levels: &[i32]) -> bool {
        for i in 0..levels.len() {
            let mut lvl = levels[0..i].to_vec();
            lvl.extend_from_slice(&levels[(i + 1)..]);

            if self.is_safe(&lvl) {
                return true;
            }
        }

        false
    }
}

impl AocSolution for AocDayTwoSolution {
    type Output = u64;
    const INPUT: &str = include_str!("../../input/day2.txt");

    fn part_one(&self) -> Self::Output {
        let validator = LevelValidator;

        Self::INPUT
            .lines()
            .map(|n| {
                n.split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            })
            .filter(|s| validator.is_safe(s))
            .count() as u64
    }

    fn part_two(&self) -> Self::Output {
        let validator = LevelValidator;

        Self::INPUT
            .lines()
            .map(|n| {
                n.split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            })
            .filter(|s| validator.is_safe(s) || validator.is_safe_with_rem(s))
            .count() as u64
    }
}
