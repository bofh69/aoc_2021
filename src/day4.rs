use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

pub struct Data {
    numbers: Vec<u8>,
    boards: Vec<[u8; 25]>,
}
pub type Value = isize;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Data {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut boards = vec![];
    while lines.next().is_some() {
        let mut v = [0u8; 25];
        for y in 0..5 {
            let s = lines.next().unwrap();
            let mut s = s.split_ascii_whitespace();
            for x in 0..5 {
                v[x + y * 5] = s.next().unwrap().parse().unwrap();
            }
        }
        boards.push(v);
    }

    Data { numbers, boards }
}

#[aoc(day4, part1)]
pub fn solve_part1(data: &Data) -> Value {
    let mut boards = data.boards.clone();

    for n in &data.numbers {
        for b in &mut boards {
            for y in 0..5 {
                for x in 0..5 {
                    if b[x + y * 5] == *n {
                        b[x + y * 5] = 255;

                        let mut found = true;
                        for x2 in 0..5 {
                            if b[x2 + y * 5] != 255 {
                                found = false;
                                break;
                            }
                        }
                        if !found {
                            found = true;
                            for y2 in 0..5 {
                                if b[x + y2 * 5] != 255 {
                                    found = false;
                                }
                            }
                        }
                        if found {
                            return (*n as isize)
                                * (b.iter()
                                    .filter(|v| **v != 255)
                                    .map(|v| *v as isize)
                                    .sum::<isize>());
                        }
                    }
                }
            }
        }
    }
    -1
}

#[aoc(day4, part2)]
pub fn solve_part2(data: &Data) -> Value {
    let mut boards = data.boards.clone();

    let mut winning_boards = vec![];

    let mut won_boards = std::collections::HashMap::new();

    for n in &data.numbers {
        for (b_n, b) in boards.iter_mut().enumerate() {
            if !won_boards.contains_key(&b_n) {
                for y in 0..5 {
                    for x in 0..5 {
                        if b[x + y * 5] == *n {
                            b[x + y * 5] = 255;

                            let mut found = true;
                            for x2 in 0..5 {
                                if b[x2 + y * 5] != 255 {
                                    found = false;
                                    break;
                                }
                            }
                            if !found {
                                found = true;
                                for y2 in 0..5 {
                                    if b[x + y2 * 5] != 255 {
                                        found = false;
                                    }
                                }
                            }
                            if found {
                                winning_boards.push(b_n);
                                won_boards.insert(b_n, n);
                            }
                        }
                    }
                }
            }
        }
    }
    let b_n = *winning_boards.last().unwrap();
    let n = *won_boards.get(&b_n).unwrap();
    let b = boards[b_n];
    return (*n as isize)
        * (b.iter()
            .filter(|v| **v != 255)
            .map(|v| *v as isize)
            .sum::<isize>());
}
