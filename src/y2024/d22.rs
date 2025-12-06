use crate::include_input;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_input!("2024", "22");

fn next_secret(mut num: u64) -> u64 {
    num = (num ^ (num * 64)) % 16777216;
    num = (num ^ (num / 32)) % 16777216;
    num = (num ^ (num * 2048)) % 16777216;
    return num;
}

fn get_price(num: u64) -> i32 {
    return num.to_string().pop().unwrap().to_digit(10).unwrap() as i32;
}

fn load_input() -> Vec<u64> {
    return INPUT.lines().map(|l| l.parse().unwrap()).collect();
}

pub fn solve_pt1() -> u64 {
    let mut res = 0;
    let input = load_input();

    for mut num in input {
        for _ in 0..2000 {
            num = next_secret(num);
        }
        res += num;
    }
    return res;
}

pub fn solve_pt2() -> i32 {
    let input = load_input();

    let mut prices = Vec::with_capacity(2000);
    let mut changes = Vec::with_capacity(2000);
    let mut sequences = HashMap::new();
    let mut buyer_sequences = HashSet::new();
    for idx in 0..input.len() {
        prices.clear();
        changes.clear();
        buyer_sequences.clear();

        let mut num = input[idx];
        let mut current_price = get_price(num);
        for _ in 0..2000 {
            num = next_secret(num);
            let next_price = get_price(num);
            prices.push(next_price);
            changes.push(next_price - current_price);
            current_price = next_price;
        }
        for i in 3..2000 {
            let price = prices[i];
            if price == 0 {
                continue;
            }
            let seq = [changes[i - 3], changes[i - 2], changes[i - 1], changes[i]];
            // * The 'monkey' only sells when a sequence first occurs from a buyer.
            if buyer_sequences.insert(seq) {
                if let Some(total) = sequences.get_mut(&seq) {
                    *total += price;
                } else {
                    sequences.insert(seq, price);
                }
            }
        }
    }
    return sequences.into_values().max().unwrap();
}
