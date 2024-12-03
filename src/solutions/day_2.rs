use super::AocSolution;

fn is_safe(levels: &[i32]) -> bool {
    let dec = levels[0] > levels[1];

    for (cur, nxt) in levels.windows(2).map(|w| (w[0], w[1])) {
        let diff = cur - nxt;

        if diff == 0 || diff.abs() > 3 || (diff > 0) != dec {
            return false;
        }
    }

    true
}

fn is_safe_with_rem(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        let mut lvl = levels[0..i].to_vec();
        lvl.extend_from_slice(&levels[(i + 1)..]);

        if is_safe(&lvl) {
            return true;
        }
    }

    false
}

#[derive(Clone, Copy)]
pub struct AocDayTwoSolution;

impl AocSolution for AocDayTwoSolution {
    type Output = u64;
    const INPUT: &str = include_str!("../../input/day2.txt");

    fn part_one(&self) -> Self::Output {
        Self::INPUT
            .lines()
            .map(|n| {
                n.split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            })
            .filter(|s| is_safe(s))
            .count() as u64
    }

    fn part_two(&self) -> Self::Output {
        Self::INPUT
            .lines()
            .map(|n| {
                n.split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            })
            .filter(|s| is_safe(s) || is_safe_with_rem(s))
            .count() as u64
    }
}
