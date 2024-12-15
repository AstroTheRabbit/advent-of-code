use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use crate::include_input;

const INPUT: &str = include_input!("2024", "15");

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TilePt1 {
    Box,
    Wall,
}

fn load_inputs_pt1() -> (Pos, HashMap<Pos, TilePt1>, Vec<Pos>) {
    let (input_warehouse, input_movements) = INPUT.split_once("\n\n").unwrap();

    let mut warehouse = HashMap::new();
    let mut robot_pos = Pos::new(0, 0);
    for (y, line) in input_warehouse.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let pos = Pos::new(x as isize, y as isize);
            let tile = match c {
                'O' => TilePt1::Box,
                '#' => TilePt1::Wall,
                '@' => {
                    robot_pos = pos;
                    continue;
                }
                '.' => continue,
                _ => panic!("Invalid warehouse input!"),
            };
            warehouse.insert(pos, tile);
        }
    }

    let mut movements = Vec::new();
    for c in input_movements.chars() {
        let dir = match c {
            '>' => Pos::PX,
            'v' => Pos::PY,
            '<' => Pos::NX,
            '^' => Pos::NY,
            '\n' => continue,
            _ => panic!("Invalid movement input!"),
        };
        movements.push(dir);
    }

    return (robot_pos, warehouse, movements);
}

pub fn solve_pt1() -> u32 {
    let (mut robot_pos, mut map, movements) = load_inputs_pt1();

    let mut to_move = HashSet::new();
    for dir in movements {
        to_move.clear();
        let mut current_pos = robot_pos;
        let mut can_move = false;
        loop {
            current_pos = current_pos + dir;
            match map.get(&current_pos) {
                Some(TilePt1::Box) => {
                    to_move.insert(current_pos);
                }
                Some(TilePt1::Wall) => {
                    break;
                }
                None => {
                    can_move = true;
                    break;
                }
            }
        }
        if can_move {
            robot_pos = robot_pos + dir;
            for box_pos in &to_move {
                map.remove(box_pos);
            }
            for &box_pos in &to_move {
                map.insert(box_pos + dir, TilePt1::Box);
            }
        }
    }

    let mut res = 0;
    for (pos, tile) in map {
        if tile == TilePt1::Box {
            res += pos.x + (100 * pos.y);
        }
    }
    return res as u32;
}

#[derive(Debug, PartialEq, Eq)]
enum TilePt2 {
    Wall,
    BoxLeft,
    BoxRight,
}

fn load_inputs_pt2() -> (Pos, HashMap<Pos, TilePt2>, Vec<Pos>) {
    let (input_warehouse, input_movements) = INPUT.split_once("\n\n").unwrap();

    let mut warehouse = HashMap::new();
    let mut robot_pos = Pos::new(0, 0);
    for (y, line) in input_warehouse.lines().enumerate() {
        for (mut x, c) in line.char_indices() {
            x *= 2;
            let pos_left = Pos::new(x as isize, y as isize);
            let pos_right = Pos::new((x + 1) as isize, y as isize);
            match c {
                '#' => {
                    warehouse.insert(pos_left, TilePt2::Wall);
                    warehouse.insert(pos_right, TilePt2::Wall);
                }
                'O' => {
                    warehouse.insert(pos_left, TilePt2::BoxLeft);
                    warehouse.insert(pos_right, TilePt2::BoxRight);
                }
                '@' => {
                    robot_pos = pos_left;
                    continue;
                }
                '.' => continue,
                _ => panic!("Invalid warehouse input!"),
            }
        }
    }

    let mut movements = Vec::new();
    for c in input_movements.chars() {
        let dir = match c {
            '>' => Pos::PX,
            'v' => Pos::PY,
            '<' => Pos::NX,
            '^' => Pos::NY,
            '\n' => continue,
            _ => panic!("Invalid movement input!"),
        };
        movements.push(dir);
    }

    return (robot_pos, warehouse, movements);
}

pub fn solve_pt2() -> u32 {
    let (mut robot_pos, mut map, movements) = load_inputs_pt2();

    // * In the case of part 2, we'll just store the left position of each box in `to_move`.
    let mut to_move = HashSet::new();

    for dir in movements {
        to_move.clear();
        let mut next_pos = robot_pos + dir;

        let left_pos = match map.get_mut(&next_pos) {
            None => {
                // * The next tile is already empty, so it's safe to move without considering boxes.
                robot_pos = next_pos;
                continue;
            }
            Some(TilePt2::Wall) => {
                // * The next tile is a wall, so we don't move anything.
                continue;
            }
            Some(TilePt2::BoxLeft) => next_pos,
            Some(TilePt2::BoxRight) => next_pos + Pos::NX,
        };

        let mut can_move = false;
        if dir.y == 0 {
            // * Boxes are only being moved along the x-axis, so we can use a similar method to pt. 1.
            loop {
                match map.get(&next_pos) {
                    Some(TilePt2::Wall) => break,
                    Some(TilePt2::BoxLeft) => {
                        to_move.insert(next_pos);
                    }
                    Some(TilePt2::BoxRight) => {
                        // * Since we're guaranteed to encounter the left half of this box moving either direction, we don't need to add it to `to_move` here.
                    }
                    None => {
                        can_move = true;
                        break;
                    }
                }
                next_pos = next_pos + dir;
            }
        } else {
            // * Moving along the y-axis, however, is a bit more complicated...
            can_move = true;
            // * `row` stores the left-half position of each box that needs to pushed for some y value.
            let mut row = HashSet::from_iter([left_pos]);
            let mut next_row = HashSet::new();
            while can_move && row.len() > 0 {
                for &pos in &row {
                    to_move.insert(pos);
                    let next_left = pos + dir;
                    let next_right = next_left + Pos::PX;
                    match map.get(&next_left) {
                        Some(TilePt2::Wall) => {
                            can_move = false;
                            break;
                        }
                        Some(TilePt2::BoxLeft) => {
                            next_row.insert(next_left);
                            continue;
                        }
                        Some(TilePt2::BoxRight) => {
                            next_row.insert(next_left + Pos::NX);
                        }
                        None => {}
                    }
                    match map.get(&next_right) {
                        Some(TilePt2::Wall) => {
                            can_move = false;
                            break;
                        }
                        Some(TilePt2::BoxLeft) => {
                            next_row.insert(next_right);
                        }
                        Some(TilePt2::BoxRight) => {
                            // * Since pushing one box into the exact same position as another is already handled by the first `match` statement, reaching this branch is an error.
                            panic!("Box position error!");
                        }
                        None => {}
                    }
                }
                std::mem::swap(&mut row, &mut next_row);
                next_row.clear();
            }
        }

        if can_move {
            robot_pos = robot_pos + dir;
            for &box_pos in &to_move {
                map.remove(&box_pos);
                map.remove(&(box_pos + Pos::PX));
            }
            for &box_pos in &to_move {
                map.insert(box_pos + dir, TilePt2::BoxLeft);
                map.insert(box_pos + dir + Pos::PX, TilePt2::BoxRight);
            }
        }
    }

    let mut res = 0;
    for (pos, tile) in map {
        // * The left side of each box will always be the closest to the top-left corner.
        if tile == TilePt2::BoxLeft {
            res += pos.x + (100 * pos.y);
        }
    }
    return res as u32;
}
