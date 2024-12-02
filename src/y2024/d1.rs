use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    iter::zip,
};

const INPUT_PATH: &str = "./inputs/day1.txt";

fn load_lists() -> (Vec<u32>, Vec<u32>) {
    // * Read lists into two Vec<u32>.
    let file = OpenOptions::new().read(true).open(INPUT_PATH).unwrap();
    let mut list1 = Vec::with_capacity(1000);
    let mut list2 = Vec::with_capacity(1000);

    for line in BufReader::new(file).lines().flatten() {
        let num1 = line[0..5].parse::<u32>().unwrap();
        let num2 = line[8..13].parse::<u32>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }

    return (list1, list2);
}

pub fn solve_pt1() -> u32 {
    let (mut list1, mut list2) = load_lists();

    // * Sort the two lists in ascending order.
    list1.sort_unstable();
    list2.sort_unstable();

    // * Sum the differences between each pair of numbers.
    let mut res = 0;
    for (num1, num2) in zip(list1, list2) {
        res += max(num1, num2) - min(num1, num2);
    }
    return res;
}

pub fn solve_pt2() -> u32 {
    let (list1, list2) = load_lists();

    // * Get counts of each number in list2.
    let mut counts = HashMap::new();
    for num in list2 {
        if let Some(count) = counts.get_mut(&num) {
            *count += 1;
        } else {
            counts.insert(num, 1);
        }
    }
    // * Sum each number in list2 with its respective list1 count.
    let mut res = 0;
    for num in list1 {
        if let Some(count) = counts.get(&num) {
            res += num * count;
        }
    }
    return res;
}
