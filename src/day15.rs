use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
// use rayon::prelude::*;

#[derive(Debug)]
pub struct Data {
    map: Vec<u32>,
    width: usize,
    height: usize,
}
pub type Value = isize;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Entry {
    cost: isize,
    pos: (usize, usize),
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Data {
    Data {
        map: input
            .lines()
            .map(|s| s.chars().map(|c| c as u32 - 0x30).collect::<Vec<_>>())
            .flatten()
            .collect(),
        width: input.lines().next().unwrap().len(),
        height: input.lines().count(),
    }
}

fn maybe_update_cost(
    dx: isize,
    dy: isize,
    entry: &Entry,
    cost: &mut BinaryHeap<Reverse<Entry>>,
    cost_map: &mut HashMap<(usize, usize), isize>,
    data: &Data,
) {
    let new_pos = (
        (entry.pos.0 as isize + dx) as usize,
        (entry.pos.1 as isize + dy) as usize,
    );
    let new_cost = entry.cost + data.map[new_pos.0 + new_pos.1 * data.width] as isize;
    if let Some(old_cost) = cost_map.get(&new_pos) {
        if *old_cost <= entry.cost + new_cost {
            return;
        }
    }
    cost_map.insert(new_pos, new_cost);
    cost.push(Reverse(Entry {
        pos: new_pos,
        cost: new_cost,
    }));
}

#[aoc(day15, part1)]
pub fn solve_part1(data: &Data) -> Value {
    let mut cost = BinaryHeap::new();
    let end = (data.width - 1, data.height - 1);
    let mut cost_map = HashMap::new();

    cost.push(Reverse(Entry {
        pos: (0, 0),
        cost: 0,
    }));
    cost_map.insert((0, 0), 0);

    while let Some(Reverse(entry)) = cost.pop() {
        // dbg!(&entry);
        if entry.pos == end {
            return entry.cost;
        }
        if entry.pos.0 > 0 {
            maybe_update_cost(-1, 0, &entry, &mut cost, &mut cost_map, data);
        }
        if entry.pos.0 < data.width - 1 {
            maybe_update_cost(1, 0, &entry, &mut cost, &mut cost_map, data);
        }
        if entry.pos.1 > 0 {
            maybe_update_cost(0, -1, &entry, &mut cost, &mut cost_map, data);
        }
        if entry.pos.1 < data.height - 1 {
            maybe_update_cost(0, 1, &entry, &mut cost, &mut cost_map, data);
        }
    }
    0
}

#[aoc(day15, part2)]
pub fn solve_part2(data1: &Data) -> Value {
    let mut data = Data {
        map: vec![],
        width: data1.width * 5,
        height: data1.height * 5,
    };
    for y in 0..5 {
        for y1 in 0..data1.height {
            for x in 0..5 {
                for x1 in 0..data1.width {
                    let mut new_risk = data1.map[x1 + y1 * data1.width] + x + y;
                    while new_risk > 9 {
                        new_risk -= 9;
                    }
                    data.map.push(new_risk);
                }
            }
        }
    }
    solve_part1(&data)
}
