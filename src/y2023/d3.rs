use std::{collections::HashMap, ops::RangeInclusive};
use crate::include_input;

const INPUT: &str = include_input!("2023", "3");

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

    // * Check if this `LineRange` contains a value that is also contained in `indices`, returning the index if shared.
    pub fn check_range(&self, indices: &Vec<usize>) -> Option<usize> {
        for idx in self.into_neighbours_range() {
            if indices.contains(&idx) {
                return Some(idx);
            }
        }
        return None;
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
                numbers.push(LineNumber {
                    num,
                    line_idx,
                    range: num_range,
                });
                num_range = LineRange::None;
                num_chars.clear();
            }
        }

        if num_chars.len() > 0 {
            let num = num_chars.parse::<u32>().unwrap();
            numbers.push(LineNumber {
                num,
                line_idx,
                range: num_range,
            });
        }
        symbols.push(line_symbols);
    }

    let mut res = 0;
    for n in numbers {
        let mut neighbours_symbol = false;
        if n.range.check_range(&symbols[n.line_idx]).is_some() {
            // * Check this line for symbols next to the number.
            neighbours_symbol = true;
        } else if n.line_idx > 0 && n.range.check_range(&symbols[n.line_idx - 1]).is_some() {
            // * Check the line 'above' this one for symbols next to the number.
            neighbours_symbol = true;
        } else if n.line_idx < max_line_idx
            && n.range.check_range(&symbols[n.line_idx + 1]).is_some()
        {
            // * Check the line 'below' this one for symbols next to the number.
            neighbours_symbol = true;
        }
        if neighbours_symbol {
            res += n.num;
        }
    }
    return res;
}

pub fn solve_pt2() -> u32 {
    let mut numbers = Vec::new();
    let mut gears = Vec::new();
    let mut max_line_idx = 0;

    for (line_idx, line) in INPUT.lines().enumerate() {
        max_line_idx = line_idx;

        let mut line_gears = Vec::new();
        let mut num_range = LineRange::None;
        let mut num_chars = String::new();

        for (char_idx, c) in line.char_indices() {
            let mut num_finished = false;
            if c.is_ascii_digit() {
                num_chars.push(c);
                num_range.update(char_idx);
            } else if c == '*' {
                line_gears.push(char_idx);
                num_finished = true;
            } else {
                num_finished = true;
            }

            if num_finished && num_chars.len() > 0 {
                let num = num_chars.parse::<u32>().unwrap();
                numbers.push(LineNumber {
                    num,
                    line_idx,
                    range: num_range,
                });
                num_range = LineRange::None;
                num_chars.clear();
            }
        }
        if num_chars.len() > 0 {
            let num = num_chars.parse::<u32>().unwrap();
            numbers.push(LineNumber {
                num,
                line_idx,
                range: num_range,
            });
        }
        gears.push(line_gears);
    }

    let mut res = 0;
    let mut gear_ratios = HashMap::new();

    for n in numbers {
        // ? I'm fairly certain that no 'conjoined gears' exist (e.g. "123*456*789"), but I may as well support it!
        let mut neighbouring_gears = Vec::new();
        if let Some(char_idx) = n.range.check_range(&gears[n.line_idx]) {
            // * Check this line for symbols next to the number.
            neighbouring_gears.push((n.line_idx, char_idx));
        }
        if n.line_idx > 0 {
            // * Check the line 'above' this one for s next to the number.
            if let Some(char_idx) = n.range.check_range(&gears[n.line_idx - 1]) {
                neighbouring_gears.push((n.line_idx - 1, char_idx));
            }
        }
        if n.line_idx < max_line_idx {
            // * Check the line 'below' this one for symbols next to the number.
            if let Some(char_idx) = n.range.check_range(&gears[n.line_idx + 1]) {
                neighbouring_gears.push((n.line_idx + 1, char_idx));
            }
        }
        for pos in neighbouring_gears {
            if let Some(gear_ratio) = gear_ratios.get(&pos) {
                res += gear_ratio * n.num;
            } else {
                gear_ratios.insert(pos, n.num);
            }
        }
    }
    return res;
}
