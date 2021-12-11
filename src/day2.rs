use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

pub type Value = isize;
pub struct Data(String, Value);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|s| {
            let mut s = s.split(' ');
            Data(
                s.next().unwrap().to_owned(),
                s.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &[Data]) -> Value {
    let (x, y) = data
        .iter()
        .fold((0, 0), |(x, y), Data(s, v)| match s.as_str() {
            "forward" => (x + v, y),
            "down" => (x, y + v),
            "up" => (x, y - v),
            _ => panic!("Unknown command {}", s),
        });
    x * y
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &[Data]) -> Value {
    let (x, y, _aim) = data
        .iter()
        .fold((0, 0, 0), |(x, y, aim), Data(s, v)| match s.as_str() {
            "forward" => (x + v, y + aim * v, aim),
            "down" => (x, y, aim + v),
            "up" => (x, y, aim - v),
            _ => panic!("Unknown command {}", s),
        });
    x * y
}
