use crate::include_input;

const INPUT: &str = include_input!("2025", "1");

#[derive(Debug)]
struct Dial(i32);

impl Dial {
    pub fn parse_rot(input: &str) -> i32 {
        // * +rot = right, -rot = left.
        let mut chars = input.chars();
        let dir = chars.next().expect("`input` is empty!");
        let rot = chars.collect::<String>().parse::<i32>().unwrap();
        return match dir {
            'R' => rot,
            'L' => -rot,
            c => panic!("Unexpected character '{}' in input!", c),
        };
    }
    
    pub fn apply_rot(&mut self, rot: i32) -> u32 {
        let mut count = 0;
        let dir = rot.signum();
        for _ in 0..rot.abs() {
            self.0 += dir;
            if self.0 < 0 {
                self.0 += 100;
            }
            if self.0 > 99 {
                self.0 -= 100;
            }
            if self.0 == 0 {
                count += 1;
            }
        }
        return count;
    }
}

pub fn solve_pt1() -> u32 {
    let mut count = 0;
    let mut dial = Dial(50);

    for l in INPUT.lines() {
        let rot = Dial::parse_rot(l);
        dial.apply_rot(rot);
        if dial.0 == 0 {
            count += 1;
        }
    }
    return count;
}

pub fn solve_pt2() -> u32 {
	let mut count = 0;
    let mut dial = Dial(50);

    for l in INPUT.lines() {
        let rot = Dial::parse_rot(l);
        count += dial.apply_rot(rot);
    }
    return count;
}