use crate::include_input;
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

const INPUT: &str = include_input!("2024", "10");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const PX: Self = Self::new(1, 0);
    pub const PY: Self = Self::new(0, 1);
    pub const NX: Self = Self::new(-1, 0);
    pub const NY: Self = Self::new(0, -1);

    pub const ALL_DIRS: [Self; 4] = [Self::PX, Self::PY, Self::NX, Self::NY];
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

struct Map {
    pub heights: HashMap<Pos, u32>,
    pub trailheads: HashSet<Pos>,
}

impl Map {
    pub fn load() -> Self {
        let mut heights = HashMap::new();
        let mut trailheads = HashSet::new();

        for (y, line) in INPUT.lines().enumerate() {
            let y = y as isize;
            for (x, c) in line.char_indices() {
                let pos = Pos::new(x as isize, y);
                let height = c.to_digit(10).unwrap();
                heights.insert(pos, height);
                if height == 0 {
                    trailheads.insert(pos);
                }
            }
        }
        return Self {
            heights,
            trailheads,
        };
    }
}

pub fn solve_pt1() -> u32 {
    let map = Map::load();
    let mut buffer = Vec::from_iter(map.trailheads.into_iter().enumerate());
    // * Each trail of a trailhead must have both a unique ending position for pt. 1.
    let mut completed_trails = HashSet::new();

    while let Some((trail_id, pos)) = buffer.pop() {
        let current_height = map.heights[&pos];
        if current_height == 9 {
            completed_trails.insert((trail_id, pos));
            continue;
        }
        for dir in Pos::ALL_DIRS {
            let next = pos + dir;
            if let Some(next_height) = map.heights.get(&next) {
                if *next_height == current_height + 1 {
                    buffer.push((trail_id, next));
                }
            }
        }
    }
    return completed_trails.len() as u32;
}

pub fn solve_pt2() -> u32 {
    let map = Map::load();
    let mut buffer = Vec::from_iter(map.trailheads.into_iter());
    // * Pt. 2 no longer requires unique ending positions for each trail of a trailhead.
    let mut res = 0;

    while let Some(pos) = buffer.pop() {
        let current_height = map.heights[&pos];
        if current_height == 9 {
            res += 1;
            continue;
        }
        for dir in Pos::ALL_DIRS {
            let next = pos + dir;
            if let Some(next_height) = map.heights.get(&next) {
                if *next_height == current_height + 1 {
                    buffer.push(next);
                }
            }
        }
    }
    return res;
}
