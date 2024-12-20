use std::{collections::{HashMap, HashSet}, ops::{Add, Mul, Sub}};

use crate::include_input;

const INPUT: &str = include_input!("2024", "20");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn out_of_bounds(&self, pos: &Self) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x > self.x || pos.y > self.y
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

impl Mul<Pos> for isize {
    type Output = Pos;

    fn mul(self, rhs: Pos) -> Self::Output {
        return Pos::new(self * rhs.x, self * rhs.y);
    }
}

struct Map {
    pub start_pos: Pos,
    pub end_pos: Pos,
    pub bounds: Pos,
    pub walls: HashSet<Pos>,
}

impl Map {
    pub fn load() -> Self {
        let mut start_pos = Pos::new(0, 0);
        let mut end_pos = Pos::new(0, 0);
        let mut bounds = Pos::new(0, 0);
        let mut walls = HashSet::new();

        for (y, line) in INPUT.lines().enumerate() {
            for (x, c) in line.char_indices() {
                let pos = Pos::new(x as isize, y as isize);
                bounds.x = Ord::max(bounds.x, pos.x);
                bounds.y = Ord::max(bounds.y, pos.y);
                match c {
                    '#' => {
                        walls.insert(pos);
                    },
                    'S' => {
                        start_pos = pos;
                    },
                    'E' => {
                        end_pos = pos;
                    },
                    '.' => {},
                    _ => panic!("Invalid map input!"),
                }
            }
        }
        return Self { start_pos, end_pos, bounds, walls };
    }
}

pub fn solve_pt1() -> u32 {
    let map = Map::load();

    // * `times` stores the time required to get to any position without using a cheat.
    let mut times = HashMap::new();
    let mut stack = vec![(map.start_pos, 0)];

    while let Some((pos, time)) = stack.pop() {
        if map.bounds.out_of_bounds(&pos) || map.walls.contains(&pos) {
            // * `pos` is either at the end, in a wall, or out of the map's bounds.
            continue;
        }
        match times.get_mut(&pos) {
            Some(local_time) => {
                if *local_time > time {
                    *local_time = time;
                } else {
                    // * `pos` has already been visited with a quicker `time`.
                    continue;
                }
            },
            None => {
                times.insert(pos, time);
            },
        }
        if pos == map.end_pos {
            continue;
        }
        for dir in Pos::ALL_DIRS {
            stack.push((pos + dir, time + 1u32));
        }
    }
    
    // * We now subtract each position's local time from the best (non-cheated) time at the end postion,
    // * removing any negative times (since a negative time would mean that it takes more time than the non-cheated path to get to the end).
    let mut inv_times = HashMap::new();
    let best_end = times[&map.end_pos];
    for (pos, time) in times {
        if let Some(inv_time) = best_end.checked_sub(time) {
            inv_times.insert(pos, inv_time);
        }
    }
    
    // * We now look for pairs of positions that have the correct gap and result in a `delta_time` that is >= 100.
    // * Since a gap with no walls between the positions (i.e. a 'regular' path) will only have a `delta_time` of at most 2,
    // * we don't have to explicitly check for a wall between the positions.

    // TODO: Figure out a way to get rid of this nested loop, since it's really slow.

    let gaps: HashSet<Pos> = Pos::ALL_DIRS.into_iter().map(|d| 2 * d).collect();
    let mut res = 0;
    
    for (&start_pos, &start_time) in &inv_times {
        for (&end_pos, &end_time) in &inv_times {
            // * -2 for the time taking moving through the removed wall.
            if let Some(delta_time) = end_time.checked_sub(start_time + 2) {
                let delta_pos = end_pos - start_pos;
                if gaps.contains(&delta_pos) && delta_time >= 100 {
                    res += 1;
                }
            }
        }
    }
    return res;
}

pub fn solve_pt2() -> u32 {
    let map = Map::load();

    // * `times` stores the time required to get to any position without using a cheat.
    let mut times = HashMap::new();
    let mut stack = vec![(map.start_pos, 0)];

    while let Some((pos, time)) = stack.pop() {
        if map.bounds.out_of_bounds(&pos) || map.walls.contains(&pos) {
            // * `pos` is either at the end, in a wall, or out of the map's bounds.
            continue;
        }
        match times.get_mut(&pos) {
            Some(local_time) => {
                if *local_time > time {
                    *local_time = time;
                } else {
                    // * `pos` has already been visited with a quicker `time`.
                    continue;
                }
            },
            None => {
                times.insert(pos, time);
            },
        }
        if pos == map.end_pos {
            continue;
        }
        for dir in Pos::ALL_DIRS {
            stack.push((pos + dir, time + 1u32));
        }
    }
    
    // * We now subtract each position's local time from the best (non-cheated) time at the end postion,
    // * removing any negative times (since a negative time would mean that it takes more time than the non-cheated path to get to the end).
    let mut inv_times = HashMap::new();
    let best_end = times[&map.end_pos];
    for (pos, time) in times {
        if let Some(inv_time) = best_end.checked_sub(time) {
            inv_times.insert(pos, inv_time);
        }
    }
    
    // * We now look for pairs of positions that have the correct gap and result in a `delta_time` that is >= 100.
    // * Instead of part 1's 'gap' definition, we'll just use the 'taxicab' distance to determine the the time used during the cheat.

    // TODO: Figure out a way to get rid of this nested loop, since it's really slow.

    let mut res = 0;
    for (&start_pos, &start_time) in &inv_times {
        for (&end_pos, &end_time) in &inv_times {
            let delta_pos = end_pos - start_pos;
            let dist = delta_pos.x.abs() + delta_pos.y.abs();
            // * -dist for the time taking moving through the removed wall(s).
            if let Some(delta_time) = end_time.checked_sub(start_time + dist as u32) {
                if dist <= 20 && delta_time >= 100 {
                    res += 1;
                }
            }
        }
    }
    return res;
}