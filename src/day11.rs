use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

pub type Data = i16;
pub type Value = usize;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c as i16 - 0x30).collect::<Vec<_>>())
        .flatten()
        .collect()
}

fn step(from: &[Data], to: &mut [Data]) -> usize {
    let mut result = 0;
    for i in 0..(WIDTH * HEIGHT) {
        to[i] = from[i] + 1;
    }
    'start: loop {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if to[x + y * WIDTH] > 9 {
                    result += 1;
                    to[x + y * WIDTH] = Data::MIN;
                    if x > 0 {
                        to[(x - 1) + y * WIDTH] += 1;
                        if y > 0 {
                            to[(x - 1) + (y - 1) * WIDTH] += 1;
                        }
                        if y < HEIGHT - 1 {
                            to[(x - 1) + (y + 1) * WIDTH] += 1;
                        }
                    }
                    if x < WIDTH - 1 {
                        to[(x + 1) + y * WIDTH] += 1;
                        if y > 0 {
                            to[(x + 1) + (y - 1) * WIDTH] += 1;
                        }
                        if y < HEIGHT - 1 {
                            to[(x + 1) + (y + 1) * WIDTH] += 1;
                        }
                    }
                    if y > 0 {
                        to[(x) + (y - 1) * WIDTH] += 1;
                    }
                    if y < HEIGHT - 1 {
                        to[(x) + (y + 1) * WIDTH] += 1;
                    }
                    continue 'start;
                }
            }
        }
        break;
    }
    for octopus in to.iter_mut() {
        if *octopus < 0 {
            *octopus = 0;
        }
    }
    result
}

/*
fn print_map(step: u32, map: &[Data]) {
    println!("After step {}", step);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", map[x + y * WIDTH]);
        }
        println!();
    }
    println!();
}
*/

#[aoc(day11, part1)]
pub fn solve_part1(data: &[Data]) -> Value {
    let mut map1 = Vec::from(data);
    let mut map2 = vec![0; 100];
    let mut sum = 0;
    for _step in 0..100 / 2 {
        sum += step(&map1, &mut map2);
        // print_map(1 + _step * 2, &map2);
        sum += step(&map2, &mut map1);
        // print_map(2 + _step * 2, &map1);
    }
    // print_map(100, &map1);
    sum
}

#[aoc(day11, part2)]
pub fn solve_part2(data: &[Data]) -> Value {
    let mut map1 = Vec::from(data);
    let mut map2 = vec![0; 100];
    for current in 0.. {
        step(&map1, &mut map2);
        if map2.iter().all(|octo| *octo == 0) {
            // print_map(1 + current * 2, &map2);
            return current as usize * 2 + 1;
        }
        step(&map2, &mut map1);
        if map1.iter().all(|octo| *octo == 0) {
            // print_map(2 + current * 2, &map1);
            return current as usize * 2 + 2;
        }
    }
    0
}
