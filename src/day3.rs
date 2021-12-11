use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

pub type Data = Vec<bool>;
pub type Value = isize;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c == '1').collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(data: &[Data]) -> Value {
    let total = data.len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..data[0].len() {
        let c = data.iter().map(|d| d[i]).filter(|v| *v).count();
        gamma *= 2;
        epsilon *= 2;
        if c > total / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    gamma * epsilon
}

fn bit_criteria(data: &[Data], crit: bool) -> Value {
    let mut left: Vec<_> = data.iter().collect();
    let digits = data[0].len();
    for i in 0..digits {
        let total = left.len();
        let count_true = left.iter().filter(|d| d[i]).count();
        if count_true * 2 >= total {
            left = left.iter().filter(|d| d[i] == crit).copied().collect();
        } else {
            left = left.iter().filter(|d| d[i] != crit).copied().collect();
        }
        if left.len() == 1 {
            break;
        }
    }
    left[0]
        .iter()
        .fold(0, |a, v| if *v { 1 + 2 * a } else { 2 * a })
}

#[aoc(day3, part2)]
pub fn solve_part2(data: &[Data]) -> Value {
    let oxygen = bit_criteria(data, true);
    let co2 = bit_criteria(data, false);
    oxygen * co2
}
