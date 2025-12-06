use crate::include_input;
use std::{
    cmp::Ordering,
    collections::HashSet,
    ops::{Add, Mul, Sub},
};

const INPUT: &str = include_input!("2024", "14");

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

#[derive(Debug)]
struct Robot {
    pub pos: Pos,
    pub vel: Pos,
}

impl Robot {
    fn read_num(chars: &mut impl Iterator<Item = char>) -> isize {
        return chars
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect::<String>()
            .parse()
            .unwrap();
    }

    pub fn load() -> Vec<Robot> {
        let mut res = Vec::new();
        for line in INPUT.lines() {
            let chars = line.chars();
            // * Skip "p=", read `pos`.
            let mut chars = chars.skip(2);
            let px = Self::read_num(&mut chars);
            let py = Self::read_num(&mut chars);
            // * Skip "v=", read `vel`.
            let mut chars = chars.skip(2);
            let vx = Self::read_num(&mut chars);
            let vy = Self::read_num(&mut chars);
            res.push(Self {
                pos: Pos::new(px, py),
                vel: Pos::new(vx, vy),
            });
        }
        return res;
    }
}

/// Rust's built-in modulus function does handle negatives correctly for this puzzle.
#[inline]
fn pos_mod(mut lhs: isize, rhs: isize) -> isize {
    lhs %= rhs;
    if lhs < 0 {
        lhs += rhs;
    }
    return lhs;
}

pub fn solve_pt1() -> u32 {
    const PERIOD: isize = 100;
    const BOUNDS: Pos = Pos::new(101, 103);
    const HALF: Pos = Pos::new(50, 51);

    let mut count_1 = 0;
    let mut count_2 = 0;
    let mut count_3 = 0;
    let mut count_4 = 0;

    let mut robots = Robot::load();
    for robot in &mut robots {
        let mut final_pos = robot.pos + (PERIOD * robot.vel);
        final_pos.x = pos_mod(final_pos.x, BOUNDS.x);
        final_pos.y = pos_mod(final_pos.y, BOUNDS.y);
        robot.pos = final_pos;

        match (
            Ord::cmp(&final_pos.x, &HALF.x),
            Ord::cmp(&final_pos.y, &HALF.y),
        ) {
            // * `final_pos` is in one of the four quadrants.
            (Ordering::Less, Ordering::Less) => count_1 += 1,
            (Ordering::Less, Ordering::Greater) => count_2 += 1,
            (Ordering::Greater, Ordering::Less) => count_3 += 1,
            (Ordering::Greater, Ordering::Greater) => count_4 += 1,
            // * `final_pos` is in-between between some quadrants.
            _ => continue,
        }
    }
    return count_1 * count_2 * count_3 * count_4;
}

// ? FYI: This puzzle is really annoying, since the prompt provides no real way to determine if the robots are in the correct position other than having the user scan through steps.
// ! For future me - DO NOT add this solution to any tests, as it relies on user input.
#[allow(unreachable_code)]
pub fn solve_pt2() -> u32 {
    const BOUNDS: Pos = Pos::new(101, 103);

    let mut robots = Robot::load();
    let mut positions = HashSet::new();
    let mut checked_positions = HashSet::new();
    for time in 1u32.. {
        positions.clear();
        checked_positions.clear();

        for robot in &mut robots {
            let mut next_pos = robot.pos + robot.vel;
            next_pos.x = pos_mod(next_pos.x, BOUNDS.x);
            next_pos.y = pos_mod(next_pos.y, BOUNDS.y);
            robot.pos = next_pos;
            positions.insert(next_pos);
        }

        // * Search for 'blobs' (using flood fill) larger than some arbitary value to skip random states (This question *really* annoys me).

        let mut max_blob = 0;
        let mut stack = Vec::new();
        while let Some(start_pos) = positions.iter().next().cloned() {
            stack.clear();
            stack.push(start_pos);
            let mut current_blob = 0;
            while let Some(pos) = stack.pop() {
                if positions.remove(&pos) {
                    current_blob += 1;
                    checked_positions.insert(pos);
                    for dir in Pos::ALL_DIRS {
                        stack.push(pos + dir);
                    }
                }
            }
            if current_blob > max_blob {
                max_blob = current_blob;
            }
        }
        if max_blob < 25 {
            // println!("Skipping t = {}...", time);
            continue;
        }

        return time;

        // ? Used for scanning; no longer needed after solution is found.
        // * Print robot positions to terminal.
        // for y in 0..BOUNDS.y {
        //     for x in 0..BOUNDS.x {
        //         let pos = Pos::new(x, y);
        //         if checked_positions.contains(&pos) {
        //             print!("█");
        //         } else {
        //             print!(" ");
        //         }
        //     }
        //     println!();
        // }
        // println!("Is this a tree? (t = {})", time);

        // * Ask user if robots form a tree.
        // let mut user_input = String::new();
        // match std::io::stdin().read_line(&mut user_input) {
        //     Ok(count) => {
        //         if count > 1 {
        //             return time;
        //         }
        //     }
        //     Err(_) => panic!("Error trying to read from stdin!"),
        // }
    }
    panic!("No correct state found after {} steps!", u32::MAX);
}
