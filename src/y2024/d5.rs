use crate::include_input;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_input!("2024", "5");

struct Pages {
    pub ordering: HashMap<u32, HashSet<u32>>,
    pub updates: Vec<Vec<u32>>,
}

impl Pages {
    pub fn load() -> Self {
        let (ordering_input, updates_input) = INPUT.split_once("\n\n").unwrap();

        let mut ordering: HashMap<u32, HashSet<u32>> = HashMap::new();
        for line in ordering_input.lines() {
            let (num1, num2) = line.split_once('|').unwrap();
            let num1 = num1.parse().unwrap();
            let num2 = num2.parse().unwrap();

            if let Some(ord_vals) = ordering.get_mut(&num1) {
                ord_vals.insert(num2);
            } else {
                ordering.insert(num1, HashSet::from([num2]));
            }
        }

        let mut updates = Vec::new();
        for line in updates_input.lines() {
            let mut update = Vec::new();
            for num in line.split(',') {
                update.push(num.parse().unwrap());
            }
            updates.push(update);
        }

        return Self { ordering, updates };
    }
}

pub fn solve_pt1() -> u32 {
    let pages = Pages::load();
    let mut res = 0;

    for update in pages.updates {
        let len = update.len();
        let mut passed = true;
        'i: for i in 0..len {
            // * Iterate through every page number in this update.
            let must_precede = update[i];
            let order_check = &pages.ordering[&must_precede];
            for j in 0..i {
                // * Iterate through every page number before the current one.
                let must_succeed = update[j];
                if order_check.contains(&must_succeed) {
                    // * `must_succeed` came before `must_precede` when it shouldn't, this update failed the check.
                    passed = false;
                    break 'i;
                }
            }
        }
        if passed {
            res += update[len / 2];
        }
    }
    return res;
}

pub fn solve_pt2() -> u32 {
    let pages = Pages::load();
    let mut res = 0;

    for mut update in pages.updates {
        let len = update.len();
        let mut was_incorrect = false;
        'sort: loop {
            for i in 0..len {
                // * Iterate through every page number in this update.
                let must_precede = update[i];
                let order_check = &pages.ordering[&must_precede];
                for j in 0..i {
                    // * Iterate through every page number before the current one.
                    let must_succeed = update[j];
                    if order_check.contains(&must_succeed) {
                        // * `must_succeed` came before `must_precede` when it shouldn't, swapping their positions and rechecking.
                        update.swap(i, j);
                        was_incorrect = true;
                        continue 'sort;
                    }
                }
            }
            // * `update` is ordered correctly!
            break;
        }
        if was_incorrect {
            // * The pt. 2 puzzle only asks for the 'middle number sum' of updates that weren't already ordered correctly.
            res += update[len / 2];
        }
    }
    return res;
}
