use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../inputs/y2023/d3.txt");

struct LineNumber {
    pub num: u32,
    pub line_idx: usize,
    pub range: LineRange,
}

enum LineRange {
    None,
    Start(usize),
    End(usize, usize),
}

impl LineRange {
    pub fn update(&mut self, new_idx: usize) {
        *self = match *self {
            Self::None => Self::Start(new_idx.saturating_sub(1)),
            Self::Start(start) => Self::End(start, new_idx + 1),
            Self::End(start, _) => Self::End(start, new_idx + 1),
        }
    }

    fn into_neighbours_range(&self) -> RangeInclusive<usize> {
        match &self {
            LineRange::None => panic!("Line range is empty!"),
            LineRange::Start(idx) => *idx..=(idx + 2),
            LineRange::End(start, end) => *start..=*end,
        }
    }

    // * Check if this `LineRange` contains a value that is also contained in `indices`.
    pub fn check_range(&self, indices: &Vec<usize>) -> bool {
        for idx in self.into_neighbours_range() {
            if indices.contains(&idx) {
                return true;
            }
        }
        return false;
    }
}

pub fn solve_pt1() -> u32 {
    // * `numbers` consists of each number and the indices of its characters in the schematic.
    let mut numbers = Vec::new();
    // * `symbols` is used to determine if a specific position on the schematic is a 'symbol'.
    let mut symbols = Vec::new();
    let mut max_line_idx = 0;

    for (line_idx, line) in INPUT.lines().enumerate() {
        max_line_idx = line_idx;

        let mut line_symbols = Vec::new();
        let mut num_range = LineRange::None;
        let mut num_chars = String::new();

        for (char_idx, c) in line.char_indices() {
            let mut num_finished = false;
            if c.is_ascii_digit() {
                num_chars.push(c);
                num_range.update(char_idx);
            } else if c == '.' {
                num_finished = true;
            } else {
                line_symbols.push(char_idx);
                num_finished = true;
            }

            if num_finished && num_chars.len() > 0 {
                let num = num_chars.parse::<u32>().unwrap();
                numbers.push(LineNumber { num, line_idx, range: num_range });
                num_range = LineRange::None;
                num_chars.clear();
            }
        }

        if num_chars.len() > 0 {
            let num = num_chars.parse::<u32>().unwrap();
            numbers.push(LineNumber { num, line_idx, range: num_range });
        }
        symbols.push(line_symbols);
    }

    let mut res = 0;
    for n in numbers {
        let mut neighbours_symbol = false;
        if n.line_idx > 0 && n.range.check_range(&symbols[n.line_idx - 1]) {
            // * Check the line 'above' this one for symbols next to the number.
            neighbours_symbol = true;
        } else if n.range.check_range(&symbols[n.line_idx]) {
            // * Check this line for symbols next to the number.
            neighbours_symbol = true;
        } else if n.line_idx < max_line_idx && n.range.check_range(&symbols[n.line_idx + 1]) {
            // * Check the line 'below' this one for symbols next to the number.
            neighbours_symbol = true;
        }
        if neighbours_symbol {
            res += n.num;
        }
    }
    return res;
}