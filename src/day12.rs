use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
// use rayon::prelude::*;

pub type Data = (String, String);
pub type Value = isize;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|s| {
            let mut iter = s.split('-');
            (
                iter.next().unwrap().to_owned(),
                iter.next().unwrap().to_owned(),
            )
        })
        .collect()
}

fn paths_from(
    current: &str,
    vertexes: &HashSet<&str>,
    edges: &HashMap<&str, HashSet<&str>>,
    small_caves: &HashSet<&str>,
    visited_small_caves: &HashSet<&str>,
    used_edges: &HashSet<(&str, &str)>,
) -> Value {
    if current == "end" {
        return 1;
    }
    let mut paths = 0;

    let mut visited_small_caves = visited_small_caves.clone();
    let mut used_edges = used_edges.clone();

    for edge in edges.get(current).unwrap() {
        if used_edges.contains(&(current, edge)) {
            continue;
        }
        if small_caves.contains(edge) {
            if visited_small_caves.contains(edge) {
                continue;
            }
            visited_small_caves.insert(edge);
        }
        used_edges.insert((current, *edge));
        paths += paths_from(
            edge,
            vertexes,
            edges,
            small_caves,
            &visited_small_caves,
            &used_edges,
        );
        visited_small_caves.remove(edge);
        used_edges.remove(&(current, edge));
    }
    paths
}

#[aoc(day12, part1)]
pub fn solve_part1(data: &[Data]) -> Value {
    let mut vertexes = HashSet::new();
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (v1, v2) in data {
        vertexes.insert(v1.as_str());
        vertexes.insert(v2.as_str());

        if v1 != "end" && v2 != "start" {
            if let Some(l) = edges.get_mut(v1.as_str()) {
                l.insert(v2);
            } else {
                let mut verts = HashSet::new();
                verts.insert(v2.as_str());
                edges.insert(v1.as_str(), verts);
            }
        }
        if v2 != "end" && v1 != "start" {
            if let Some(l) = edges.get_mut(v2.as_str()) {
                l.insert(v1.as_str());
            } else {
                let mut verts = HashSet::new();
                verts.insert(v1.as_str());
                edges.insert(v2.as_str(), verts);
            }
        }
    }
    let mut small_caves = HashSet::new();
    for vert in &vertexes {
        if vert.chars().all(|c| c.is_ascii_lowercase()) {
            small_caves.insert(*vert);
        }
    }

    paths_from(
        "start",
        &vertexes,
        &edges,
        &small_caves,
        &HashSet::new(),
        &HashSet::new(),
    )
}

fn paths_from_2(
    current: &str,
    vertexes: &HashSet<&str>,
    edges: &HashMap<&str, HashSet<&str>>,
    small_caves: &HashSet<&str>,
    visited_small_caves: &HashSet<&str>,
    double_visit: Option<&str>,
) -> Value {
    if current == "end" {
        return 1;
    }
    let mut paths = 0;

    let mut visited_small_caves = visited_small_caves.clone();
    let mut double_visit = double_visit;

    for edge in edges.get(current).unwrap() {
        if small_caves.contains(edge) {
            if visited_small_caves.contains(edge) {
                if double_visit.is_some() {
                    continue;
                } else {
                    double_visit = Some(edge);
                }
            }
            visited_small_caves.insert(edge);
        }
        paths += paths_from_2(
            edge,
            vertexes,
            edges,
            small_caves,
            &visited_small_caves,
            double_visit,
        );
        if Some(*edge) == double_visit {
            double_visit = None;
        } else {
            visited_small_caves.remove(edge);
        }
    }
    paths
}

#[aoc(day12, part2)]
pub fn solve_part2(data: &[Data]) -> Value {
    let mut vertexes = HashSet::new();
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (v1, v2) in data {
        vertexes.insert(v1.as_str());
        vertexes.insert(v2.as_str());

        if v1 != "end" && v2 != "start" {
            if let Some(l) = edges.get_mut(v1.as_str()) {
                l.insert(v2);
            } else {
                let mut verts = HashSet::new();
                verts.insert(v2.as_str());
                edges.insert(v1.as_str(), verts);
            }
        }
        if v2 != "end" && v1 != "start" {
            if let Some(l) = edges.get_mut(v2.as_str()) {
                l.insert(v1.as_str());
            } else {
                let mut verts = HashSet::new();
                verts.insert(v1.as_str());
                edges.insert(v2.as_str(), verts);
            }
        }
    }
    let mut small_caves = HashSet::new();
    for vert in &vertexes {
        if vert.chars().all(|c| c.is_ascii_lowercase()) {
            small_caves.insert(*vert);
        }
    }

    paths_from_2(
        "start",
        &vertexes,
        &edges,
        &small_caves,
        &HashSet::new(),
        None,
    )
}
