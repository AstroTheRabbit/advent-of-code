use std::{collections::HashSet, ops::{Add, Sub}};

use crate::include_input;

const INPUT: &str = include_input!("2025", "4");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: usize,
    pub y: usize,
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

impl Pos {
    #[inline]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    const PX: Self = Self::new(1, 0);
    const PY: Self = Self::new(0, 1);

    pub fn neighbours(&self) -> Vec<Self> {
        let mut neighbours = Vec::new();
        neighbours.push(*self + Self::PX);
        neighbours.push(*self + Self::PY);
        neighbours.push(*self + Self::PX + Self::PY);
        if self.x > 0 && self.y > 0 {
            neighbours.push(*self - Self::PX - Self::PY);
        }
        if self.x > 0 {
            neighbours.push(*self - Self::PX);
            neighbours.push(*self - Self::PX + Self::PY);
        }
        if self.y > 0 {
            neighbours.push(*self - Self::PY);
            neighbours.push(*self - Self::PY + Self::PX);
        }
        return neighbours;
    }
}

fn load_grid() -> HashSet<Pos> {
    let mut grid = HashSet::new();
    for (y, l) in INPUT.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '@' {
                let pos = Pos::new(x, y);
                assert!(grid.insert(pos));
            }
        }
    }
    return grid;
}

pub fn solve_pt1() -> u32 {
    let grid = load_grid();
    let mut count = 0;
    for pos in &grid {
        let c = pos
            .neighbours()
            .into_iter()
            .filter(|n| grid.contains(n))
            .count();
        if c < 4 {
            count += 1;
        }
    }
    return count;
}

pub fn solve_pt2() -> u32 {
	let mut grid = load_grid();
    let mut remove = HashSet::new();
    let mut count = 0;
    loop {
        for pos in &grid {
            let c = pos
                .neighbours()
                .into_iter()
                .filter(|n| grid.contains(n))
                .count();
            if c < 4 {
                remove.insert(*pos);
            }
        }

        let len = remove.len() as u32;
        if len == 0 {
            break;
        } else {
            count += len;
            grid.retain(|p| !remove.contains(p));
            remove.clear();
        }
    }
    return count;
}