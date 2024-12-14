use std::collections::HashMap;

use crate::include_input;

const INPUT: &str = include_input!("2024", "11");

/// Add `count` to the value of `map[key]`, or insert it if not found.
fn add_or_insert(map: &mut HashMap<u64, u64>, key: u64, count: u64) {
    match map.get_mut(&key) {
        Some(value) => {
            *value += count;
        }
        None => {
            map.insert(key, count);
        }
    }
}

fn load_stones() -> HashMap<u64, u64> {
    return INPUT.split(' ').map(|n| (n.parse().unwrap(), 1)).collect();
}

pub fn solve_pt1() -> u64 {
    const NUM_BLINKS: u64 = 25;
    let mut stones = load_stones();
    let mut next = HashMap::new();

    for _ in 0..NUM_BLINKS {
        for (value, count) in stones.drain() {
            if value == 0 {
                // * Rule 1
                add_or_insert(&mut next, 1, count);
                continue;
            }
            let as_str = value.to_string();
            let str_len = as_str.len();
            if str_len % 2 == 0 {
                // * Rule 2
                let (first, second) = as_str.split_at(str_len / 2);
                let first = first.parse().unwrap();
                let second = second.parse().unwrap();
                add_or_insert(&mut next, first, count);
                add_or_insert(&mut next, second, count);
            } else {
                // * Rule 3
                add_or_insert(&mut next, 2024 * value, count);
            }
        }
        std::mem::swap(&mut stones, &mut next);
    }
    return stones.into_values().sum();
}

pub fn solve_pt2() -> u64 {
    const NUM_BLINKS: u64 = 75;
    let mut stones = load_stones();
    let mut next = HashMap::new();

    for _ in 0..NUM_BLINKS {
        for (value, count) in stones.drain() {
            if value == 0 {
                // * Rule 1
                add_or_insert(&mut next, 1, count);
                continue;
            }
            let as_str = value.to_string();
            let str_len = as_str.len();
            if str_len % 2 == 0 {
                // * Rule 2
                let (first, second) = as_str.split_at(str_len / 2);
                let first = first.parse().unwrap();
                let second = second.parse().unwrap();
                add_or_insert(&mut next, first, count);
                add_or_insert(&mut next, second, count);
            } else {
                // * Rule 3
                add_or_insert(&mut next, 2024 * value, count);
            }
        }
        std::mem::swap(&mut stones, &mut next);
    }
    return stones.into_values().sum();
}
