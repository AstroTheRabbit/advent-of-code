use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/y2023/d4.txt");

#[derive(Debug)]
struct Card {
    pub id: u32,
    pub numbers: Vec<u32>,
    pub winning_numbers: HashSet<u32>,
}

impl Card {}

fn load_cards() -> Vec<Card> {
    let mut res = Vec::new();

    for line in INPUT.lines() {
        // * Skip the 'Card ' prefix of every line.
        let string = line.to_string();
        // * `.skip(1)` to skip the 'Card' prefix of each line.
        let mut iter = string.split_whitespace().skip(1);

        let id = iter
            .next()
            .unwrap()
            .chars()
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let mut finished_winning_numbers = false;
        let mut numbers = Vec::new();
        let mut winning_numbers = HashSet::new();
        for chars in iter {
            if chars == "|" {
                finished_winning_numbers = true;
                continue;
            }

            let num = chars.parse::<u32>().unwrap();
            if finished_winning_numbers {
                numbers.push(num);
            } else {
                winning_numbers.insert(num);
            }
        }
        res.push(Card {
            id,
            numbers,
            winning_numbers,
        });
    }
    return res;
}

pub fn solve_pt1() -> u32 {
    let cards = load_cards();
    let mut res = 0;

    for card in cards {
        let mut score = 0;
        for num in &card.numbers {
            if card.winning_numbers.contains(num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        res += score;
    }
    return res;
}

pub fn solve_pt2() -> u32 {
    let cards = load_cards();

    let mut res = 0;
    let mut max_id = 0;
    let mut stack = Vec::new();
    let mut matches_lookup = HashMap::new();

    for card in cards {
        if card.id > max_id {
            max_id = card.id;
        }
        stack.push(card.id);
        let mut count = 0;
        for num in card.numbers {
            if card.winning_numbers.contains(&num) {
                count += 1;
            }
        }
        matches_lookup.insert(card.id, count);
    }

    while let Some(id) = stack.pop() {
        let num_copies = matches_lookup[&id];
        for rel_id in 1..=num_copies {
            stack.push(id + rel_id);
        }
        res += 1;
    }
    return res;
}
