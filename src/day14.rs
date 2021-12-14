use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
// use rayon::prelude::*;

#[derive(Debug)]
pub struct Data {
    init: String,
    rules: HashMap<(u8, u8), u8>,
}
pub type Value = usize;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Data {
    let mut lines = input.lines();
    let init = lines.next().unwrap().to_owned();
    lines.next();
    let mut rules = HashMap::new();
    for rule in lines {
        let (from, to) = rule.split_at(2);
        let from = from.as_bytes();
        let from = (from[0], from[1]);
        let (_, to) = to.split_at(4);
        let to = to.as_bytes()[0];
        rules.insert(from, to);
    }
    Data { init, rules }
}

fn solve_steps(steps: usize, data: &Data) -> Value {
    let mut polymer = HashMap::new();
    // Insert pair of elements:
    for w in data.init.as_bytes().windows(2) {
        polymer.insert((w[0], w[1]), 1);
    }

    // Start element:
    polymer.insert((0, data.init.as_bytes()[0]), 1);
    // End element:
    polymer.insert((*data.init.as_bytes().last().unwrap(), 0), 1);

    for _ in 0..steps {
        let mut new = HashMap::new();
        for (k, v) in polymer {
            if let Some(c) = data.rules.get(&k) {
                // These pairs should get a new element between them
                // println!("  -> {}{} -> {}", k.0 as char, k.1 as char, *c as char);
                *new.entry((k.0, *c)).or_insert(0) += v;
                *new.entry((*c, k.1)).or_insert(0) += v;
            } else {
                // These pairs should remain
                // println!("  -> {}{}", k.0 as char, k.1 as char);
                *new.entry((k.0, k.1)).or_insert(0) += v;
            }
        }
        polymer = new;
        /*
        for (k, v) in &polymer {
            println!("{}{}: {}", k.0 as char, k.1 as char, v)
        }
        */
    }

    // Sum up elements:
    let mut elements = HashMap::new();
    for (k, v) in &polymer {
        if k.0 != 0 {
            *elements.entry(k.0).or_insert(0_usize) += v;
        }
    }

    /*
    for (k, v) in &elements {
        println!("{}: {}", *k as char, *v);
    }
    */
    let min = elements.values().min().unwrap();
    let max = elements.values().max().unwrap();
    max - min
}

#[aoc(day14, part1)]
pub fn solve_part1(data: &Data) -> Value {
    solve_steps(10, data)
}

#[aoc(day14, part2)]
pub fn solve_part2(data: &Data) -> Value {
    solve_steps(40, data)
}
