use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/day1.txt");

pub fn part_one() -> u64 {
    let (mut left, mut right) = INPUT
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

    left.into_iter()
        .zip(right)
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum()
}

pub fn part_two() -> u64 {
    let mut cache = HashMap::<u64, u64>::new();
    let mut similarity_score = 0;

    let (left, right) = INPUT
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

    similarity_score
}
