use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
// use rayon::prelude::*;

pub type Data = Vec<HashSet<char>>;
pub type Value = usize;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|s| {
            s.replace(" | ", " ")
                .split(' ')
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(data: &[Data]) -> Value {
    data.iter()
        .map(|v| vec![&v[10], &v[11], &v[12], &v[13]])
        .flatten()
        .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
        .count()
}

#[aoc(day8, part2)]
pub fn solve_part2(data: &[Data]) -> Value {
    let mut sum = 0;
    for line in data {
        let one: HashSet<char> = line.iter().take(10).find(|s| s.len() == 2).unwrap().clone();
        let four: HashSet<char> = line.iter().take(10).find(|s| s.len() == 4).unwrap().clone();
        let seven: HashSet<char> = line.iter().take(10).find(|s| s.len() == 3).unwrap().clone();
        let eight: HashSet<char> = line.iter().take(10).find(|s| s.len() == 7).unwrap().clone();
        let nine: HashSet<char> = line
            .iter()
            .take(10)
            .find(|s| s.len() == 6 && s.is_superset(&four))
            .unwrap()
            .clone();

        let four_minus_one = four.difference(&one).copied().collect::<HashSet<char>>();

        let five: HashSet<char> = line
            .iter()
            .take(10)
            .find(|s| s.len() == 5 && s.is_superset(&four_minus_one))
            .unwrap()
            .clone();
        let six: HashSet<char> = line
            .iter()
            .take(10)
            .find(|s| s.len() == 6 && s.is_superset(&four_minus_one) && **s != nine)
            .unwrap()
            .clone();

        let six_minus_four = six.difference(&four).copied().collect::<HashSet<char>>();

        let two: HashSet<char> = line
            .iter()
            .take(10)
            .find(|s| s.len() == 5 && s.is_superset(&six_minus_four))
            .unwrap()
            .clone();
        let three: HashSet<char> = line
            .iter()
            .take(10)
            .find(|s| s.len() == 5 && **s != two && **s != five)
            .unwrap()
            .clone();
        let zero: HashSet<char> = line
            .iter()
            .take(10)
            .find(|s| s.len() == 6 && **s != six && **s != nine)
            .unwrap()
            .clone();

        let trans = vec![zero, one, two, three, four, five, six, seven, eight, nine];

        let mut display = 0;
        for digit in line.iter().skip(10) {
            display *= 10;
            display += trans
                .iter()
                .enumerate()
                .find(|(_n, v)| *v == digit)
                .unwrap()
                .0;
        }

        sum += display;
    }
    sum
}
