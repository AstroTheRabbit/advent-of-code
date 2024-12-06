use crate::include_input;
use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

const INPUT: &str = include_input!("2024", "6");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self::new(self.x - rhs.x, self.y - rhs.y);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    PX,
    PY,
    NX,
    NY,
}

impl Dir {
    pub fn rotate_clockwise(&self) -> Self {
        // *   NY
        // * NX  PX
        // *   PY
        match self {
            Self::PX => Self::PY,
            Self::PY => Self::NX,
            Self::NX => Self::NY,
            Self::NY => Self::PX,
        }
    }

    pub fn to_pos(&self) -> Pos {
        match self {
            Self::PX => Pos::new(1, 0),
            Self::PY => Pos::new(0, 1),
            Self::NX => Pos::new(-1, 0),
            Self::NY => Pos::new(0, -1),
        }
    }
}

struct Map {
    // * `bounds` determines the maximum position that the guard can stand in.
    pub bounds: Pos,
    pub guard_start: Pos,
    pub obstacles: HashSet<Pos>,
}

impl Map {
    pub fn load() -> Self {
        let mut bounds_x = 0;
        let mut bounds_y = 0;
        let mut guard_start = None;
        let mut obstacles = HashSet::new();

        for (y, line) in INPUT.lines().enumerate() {
            bounds_y = bounds_y.max(y);
            for (x, c) in line.char_indices() {
                bounds_x = bounds_x.max(x);
                let pos = Pos::new(x as i32, y as i32);
                match c {
                    '#' => {
                        obstacles.insert(pos);
                    }
                    '^' => {
                        guard_start = Some(pos);
                    }
                    _ => continue,
                }
            }
        }
        return Self {
            bounds: Pos::new(bounds_x as i32, bounds_y as i32),
            guard_start: guard_start.unwrap(),
            obstacles,
        };
    }

    // * Check if a position is within the `bounds` of this map.
    pub fn pos_in_bounds(&self, pos: &Pos) -> bool {
        return pos.x >= 0 && pos.y >= 0 && pos.x <= self.bounds.x && pos.y <= self.bounds.y;
    }
}

pub fn solve_pt1() -> u32 {
    let map = Map::load();
    // * 'NY' in the context of text (such as this input) is actually 'up' the page.
    let mut current_pos = map.guard_start;
    let mut current_dir = Dir::NY;
    let mut guard_positions = HashSet::from([current_pos]);

    while map.pos_in_bounds(&current_pos) {
        let new_pos = current_pos + current_dir.to_pos();
        if map.obstacles.contains(&new_pos) {
            current_dir = current_dir.rotate_clockwise();
        } else {
            current_pos = new_pos;
            guard_positions.insert(current_pos);
        }
    }
    return guard_positions.len() as u32;
}

// ! This solution is quite slow!
pub fn solve_pt2() -> u32 {
    let map = Map::load();
    let mut res = 0;

    // * 'NY' in the context of text (such as this input) is actually 'up' the page.
    let mut current_pos = map.guard_start;
    let mut current_dir = Dir::NY;
    let mut original_history = HashSet::from([current_pos]);

    while map.pos_in_bounds(&current_pos) {
        let new_pos = current_pos + current_dir.to_pos();
        if map.obstacles.contains(&new_pos) {
            current_dir = current_dir.rotate_clockwise();
        } else {
            current_pos = new_pos;
        }
        original_history.insert(current_pos);
    }

    let mut history = HashSet::new();
    for obstacle_pos in original_history {
        if obstacle_pos == map.guard_start {
            continue;
        }
        history.clear();

        current_pos = map.guard_start;
        current_dir = Dir::NY;

        let mut looped = false;
        while map.pos_in_bounds(&current_pos) {
            let new_pos = current_pos + current_dir.to_pos();
            if map.obstacles.contains(&new_pos) || new_pos == obstacle_pos {
                current_dir = current_dir.rotate_clockwise();
            } else {
                current_pos = new_pos;

                let state = (current_pos, current_dir);
                if history.contains(&state) {
                    looped = true;
                    break;
                }
                history.insert((current_pos, current_dir));
            }
        }
        if looped {
            res += 1;
        }
    }
    return res;
}
