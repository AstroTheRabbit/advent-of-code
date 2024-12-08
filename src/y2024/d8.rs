use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, Sub},
};

use crate::include_input;

const INPUT: &str = include_input!("2024", "8");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

struct Map {
    pub bounds: Pos,
    pub nodes: HashMap<char, Vec<Pos>>,
}

impl Map {
    pub fn load() -> Self {
        let mut bounds = Pos::new(0, 0);
        let mut nodes: HashMap<_, Vec<_>> = HashMap::new();
        for (y, line) in INPUT.lines().enumerate() {
            for (x, c) in line.char_indices() {
                let pos = Pos::new(x as isize, y as isize);
                bounds = pos;
                if c != '.' {
                    if let Some(node) = nodes.get_mut(&c) {
                        node.push(pos);
                    } else {
                        nodes.insert(c, vec![pos]);
                    }
                }
            }
        }
        return Self { bounds, nodes };
    }

    pub fn in_bounds(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x <= self.bounds.x && pos.y <= self.bounds.y
    }
}

pub fn solve_pt1() -> u32 {
    let map = Map::load();
    let mut antinodes = HashSet::new();

    for (_, nodes) in &map.nodes {
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if i == j {
                    continue;
                }
                let antinode = nodes[i] + nodes[i] - nodes[j];
                if map.in_bounds(&antinode) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    return antinodes.len() as u32;
}

pub fn solve_pt2() -> u32 {
    let map = Map::load();
    let mut antinodes = HashSet::new();

    for (_, nodes) in &map.nodes {
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if i == j {
                    continue;
                }
                let mut pos = nodes[i];
                let d = nodes[j] - pos;
                while map.in_bounds(&pos) {
                    antinodes.insert(pos);
                    pos += d;
                }
            }
        }
    }
    return antinodes.len() as u32;
}
