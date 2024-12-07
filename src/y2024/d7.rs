use crate::include_input;

const INPUT: &str = include_input!("2024", "7");

#[derive(Debug)]
struct Equation {
    pub result: u64,
    pub operands: Vec<u64>,
}

impl Equation {
    pub fn load() -> Vec<Self> {
        let mut equations = Vec::new();
        for line in INPUT.lines() {
            let mut chars = line.chars();
            // * Read the equation's result.
            let result = chars
                .by_ref()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap();
            // * Skip the space after the colon seperator.
            let mut chars = chars.skip(1);
            // * Read the operands of the equation.
            let operands = chars
                .by_ref()
                .collect::<String>()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();
            equations.push(Equation { result, operands });
        }
        return equations;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Mult,
    Concat,
}

impl Operation {
    pub fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Mult => a * b,
            Operation::Concat => format!("{}{}", a, b).parse().unwrap(),
        }
    }

    pub fn apply_all(operations: &Vec<Operation>, operands: &Vec<u64>) -> u64 {
        let mut res = operands[0];
        for i in 0..operations.len() {
            res = operations[i].apply(res, operands[i + 1]);
        }
        return res;
    }
}

pub fn solve_pt1() -> u64 {
    let equations = Equation::load();
    let mut res = 0;
    let mut ops = Vec::new();
    for eq in equations {
        let ops_len = eq.operands.len() - 1;
        ops.clear();
        ops.resize(ops_len, Operation::Add);

        loop {
            let current = Operation::apply_all(&ops, &eq.operands);
            if current == eq.result {
                // * Current operations resulted in a correct value!
                res += eq.result;
                break;
            }
            // * Current operations are incorrect...
            // * Getting the next operation combination works similarly to binary counting - find the first `Add`, change it to a `Mul` and everything before it back to an `Add`.
            let mut tested_all = true;
            for i in 0..ops.len() {
                match ops[i] {
                    Operation::Add => {
                        ops[i] = Operation::Mult;
                        tested_all = false;
                        break;
                    }
                    Operation::Mult => {
                        ops[i] = Operation::Add;
                    }
                    Operation::Concat => panic!("`Concat` operation not used in part 1!"),
                }
            }
            if tested_all {
                break;
            }
        }
    }
    return res;
}

pub fn solve_pt2() -> u64 {
    let equations = Equation::load();
    let mut res = 0;
    let mut ops = Vec::new();
    for eq in equations {
        let ops_len = eq.operands.len() - 1;
        ops.clear();
        ops.resize(ops_len, Operation::Add);

        loop {
            let current = Operation::apply_all(&ops, &eq.operands);
            if current == eq.result {
                // * Current operations resulted in a correct value!
                res += eq.result;
                break;
            }
            // * Current operations are incorrect...
            let mut tested_all = true;
            for i in 0..ops.len() {
                match ops[i] {
                    Operation::Add => {
                        ops[i] = Operation::Mult;
                        tested_all = false;
                        break;
                    }
                    Operation::Mult => {
                        ops[i] = Operation::Concat;
                        tested_all = false;
                        break;
                    }
                    Operation::Concat => {
                        ops[i] = Operation::Add;
                        continue;
                    }
                }
            }
            if tested_all {
                break;
            }
        }
    }
    return res;
}

// 000
// 001 +0
// 002 +0
// 010 +1~0
// 011 +0
// 012 +0
// 020 +1~0
// 021 +0
// 022 +0
// 100 +2~0,1
// 101 +0
// 102 +0
// 110 +1~0
// 111 +
// 112 +
// 120 +
// 121 +
// 122 +
// 200 +
// ...

// ! 1289579105366
