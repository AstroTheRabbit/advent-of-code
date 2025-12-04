use crate::include_input;

const INPUT: &str = include_input!("2025", "3");

fn load_banks() -> Vec<Vec<char>> {
    return INPUT.lines().map(|l| l.chars().collect()).collect();
}

pub fn solve_pt1() -> u32 {
    let mut sum = 0;
    for bank in load_banks() {
        let mut best_joltage = 0;
        let len = bank.len();
        for i in 0..(len - 1) {
            for j in (i + 1)..len {
                let joltage = format!("{}{}", bank[i], bank[j]).parse().unwrap();
                if joltage > best_joltage {
                    best_joltage = joltage;
                }
            }
        }
        sum += best_joltage;
    }
    return sum;
}

pub fn solve_pt2() -> u64 {
    unimplemented!();
    // ? https://www.reddit.com/r/adventofcode/comments/1pd5hm5/2025_day_3_my_approach_visualized/

    // const NUM_DIGITS: usize = 12;
    // let mut sum = 0;

    // for mut bank in load_banks() {
    
    // }
    // return sum;
}