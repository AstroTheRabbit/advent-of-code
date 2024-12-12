use std::{collections::{HashMap, HashSet}, ops::{Add, Sub}};

use crate::include_input;

const INPUT: &str = include_input!("2024", "12");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    // * Rotates this position 90° clockwise.
    pub fn rotate(&self) -> Self {
        return Self::new(self.y, -self.x);
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

struct Map {
    pub plants: HashMap<Pos, char>,
}

impl Map {
    pub fn load() -> Self {
        let mut plants = HashMap::new();
        for (y, line) in INPUT.lines().enumerate() {
            for (x, c) in line.char_indices() {
                let pos = Pos::new(x as isize, y as isize);
                plants.insert(pos, c);
            }
        }
        return Self { plants };
    }
}

pub fn solve_pt1() -> u32 {
    let mut map = Map::load();
    let mut res = 0;
    let mut stack = Vec::new();
    let mut region = HashSet::new();

    while let Some((&start_pos, &region_plant)) = map.plants.iter().next() {
        let mut perimeter = 0;
        stack.clear();
        region.clear();
        stack.push(start_pos);
        
        while let Some(pos) = stack.pop() {
            if !region.contains(&pos) {
                region.insert(pos);
                for dir in Pos::ALL_DIRS {
                    let next = pos + dir;
                    if map.plants.get(&next).is_none_or(|&c| c != region_plant) {
                        // * `next` is not within the current region.
                        perimeter += 1;
                    } else {
                        // * `next` is within the current region.
                        stack.push(pos + dir);
                    }
                }
            }
        }
        res += region.len() * perimeter;

        for p in &region {
            map.plants.remove(p);
        }
    }
    return res as u32;
}

pub fn solve_pt2() -> u32 {
	let mut map = Map::load();
    let mut res = 0;
    let mut stack = Vec::new();
    let mut region = HashSet::new();
    let mut perimeter = HashSet::new();

    while let Some((&start_pos, &region_plant)) = map.plants.iter().next() {
        stack.clear();
        region.clear();
        perimeter.clear();
        stack.push(start_pos);
        
        while let Some(pos) = stack.pop() {
            if !region.contains(&pos) {
                region.insert(pos);
                for dir in Pos::ALL_DIRS {
                    let next = pos + dir;
                    if map.plants.get(&next).is_none_or(|&c| c != region_plant) {
                        // * `next` is not within the current region.
                        perimeter.insert((pos, next));
                    } else {
                        // * `next` is within the current region.
                        stack.push(pos + dir);
                    }
                }
            }
        }

        let mut checked = HashSet::new();
        let mut sides = 0;
        while let Some(pair) = perimeter.iter().next().cloned() {
            perimeter.remove(&pair);

            // ? Sides are detected by "walking their surface":
            // ? We start 'standing' on some part of the perimeter, with our 'body' on the outside of the region.
            // ? Each step forward, we can check that we're still on the 'surface' by:
            // ?     checking if our 'body' has entered the region ("walking into a wall"), or
            // ?     checking if our 'feet' are no longer touching the surface ("walking off a cliff").

            let (inside, outside) = pair;
            let inside_dir = inside - outside;
            let move_dir = inside_dir.rotate();
            let mut current_pos = outside;
            loop {
                let inside_pos = current_pos + inside_dir;
                if !checked.insert((current_pos, inside_pos)) {
                    // * The side containing this perimeter segment was already found.
                    break;
                }
                // ? "Wall & cliff" check
                if !region.contains(&inside_pos) || region.contains(&current_pos) {
                    // * `current_pos` is no longer on a side.
                    sides += 1;
                    break;
                }
                current_pos = current_pos + move_dir;
            }
        }

        res += region.len() * sides;

        for p in &region {
            map.plants.remove(p);
        }
    }
    return res as u32;
}