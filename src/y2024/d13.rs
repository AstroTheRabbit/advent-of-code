use std::ops::{Add, Mul, Sub};

use crate::include_input;

const INPUT: &str = include_input!("2024", "13");

const COST_A: u64 = 3;
const COST_B: u64 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    pub x: u64,
    pub y: u64,
}

impl Pos {
    pub const fn new(x: u64, y: u64) -> Self {
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

impl Mul<Pos> for u64 {
    type Output = Pos;

    fn mul(self, rhs: Pos) -> Self::Output {
        return Pos::new(self * rhs.x, self * rhs.y);
    }
}

#[derive(Debug)]
struct Machine {
    pub button_a: Pos,
    pub button_b: Pos,
    pub prize: Pos,
}

impl Machine {
    fn read_num(chars: &mut impl Iterator<Item = char>) -> u64 {
        return chars.take_while(char::is_ascii_digit).collect::<String>().parse().unwrap();
    }

    pub fn load() -> Vec<Self> {
        let mut res = Vec::new();
        for m in INPUT.split("\n\n") {
            let mut chars = m.chars();
            // * Skip "Button A: X+", read `ax`.
            let mut chars = chars.by_ref().skip(12);
            let ax = Self::read_num(&mut chars);
            // * Skip " Y+", read `ay`.
            let mut chars = chars.by_ref().skip(3);
            let ay = Self::read_num(&mut chars);
            // * Skip "Button B: X+", read `bx`.
            let mut chars = chars.by_ref().skip(12);
            let bx = Self::read_num(&mut chars);
            // * Skip " Y+", read `by`.
            let mut chars = chars.by_ref().skip(3);
            let by = Self::read_num(&mut chars);
            // * Skip "Prize: X=", read `px`.
            let mut chars = chars.by_ref().skip(9);
            let px = Self::read_num(&mut chars);
            // * Skip " Y=", read `py`.
            let mut chars = chars.by_ref().skip(3);
            let py = Self::read_num(&mut chars);
            
            res.push(Machine { button_a: Pos::new(ax, ay), button_b: Pos::new(bx, by), prize: Pos::new(px, py) });
        }
        return res;
    }
}

pub fn solve_pt1() -> u64 {
    let machines = Machine::load();
    let mut res = 0;

    for machine in machines {
        // ? The following can be derived from the equation "a * A + b * B = P":
        // ?     `A`, `B`, and `P` are their respective 'machine constants' - `button_a`, `button_b`, and `prize` respectively.
        // ?     `a` and `b` are the amount of times each button is pressed, and so must be positive integers.
        // ?     This equation can be solved for `a` and `b` simultaneously by using the X and Y coords of `A`, `B`, and `P`.

        let a = machine.button_a;
        let b = machine.button_b;
        let p = machine.prize;
        
        let d = u64::abs_diff(a.x * b.y, a.y * b.x);
        let na = u64::abs_diff(p.x * b.y, p.y * b.x);
        let nb = u64::abs_diff(p.y * a.x, p.x * a.y);

        if na % d == 0 && nb % d == 0 {
            let a = na / d;
            let b = nb / d;
            res += (a * COST_A) + (b * COST_B);
        }
    }
    return res as u64;
}

pub fn solve_pt2() -> u64 {
    let machines = Machine::load();
    let mut res = 0;

    for machine in machines {
        let a = machine.button_a;
        let b = machine.button_b;
        let p = machine.prize + Pos::new(10000000000000, 10000000000000);

        let d = u64::abs_diff(a.x * b.y, a.y * b.x);
        let na = u64::abs_diff(p.x * b.y, p.y * b.x);
        let nb = u64::abs_diff(p.y * a.x, p.x * a.y);

        if na % d == 0 && nb % d == 0 {
            let a = na / d;
            let b = nb / d;
            res += (a * COST_A) + (b * COST_B);
        }
    }
    return res as u64;
}