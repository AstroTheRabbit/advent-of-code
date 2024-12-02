use std::iter;

const INPUT: &str = include_str!("../../inputs/y2023/d6.txt");

pub fn solve_pt1() -> u64 {
    let lines = INPUT.lines().collect::<Vec<_>>();
    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());
    let distances = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());

    let mut res = 1;
    
    for (time, dist) in iter::zip(times, distances) {
        let mut count = 0;
        for t in 0..=time {
            let d = t * (time - t);
            if d > dist {
                count += 1;
            }
        }
        res *= count;
    }

    return res;
}

pub fn solve_pt2() -> u64 {
    let lines = INPUT.lines().collect::<Vec<_>>();
    let time = lines[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let dist = lines[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut res = 0;
    for t in 0..=time {
        let d = t * (time - t);
        if d > dist {
            res += 1;
        }
    }
    return res;
}