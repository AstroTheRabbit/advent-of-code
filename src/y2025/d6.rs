use crate::include_input;

const INPUT: &str = include_input!("2025", "6");

enum Operation {
    Sum,
    Product,
}

impl Operation {
    pub fn from_char(c: char) -> Self {
        match c {
            '+' => Self::Sum,
            '*' => Self::Product,
            _ => panic!("Invalid character '{}' in input!", c),
        }
    }

    pub fn apply(&self, vals: &[u64]) -> u64 {
        match self {
            Self::Sum => vals.iter().sum(),
            Self::Product => vals.iter().product(),
        }
    }
}

fn load_worksheet() -> (Vec<Vec<char>>, Vec<char>) {
    let lines = INPUT.lines().collect::<Vec<_>>();
    let (ops, vals) = lines.split_last().unwrap();

    let ops = ops
        .chars()
        .collect::<Vec<_>>();

    let vals = vals
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    return (vals, ops);
}

pub fn solve_pt1() -> u64 {
    let (vals, ops) = load_worksheet();
    let num_rows = vals.len();

    let vals = vals
        .into_iter()
        .map(|row| String::from_iter(row))
        .flat_map(|row| row
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let ops = ops
        .into_iter()
        .filter(|c| !c.is_whitespace())
        .map(|c| Operation::from_char(c))
        .collect::<Vec<_>>();

    let num_columns = vals.len() / num_rows;
    assert!(vals.len() == num_rows * num_columns, "Input has an incorrect amount of values!");

    let mut sum = 0;
    let mut problem_vals = Vec::new();
    for col in 0..num_columns {
        for row in 0..num_rows {
            let idx = (row * num_columns) + col;
            problem_vals.push(vals[idx]);
        }
        sum += ops[col].apply(&problem_vals);
        problem_vals.clear();
    }
    return sum;
}

pub fn solve_pt2() -> u64 {
    let (vals, ops) = load_worksheet();
	let num_rows = vals.len();
    let num_columns = vals.iter().map(|s| s.len()).max().unwrap();
    
    let mut problem_vals = Vec::new();
    let mut buffer = String::new();
    let mut sum = 0;

    for col in (0..num_columns).rev() {
        for row in 0..num_rows {
            let c = vals[row][col];
            if c != ' ' {
                buffer.push(c);
            }
        }
        if buffer.is_empty() {
            // * Empty column between problems.
            continue;
        }
        problem_vals.push(buffer.parse().unwrap());
        buffer.clear();
        
        let c = ops[col];
        if c != ' ' {
            // * End of the problem (reading right-to-left).
            let op = Operation::from_char(c);
            sum += op.apply(&problem_vals);
            problem_vals.clear();
        }
    }
    return sum;
}