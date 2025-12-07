use std::collections::{HashMap, HashSet};

use crate::include_input;

const INPUT: &str = include_input!("2025", "7");

fn load_diagram() -> (usize, Vec<HashSet<usize>>) {
    let mut lines = INPUT.lines().map(|l| l.char_indices());

    let entrance = lines
        .next()
        .unwrap()
        .find(|(_, c)| *c == 'S')
        .unwrap()
        .0;

    let mut splitters = Vec::new();
    for l in lines {
        let mut line_splitters = HashSet::new();
        for (i, c) in l {
            if c == '^' {
                line_splitters.insert(i);
            }
        }
        splitters.push(line_splitters);
    }

    return (entrance, splitters);
}

pub fn solve_pt1() -> u32 {
    let (entrance, splitters) = load_diagram();

    let mut beams = HashSet::from([entrance]);
    let mut next_beams = HashSet::new();
    let mut count = 0;

    for line_splitters in splitters {
        for b in &beams {
            if line_splitters.contains(b) {
                next_beams.insert(b - 1);
                next_beams.insert(b + 1);
                count += 1;
            } else {
                next_beams.insert(*b);
            }
        }
        std::mem::swap(&mut beams, &mut next_beams);
        next_beams.clear();
    }
    return count;
}

pub fn solve_pt2() -> u64 {
    let (entrance, splitters) = load_diagram();

    let mut beams = HashMap::from([(entrance, 1)]);
    let mut next_beams = HashMap::new();

    fn add_beam(map: &mut HashMap<usize, usize>, idx: usize, count: usize) {
        if let Some(next_count) = map.get_mut(&idx) {
            *next_count += count;
        } else {
            map.insert(idx, count);
        }
    }

    for line_splitters in splitters {
        for (b, c) in &beams {
            if line_splitters.contains(b) {
                add_beam(&mut next_beams, b - 1, *c);
                add_beam(&mut next_beams, b + 1, *c);
            } else {
                add_beam(&mut next_beams, *b, *c);
            }
        }
        std::mem::swap(&mut beams, &mut next_beams);
        next_beams.clear();
    }
    return beams.into_values().sum::<usize>().try_into().unwrap();
}