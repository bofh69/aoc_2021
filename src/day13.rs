use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
// use rayon::prelude::*;

#[derive(Debug)]
pub struct Data {
    points: HashSet<(i32, i32)>,
    folds: Vec<i32>,
}
pub type Value = isize;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Data {
    let mut result = Data {
        points: HashSet::new(),
        folds: vec![],
    };
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut line = line.split(',');
        result.points.insert((
            line.next().unwrap().parse().unwrap(),
            line.next().unwrap().parse().unwrap(),
        ));
    }
    for line in lines.by_ref() {
        // fold along x=327
        if line.contains("fold along x") {
            result.folds.push(line[13..].parse().unwrap());
        } else {
            result.folds.push(-line[13..].parse::<i32>().unwrap());
        }
    }
    result
}

fn fold_x(points: &HashSet<(i32, i32)>, x_line: i32) -> HashSet<(i32, i32)> {
    points
        .iter()
        .map(|(x, y)| {
            if *x < x_line {
                (*x, *y)
            } else {
                (x_line - (*x - x_line), *y)
            }
        })
        .collect()
}

fn fold_y(points: &HashSet<(i32, i32)>, y_line: i32) -> HashSet<(i32, i32)> {
    points
        .iter()
        .map(|(x, y)| {
            if *y < y_line {
                (*x, *y)
            } else {
                (*x, y_line - (*y - y_line))
            }
        })
        .collect()
}

fn fold(points: &HashSet<(i32, i32)>, line: i32) -> HashSet<(i32, i32)> {
    if line > 0 {
        fold_x(points, line)
    } else {
        fold_y(points, -line)
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(data: &Data) -> Value {
    fold(&data.points, data.folds[0]).len() as isize
}

#[aoc(day13, part2)]
pub fn solve_part2(data: &Data) -> Value {
    let mut points = data.points.clone();
    for fold_pos in &data.folds {
        points = fold(&points, *fold_pos);
    }
    let min_x = *points.iter().map(|(x, _y)| x).min().unwrap();
    let min_y = *points.iter().map(|(_x, y)| y).min().unwrap();
    let max_x = *points.iter().map(|(x, _y)| x).max().unwrap();
    let max_y = *points.iter().map(|(_x, y)| y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.contains(&(x, y)) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
    0
}
