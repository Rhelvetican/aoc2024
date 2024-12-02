pub mod day_1;
pub mod day_2;

pub trait AocSolution {
    type Output;
    const INPUT: &str;

    fn part_one(&self) -> Self::Output;
    fn part_two(&self) -> Self::Output;
}
