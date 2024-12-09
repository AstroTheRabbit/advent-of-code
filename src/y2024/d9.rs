use std::{iter::repeat_n, mem::replace};

use crate::include_input;

const INPUT: &str = include_input!("2024", "9");

#[derive(Debug, Clone)]
struct BlockSpan {
    pub length: usize,
    pub file_id: Option<u64>,
}

impl BlockSpan {
    pub fn new_empty(length: usize) -> Self {
        Self { length, file_id: None }
    }

    pub fn new_file(length: usize, file_id: u64) -> Self {
        Self { length, file_id: Some(file_id) }
    }

    pub fn load() -> Vec<Self> {
        let disk_map = INPUT.chars().map(|c| c.to_digit(10).unwrap() as usize);
        let mut spans = Vec::new();

        let mut file_id = 0;
        let mut is_free_space = false;
        for len in disk_map {
            match is_free_space {
                true => {
                    spans.push(Self::new_empty(len));
                    is_free_space = false;
                },
                false => {
                    spans.push(Self::new_file(len, file_id));
                    file_id += 1;
                    is_free_space = true;
                },
            }
        }
        return spans;
    }

    pub fn expand(self) -> impl Iterator<Item = Option<u64>> {
        repeat_n(self.file_id, self.length)
    }
}

pub fn solve_pt1() -> u64 {
    let mut blocks = BlockSpan::load().into_iter().flat_map(BlockSpan::expand).collect::<Vec<_>>();

    let len = blocks.len();
    let mut last_full_idx = len - 1;
    for i in 0..len {
        if blocks[i].is_some() {
            // * Space to fill is already full, skipping...
            continue;
        }
        while blocks[last_full_idx].is_none() {
            // * Get the index of the last non-empty space in `files`.
            last_full_idx -= 1;
        }
        if i >= last_full_idx {
            // * All gaps have been filled.
            break;
        }
        // * Swap the first empty space with the last full space.
        blocks.swap(i, last_full_idx);
    }

    let iter = blocks.into_iter().enumerate().filter_map(|(b, f)| match f {
        Some(f) => Some((b, f)),
        None => None,
    });
    let mut res = 0;
    for (block_idx, file_id) in iter {
        res += block_idx as u64 * file_id;
    }
    return res;
}

pub fn solve_pt2() -> u64 {
    let mut spans = BlockSpan::load();

    let mut prev_id = u64::MAX;
    for file_idx in (0..spans.len()).rev() {
        let file_id = match spans[file_idx].file_id {
            Some(id) if id < prev_id => id,
            _ => continue,
        };
        prev_id = file_id;
        let file_len = spans[file_idx].length;
        for empty_idx in 0..file_idx {
            let empty_len = spans[empty_idx].length;
            if spans[empty_idx].file_id.is_some() || file_len > empty_len {
                continue;
            }
            spans[empty_idx] = replace(&mut spans[file_idx], BlockSpan::new_empty(file_len));
            let spare_idx = empty_idx + 1;
            let spare_len = empty_len - file_len;
            if spans[spare_idx].file_id.is_none() {
                spans[spare_idx].length += spare_len;
            } else {
                spans.insert(spare_idx, BlockSpan::new_empty(spare_len));
            }
            break;
        }
    }
    
    let iter = spans.into_iter().flat_map(BlockSpan::expand).enumerate().filter_map(|(b, f)| match f {
        Some(f) => Some((b, f)),
        None => None,
    });

    let mut res = 0;
    for (block_idx, file_id) in iter {
        res += block_idx as u64 * file_id;
    }
    assert_eq!(res, 6547228115826);
    return res;
}
