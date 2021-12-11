use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &[i32]) -> i32 {
    /*
    let mut higher = 0;
    let mut last_val = data[0];
    for val in data {
        if *val > last_val {
            higher += 1;
        }
        last_val = *val;
    }
    higher
    */
    data.iter()
        .fold((0i32, i32::MAX), |(higher, last), val| {
            if *val > last {
                (higher + 1, *val)
            } else {
                (higher, *val)
            }
        })
        .0
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[i32]) -> i32 {
    let mut higher = 0;
    let mut sum = data[0] + data[1] + data[2];
    for i in 3..data.len() {
        let sum2 = sum - data[i - 3] + data[i];
        if sum2 > sum {
            higher += 1;
        }
        sum = sum2;
    }
    higher

    /*
         * incorrect where it depends on the previous value,
         * i32::MAX or 0 are equally wrong there.

    data.par_windows(3).map(|d| d[0] + d[1] + d[2])
        .fold(|| (0i32, i32::MAX), |(higher, last), val| {
            if val > last {
                (higher + 1, val)
            } else {
                (higher, val)
            }
        })
        .reduce(|| (0, 0), |(h1, _), (h2, _)| (h1 + h2, 0))
            .0 - higher
            */
}
