use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, Sub},
};

pub trait Unit:
    Debug + Clone + Copy + PartialEq + Eq + Hash + Add<Output = Self> + Sub<Output = Self> + From<i8>
{
}

impl_unit_for_type!(i8, i16, i32, i64, i128);
