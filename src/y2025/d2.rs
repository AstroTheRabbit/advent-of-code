use std::ops::RangeInclusive;

use crate::include_input;

const INPUT: &str = include_input!("2025", "2");

fn load_ranges() -> Vec<RangeInclusive<u64>> {
    return INPUT
        .split(',')
        .filter_map(|r| r.split_once('-'))
        .map(|(l, u)| l.parse().unwrap()..=u.parse().unwrap())
        .collect();
}

pub fn solve_pt1() -> u64 {
    let ranges = load_ranges();
    let mut sum = 0;

    for range in ranges {
        for id in range {
            let str = id.to_string();
            let len = str.len() / 2;
            let (a, b) = str.split_at(len);
            if a == b {
                sum += id;
            }
        }
    }
    return sum;
}

pub fn solve_pt2() -> u64 {
    let ranges = load_ranges();
    let mut sum = 0;

    for range in ranges {
        for id in range {
            let str = id.to_string().chars().collect::<Vec<_>>();
            let len = str.len();
            for div in 1..=(len / 2) {
                if len % div == 0 {
                    // * `str` can be evenly divided into `div` chunks.
                    let chunk = &str[0..div];
                    if str.chunks_exact(div).all(|c| c == chunk) {
                        // * Every chunk is the same!
                        sum += id;
                        // * Make sure not to count the same id multiple times...
                        break;
                    }
                }
            }
        }
    }
    return sum;
}
