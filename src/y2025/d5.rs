use std::ops::RangeInclusive;

use crate::include_input;

const INPUT: &str = include_input!("2025", "5");

fn load_input() -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    let (a, b) = INPUT.split_once("\n\n").unwrap();

    for l in a.lines() {
        let (lower, upper) = l.split_once('-').unwrap();
        let lower = lower.parse().unwrap();
        let upper = upper.parse().unwrap();
        ranges.push(lower..=upper);
    }
    for l in b.lines() {
        let id = l.parse().unwrap();
        ids.push(id);
    }
    return (ranges, ids);
}

pub fn solve_pt1() -> u32 {
    let (ranges, ids) = load_input();
    let mut count = 0;
    for id in &ids {
        let fresh = ranges.iter().any(|r| r.contains(id));
        if fresh {
            count += 1;
        }
    }
    return count;
}

pub fn solve_pt2() -> u64 {
	let (mut ranges, _) = load_input();
    let mut found_merge = true;

    'M: while found_merge {
        found_merge = false;

        for i in 0..ranges.len() {
            for j in (i + 1)..ranges.len() {
                let a = &ranges[i];
                let b = &ranges[j];

                // * Check if the two ranges overlap.
                // ? https://stackoverflow.com/a/3269471
                let sa = a.start();
                let ea = a.end();
                let sb = b.start();
                let eb = b.end();

                if sa <= eb && sb <= ea {
                    found_merge = true;
                    let start = u64::min(*sa, *sb);
                    let end = u64::max(*ea, *eb);
                    ranges.remove(i);
                    ranges.remove( j - 1);
                    ranges.push(start..=end);
                    continue 'M;
                }
            }
        }
    }
    return ranges.into_iter().flatten().count().try_into().unwrap();
}