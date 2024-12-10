use std::path::Path;

use crate::utils::Result;

macro_rules! define_solution {
    ($($id:ident),+ $(,)?) => {
        $(mod $id; pub use $id::*;)+
    };
}

define_solution!(
    day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9,
    day_10,
    // day_11,
    // day_12,
    // day_13,
    // day_14,
    // day_15,
    // day_16,
    // day_17,
    // day_18,
    // day_19,
    // day_20,
    // day_21,
    // day_22,
    // day_23,
    // day_24,
    // day_25,
);

pub trait AocSolution {
    type Output;

    fn get_input(&self, path: Option<&Path>) -> Result<String>;
    fn part_one(&self, input: &str) -> Result<Self::Output>;
    fn part_two(&self, input: &str) -> Result<Self::Output>;
}
