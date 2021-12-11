use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

pub type Value = isize;
pub type Data = u8;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

fn how_many_after(data: &[Data], days: isize) -> Value {
    let mut fish = vec![0; 9];

    for f in data {
        fish[*f as usize] += 1;
    }

    for _day in 0..days {
        let mut new_fish = vec![0; 9];
        new_fish[8] = fish[0];
        new_fish[..8].clone_from_slice(&fish[1..9]);
        new_fish[6] += fish[0];
        fish = new_fish;
    }
    fish.iter().sum()
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &[Data]) -> Value {
    how_many_after(data, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(data: &[Data]) -> Value {
    how_many_after(data, 256)
}
