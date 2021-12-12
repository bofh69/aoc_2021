use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

pub type Data = Vec<char>;
pub type Value = isize;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input.lines().map(|s| s.chars().collect()).collect()
}

fn is_start(c: char) -> bool {
    matches!(c, '<' | '[' | '(' | '{')
}

fn get_end(c: char) -> char {
    match c {
        '<' => '>',
        '[' => ']',
        '(' => ')',
        '{' => '}',
        _ => panic!("Unknown char {}", c),
    }
}

fn find_missing(data: &[char]) -> Option<char> {
    let mut stack = vec![];
    for &c in data {
        if is_start(c) {
            stack.push(get_end(c));
        } else {
            match stack.pop() {
                Some(expected) if c == expected => (),
                Some(_) => return Some(c),
                None => return None, // Incomplete
            }
        }
    }
    // Incomplete or done
    None
}

#[aoc(day10, part1)]
pub fn solve_part1(data: &[Data]) -> Value {
    let mut sum = 0;
    for line in data {
        // println!("Line: {:?}", line);
        if let Some(c) = find_missing(line) {
            // dbg!(&c);
            sum += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Unknown char {}", c),
            }
        }
    }
    sum
}

fn auto_complete(data: &[char]) -> Option<isize> {
    let mut stack = vec![];
    for &c in data {
        if is_start(c) {
            stack.push(get_end(c));
        } else {
            match stack.pop() {
                Some(expected) if c == expected => (),
                Some(_) => return None,
                None => break,
            }
        }
    }
    Some(
        stack
            .iter()
            .rev()
            .map(|c| match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Illegal char on stack: {}", c),
            })
            .fold(0, |acc, v| acc * 5 + v),
    )
}

#[aoc(day10, part2)]
pub fn solve_part2(data: &[Data]) -> Value {
    let mut sums = vec![];
    for line in data {
        // println!("Line: {:?}", line);
        if let Some(val) = auto_complete(line) {
            sums.push(val)
        }
    }
    sums.sort_unstable();
    sums[sums.len() / 2]
}
