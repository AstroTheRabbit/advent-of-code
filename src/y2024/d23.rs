use std::collections::{HashMap, HashSet};

use crate::include_input;

const INPUT: &str = include_input!("2024", "23");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Computer {
    pub first: char,
    pub second: char,
}

impl Computer {
    pub const fn new(first: char, second: char) -> Self {
        Self { first, second }
    }

    pub fn load_connections() -> Vec<(Self, Self)> {
        let mut connections = Vec::new();
        for line in INPUT.lines() {
            let chars = line.chars().collect::<Vec<_>>();
            let a = Self::new(chars[0], chars[1]);
            let b = Self::new(chars[3], chars[4]);
            connections.push((a, b));
        }
        return connections;
    }
}

pub fn solve_pt1() -> u32 {
    let connections = Computer::load_connections();
    // * `connected_to` stores a list of every computer along with its 'primary' connections (i.e. each connection in the provided input).
    let mut connected_to: HashMap<Computer, HashSet<Computer>> = HashMap::with_capacity(connections.len());

    for (a, b) in connections {
        if let Some(group_a) = connected_to.get_mut(&a) {
            group_a.insert(b);
        } else {
            connected_to.insert(a, HashSet::from_iter([b]));
        }
        if let Some(group_b) = connected_to.get_mut(&b) {
            group_b.insert(a);
        } else {
            connected_to.insert(b, HashSet::from_iter([a]));
        }
    }

    let mut triplets = HashSet::new();
    for (&a, conn_a) in &connected_to {
        for (&b, conn_b) in &connected_to {
            if !conn_a.contains(&b) {
                continue;
            }
            for (&c, _) in &connected_to {
                let triplet = [a, b, c];
                if triplet.iter().all(|comp| comp.first != 't') {
                    // * None of the computers' names start with 't'.
                    continue;
                }

                if !conn_a.contains(&c) || !conn_b.contains(&c) {
                    continue;
                }
                let variants = [
                    [a, c, b],
                    [b, a, c],
                    [b, c, a],
                    [c, a, b],
                    [c, b, a],
                ];
                if !variants.into_iter().any(|v| triplets.contains(&v)) {
                    triplets.insert(triplet);
                }
            }
        }
    }
    return triplets.len() as u32;

}

pub fn solve_pt2() -> String {
    let connections = Computer::load_connections();
    // * `connected_to` stores a list of every computer along with its 'primary' connections (i.e. each connection in the provided input).
    let mut connected_to: HashMap<Computer, HashSet<Computer>> = HashMap::with_capacity(connections.len());

    for (a, b) in connections {
        if let Some(group_a) = connected_to.get_mut(&a) {
            group_a.insert(b);
        } else {
            connected_to.insert(a, HashSet::from_iter([b]));
        }
        if let Some(group_b) = connected_to.get_mut(&b) {
            group_b.insert(a);
        } else {
            connected_to.insert(b, HashSet::from_iter([a]));
        }
    }

    let mut largest = 0;
    let mut password = String::new();
    let mut group = HashMap::new();
    let mut stack = Vec::new();
    for &start in connected_to.keys() {
        group.clear();
        stack.clear();
        stack.push(start);
        while let Some(from) = stack.pop() {
            if group.contains_key(&from) || group.values().any(|conn: &&HashSet<_>| !conn.contains(&from)) {
                continue;
            }
            let connections = &connected_to[&from];
            group.insert(from, connections);
            for to in connections {
                stack.push(*to);
            }
        }
        if group.len() > largest {
            largest = group.len();
            let mut names = group.keys().map(|c| format!("{}{}", c.first, c.second)).collect::<Vec<_>>();
            names.sort();
            password = names.join(",");
        }
    }
    return password;
}