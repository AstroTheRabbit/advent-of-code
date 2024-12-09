use crate::include_input;

const INPUT: &str = include_input!("2024", "9");

#[derive(Debug, Clone)]
enum BlockSpan {
    File { len: usize, file_id: u64 },
    Empty,
}

impl BlockSpan {
    pub fn load() -> Vec<Self> {
        let disk_map = INPUT.chars().map(|c| c.to_digit(10).unwrap() as usize);
        let mut spans = Vec::new();

        let mut file_id = 0;
        let mut is_free_space = false;
        for len in disk_map {
            match is_free_space {
                true => {
                    for _ in 0..len {
                        spans.push(Self::Empty);
                    }
                    is_free_space = false;
                },
                false => {
                    spans.push(Self::File { len, file_id });
                    file_id += 1;
                    is_free_space = true;
                },
            }
        }
        return spans;
    }

    pub fn expand_to_blocks(self) -> Vec<Option<u64>> {
        match self {
            BlockSpan::File { len, file_id } => vec![Some(file_id); len],
            BlockSpan::Empty => vec![None],
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }
}

pub fn solve_pt1() -> u64 {
    let mut blocks = BlockSpan::load().into_iter().flat_map(BlockSpan::expand_to_blocks).collect::<Vec<_>>();

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

    // fn dbg_spans(spans: &Vec<BlockSpan>) {
    //     for s in spans {
    //         match s {
    //             BlockSpan::File { len, file_id } => {
    //                 let s = format!("{:X}", file_id);
    //                 for _ in 0..*len {
    //                     print!("{}", s);
    //                 }
    //             },
    //             BlockSpan::Empty => print!("."),
    //         }
    //     }
    //     println!();
    // }

    // dbg_spans(&spans);

    let len = spans.len();
    let mut prev_id = u64::MAX;
    for file_idx in (1..len).rev() {
        let file_len = match spans[file_idx] {
            BlockSpan::File { len, file_id } => {
                let prev = prev_id;
                prev_id = file_id;
                // * Files can only be moved in descending order.
                match prev >= file_id {
                    true => len,
                    false => {
                        println!("Attempted to move {} after {}.", file_id, prev);
                        break;
                    },
                }
            },
            BlockSpan::Empty { .. } => continue,
        };

        if file_idx < file_len {
            // * Current file cannot be moved forward.
            break;
        }

        for window_front in 0..(file_idx - file_len) {
            let window_back = window_front + file_len;
            // * All spans in the 'window' (of length `file_len`) must be `Empty`,
            // * but at least one span between the end of the window and `file_idx` - 1 must be a `File`.
            if spans[window_front..window_back].iter().all(BlockSpan::is_empty)  && spans[(window_back + 1)..file_idx].iter().any(BlockSpan::is_file) {
                // * Swap the moved `File` with the first `Empty` of the window.
                spans.swap(window_front, file_idx);
                // * Move the rest of the window to where the moved `File` was.
                for _ in 1..file_len {
                    let empty = spans.remove(window_front + 1);
                    spans.insert(file_idx, empty);
                }
                // dbg_spans(&spans);
                break;
            }
        }
    }

    let iter = spans.into_iter().flat_map(BlockSpan::expand_to_blocks).enumerate().filter_map(|(b, f)| match f {
        Some(f) => Some((b, f)),
        None => None,
    });

    let mut res = 0;
    for (block_idx, file_id) in iter {
        res += block_idx as u64 * file_id;
    }
    return res;
}
