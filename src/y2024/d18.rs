use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, Sub},
};

use crate::include_input;

const INPUT: &str = include_input!("2024", "18");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn out_of_bounds(&self, bounds: &Self) -> bool {
        return self.x < 0 || self.y < 0 || self.x > bounds.x || self.y > bounds.y;
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

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

fn load_bytes() -> Vec<Pos> {
    let mut res = Vec::new();
    for l in INPUT.lines() {
        let mut chars = l.chars();
        let x = chars
            .by_ref()
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .unwrap();
        let y = chars
            .by_ref()
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .unwrap();
        res.push(Pos::new(x, y));
    }
    return res;
}

pub fn solve_pt1() -> u32 {
    const BOUNDS: Pos = Pos::new(70, 70);
    let bytes = load_bytes();
    let mut space = HashSet::new();

    for b in bytes.into_iter().take(1024) {
        space.insert(b);
    }

    let mut best_score = u32::MAX;
    let mut stack = VecDeque::new();
    let mut checked = HashSet::new();
    stack.push_back((Pos::new(0, 0), 0));

    while let Some((pos, score)) = stack.pop_front() {
        if !checked.insert(pos)
            || space.contains(&pos)
            || pos.out_of_bounds(&BOUNDS)
            || score > best_score
        {
            continue;
        }
        if pos == BOUNDS {
            if score < best_score {
                best_score = score;
            }
            continue;
        }
        for dir in Pos::ALL_DIRS {
            stack.push_back((pos + dir, score + 1));
        }
    }
    return best_score;
}

pub fn solve_pt2() -> String {
    const BOUNDS: Pos = Pos::new(70, 70);
    let bytes = load_bytes();
    let mut space = HashSet::new();
    let mut checked = HashSet::new();
    let mut stack = Vec::new();

    'bytes: for b in bytes {
        space.insert(b);
        checked.clear();
        stack.clear();
        stack.push(Pos::new(0, 0));

        while let Some(pos) = stack.pop() {
            if !checked.insert(pos) || space.contains(&pos) || pos.out_of_bounds(&BOUNDS) {
                continue;
            }
            if pos == BOUNDS {
                continue 'bytes;
            }
            for dir in Pos::ALL_DIRS {
                stack.push(pos + dir);
            }
        }
        return format!("{},{}", b.x, b.y);
    }
    panic!("No byte blocks the exit!");
}
