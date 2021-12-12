use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

pub type Data = (String, String);
pub type Value = isize;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|s| {
            let mut iter = s.split('-');
            (
                iter.next().unwrap().to_owned(),
                iter.next().unwrap().to_owned(),
            )
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(data: &[Data]) -> Value {
    dbg!(data);
    0
}

#[aoc(day12, part2)]
pub fn solve_part2(_data: &[Data]) -> Value {
    0
}
