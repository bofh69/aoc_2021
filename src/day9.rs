use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

pub type Data = Vec<u8>;
pub type Value = usize;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c as u8 - 0x30).collect())
        .collect()
}

fn find_low(data: &[Data]) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let height = data.len();
    let width = data[0].len();
    for (y, row) in data.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if y > 0 && data[y - 1][x] <= value {
                continue;
            }
            if x > 0 && row[x - 1] <= value {
                continue;
            }
            if y < height - 1 && data[y + 1][x] <= value {
                continue;
            }
            if x < width - 1 && row[x + 1] <= value {
                continue;
            }
            result.push((x, y));
        }
    }
    result
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &[Data]) -> Value {
    find_low(data)
        .iter()
        .map(|(x, y)| data[*y][*x] as usize + 1)
        .sum()
}

fn fill_basin(pos: (usize, usize), data: &mut [Vec<u8>]) -> usize {
    let val = data[pos.1][pos.0];
    if val == 9 {
        return 0;
    }
    data[pos.1][pos.0] = 9;
    let mut sum = 1;
    if pos.0 > 0 && data[pos.1][pos.0 - 1] > val {
        sum += fill_basin((pos.0 - 1, pos.1), data);
    }
    let width = data[0].len();
    if pos.0 < width - 1 && data[pos.1][pos.0 + 1] > val {
        sum += fill_basin((pos.0 + 1, pos.1), data);
    }
    if pos.1 > 0 && data[pos.1 - 1][pos.0] > val {
        sum += fill_basin((pos.0, pos.1 - 1), data);
    }
    let height = data.len();
    if pos.1 < height - 1 && data[pos.1 + 1][pos.0] > val {
        sum += fill_basin((pos.0, pos.1 + 1), data);
    }
    sum
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &[Data]) -> Value {
    let lows = find_low(data);
    let mut result = vec![];
    for low in lows {
        let mut dat = Vec::new();
        dat.extend_from_slice(data);
        let size = fill_basin(low, &mut dat);
        result.push(size);
    }
    result.sort_unstable();
    result.reverse();
    result.iter().take(3).product()
}
