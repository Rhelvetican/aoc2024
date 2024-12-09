use std::{collections::HashMap, fs::read_to_string, iter::from_fn, path::Path};

use super::AocSolution;
use crate::utils::Result;

fn checksum(fs: &[Option<u64>]) -> u64 {
    fs.iter()
        .copied()
        .zip(0..)
        .filter_map(|(n, m)| n.map(|n| n * m))
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Filedata {
    size: usize,
    ptr: usize,
}

impl Filedata {
    const fn new(size: usize, ptr: usize) -> Self {
        Self { size, ptr }
    }
}

#[derive(Debug)]
struct Disk {
    pub id: u64,
    pub metadata: HashMap<u64, Filedata>,
    pub fs: Vec<Option<u64>>,
}

impl Disk {
    fn new(src: &str) -> Self {
        let total = src.len();
        let mut flag = false;

        let (fs, metadata, id) = src
            .chars()
            .zip(from_fn(move || {
                flag = !flag;
                Some(flag)
            }))
            .filter_map(|(ch, flag)| match ch {
                '0' => Some((0, flag)),
                '1' => Some((1, flag)),
                '2' => Some((2, flag)),
                '3' => Some((3, flag)),
                '4' => Some((4, flag)),
                '5' => Some((5, flag)),
                '6' => Some((6, flag)),
                '7' => Some((7, flag)),
                '8' => Some((8, flag)),
                '9' => Some((9, flag)),
                _ => None,
            })
            .fold(
                (Vec::with_capacity(total), HashMap::new(), 0),
                |(mut v, mut meta, mut id), (len, flag)| {
                    let idx = v.len();
                    for _ in 0..len {
                        if flag {
                            meta.insert(id, Filedata::new(len, idx));
                        }

                        v.push(if flag { Some(id) } else { None });
                    }

                    if flag {
                        id += 1
                    }

                    (v, meta, id)
                },
            );

        Self { id, fs, metadata }
    }

    fn sort(&mut self) -> u64 {
        let (mut first, mut last) = (0, self.fs.len() - 1);

        fn _is_continous(fs: &[Option<u64>]) -> bool {
            if let Some(first_none) = fs.iter().position(|o| o.is_none()) {
                fs[first_none..].iter().all(|o| o.is_none())
            } else {
                true
            }
        }

        while !_is_continous(&self.fs) {
            while self.fs[first].is_some() {
                first += 1;
            }

            while self.fs[last].is_none() {
                last -= 1;
            }

            self.fs.swap(first, last);
        }

        checksum(&self.fs)
    }

    fn sort_block(&mut self) -> u64 {
        fn swap_block(fs: &mut [Option<u64>], a: usize, b: usize, size: usize) {
            for offset in 0..size {
                if fs[a + offset].is_none() || fs[b + offset].is_some() {
                    fs.swap(a + offset, b + offset);
                }
            }
        }

        fn allocate(fs: &[Option<u64>], size: usize, limit: usize) -> Option<usize> {
            let mut offset = 0;

            loop {
                let slice = match fs.get(offset..(offset + size)) {
                    Some(sl) => sl,
                    None => return None,
                };

                if slice.iter().all(|o| o.is_none()) {
                    break;
                }

                offset += 1;
                if offset >= limit {
                    return None;
                }
            }

            Some(offset)
        }

        for id in (0..self.id).rev() {
            let metadata = &self.metadata[&id];
            if let Some(ptr) = allocate(&self.fs, metadata.size, metadata.ptr) {
                swap_block(&mut self.fs, ptr, metadata.ptr, metadata.size);
            }
        }

        checksum(&self.fs)
    }
}

pub struct AocDayNineSolution;

impl AocSolution for AocDayNineSolution {
    type Output = u64;

    fn get_input(&self, path: Option<&Path>) -> Result<String> {
        Ok(match path {
            Some(p) => read_to_string(p)?,
            None => read_to_string("./input/day_9.txt")?,
        })
    }

    fn part_one(&self, input: &str) -> Result<Self::Output> {
        let mut disk = Disk::new(input);
        Ok(disk.sort())
    }

    fn part_two(&self, input: &str) -> Result<Self::Output> {
        let mut disk = Disk::new(input);
        Ok(disk.sort_block())
    }
}
