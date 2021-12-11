use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

pub struct Data(String);
pub type Value = isize;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input.lines().map(|s| Data(s.to_owned())).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(_data: &[Data]) -> Value {
    0
}

#[aoc(day9, part2)]
pub fn solve_part2(_data: &[Data]) -> Value {
    0
}
