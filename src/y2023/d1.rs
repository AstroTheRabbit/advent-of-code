use crate::include_input;

const INPUT: &str = include_input!("2023", "1");

enum LineSearch {
    None,
    OneChar(char),
    TwoChars(char, char),
}

impl LineSearch {
    pub fn found_char(&mut self, c: char) {
        *self = match *self {
            Self::None => Self::OneChar(c),
            Self::OneChar(first_c) => Self::TwoChars(first_c, c),
            Self::TwoChars(first_c, _) => Self::TwoChars(first_c, c),
        };
    }

    pub fn to_num(&self) -> u32 {
        match *self {
            Self::None => panic!("Search found no numbers!"),
            Self::OneChar(c) => String::from_iter([c; 2]).parse().unwrap(),
            Self::TwoChars(fc, lc) => String::from_iter([fc, lc]).parse().unwrap(),
        }
    }
}

pub fn solve_pt1() -> u32 {
    let mut res = 0;
    for line in INPUT.lines() {
        let mut search = LineSearch::None;
        // * Iterate through characters on each line.
        for c in line.chars() {
            if c.is_ascii_digit() {
                search.found_char(c);
            }
        }
        res += search.to_num();
    }
    return res;
}

pub fn solve_pt2() -> u32 {
    const SPELLED_LOOKUP: [(&str, char); 9] = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut res = 0;
    for line in INPUT.lines() {
        let mut search = LineSearch::None;
        let mut spelled = String::new();

        for c in line.chars() {
            if c.is_ascii_digit() {
                // * Single character check.
                search.found_char(c);
                spelled.clear();
            } else {
                // * Spelled number check.
                spelled.push(c);
                for (s, sc) in SPELLED_LOOKUP {
                    if spelled.ends_with(s) {
                        search.found_char(sc);
                        // ? Some spelled numbers overlap by one character.
                        // ? For this puzzle, these occurances are treated as two seperate digits.
                        // ? e.g. 'oneight' becomes '1' and '8'.
                        // ? Therefore, we need to keep the last character of `spelled` in case it overlaps.
                        spelled = spelled.pop().unwrap().to_string();
                    }
                }
            }
        }
        res += search.to_num();
    }
    return res;
}
