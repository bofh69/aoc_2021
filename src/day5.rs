use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

#[derive(Debug)]
pub struct Data((i16, i16), (i16, i16));
pub type Value = isize;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|s| {
            let mut vals = s.split(" -> ");
            let v1 = vals
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();
            let v2 = vals
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();
            Data((v1[0], v1[1]), (v2[0], v2[1]))
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &[Data]) -> Value {
    let mut map = std::collections::HashMap::new();

    for line in data {
        if line.0 .0 == line.1 .0 {
            let min_y = std::cmp::min(line.0 .1, line.1 .1);
            let max_y = std::cmp::max(line.0 .1, line.1 .1);
            for y in min_y..=max_y {
                if let Some(n) = map.get_mut(&(line.0 .0, y)) {
                    *n += 1;
                } else {
                    map.insert((line.0 .0, y), 1);
                }
            }
        } else if line.0 .1 == line.1 .1 {
            let min_x = std::cmp::min(line.0 .0, line.1 .0);
            let max_x = std::cmp::max(line.0 .0, line.1 .0);
            for x in min_x..=max_x {
                if let Some(n) = map.get_mut(&(x, line.0 .1)) {
                    *n += 1;
                } else {
                    map.insert((x, line.0 .1), 1);
                }
            }
        }
    }
    map.values().filter(|n| **n > 1).count() as isize
}

fn new_pos(a: i16, b: i16) -> i16 {
    if a < b {
        a + 1
    } else if a > b {
        a - 1
    } else {
        a
    }
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &[Data]) -> Value {
    let mut map = std::collections::HashMap::new();

    for line in data {
        let (mut x, mut y) = line.0;
        loop {
            if let Some(n) = map.get_mut(&(x, y)) {
                *n += 1;
            } else {
                map.insert((x, y), 1);
            }
            if (x, y) == line.1 {
                break;
            }
            x = new_pos(x, line.1 .0);
            y = new_pos(y, line.1 .1);
        }
    }
    map.values().filter(|n| **n > 1).count() as isize
}
