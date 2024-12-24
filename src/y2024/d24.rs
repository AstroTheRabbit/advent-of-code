use std::{array, collections::{HashMap, VecDeque}};

use crate::include_input;

const INPUT: &str = include_input!("2024", "24");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    AND,
    OR,
    XOR,
}

impl Operator {
    pub fn apply(&self, in1: bool, in2: bool) -> bool {
        match self {
            Operator::AND => in1 & in2,
            Operator::OR => in1 | in2,
            Operator::XOR => in1 ^ in2,
        }
    }
}

#[derive(Debug)]
struct Gate {
    pub in1: Wire,
    pub in2: Wire,
    pub op: Operator,
    pub out: Wire,
}

impl Gate {
    pub const fn new(in1: [char; 3], in2: [char; 3], op: Operator, out: [char; 3]) -> Self {
        Self { in1: Wire { label: in1 }, in2: Wire { label: in2 }, op, out: Wire { label: out } }
    }
}

impl PartialEq for Gate {
    fn eq(&self, other: &Self) -> bool {
        ((self.in1 == other.in1 && self.in2 == other.in2) || (self.in1 == other.in2 && self.in2 == other.in1)) && self.op == other.op && self.out == other.out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Wire {
    pub label: [char; 3]
}

fn load_inputs() -> (HashMap<Wire, bool>, Vec<Gate>) {
    let (input_wires, input_gates) = INPUT.split_once("\n\n").unwrap();
    let mut wires = HashMap::new();
    let mut gates = Vec::new();

    for line in input_wires.lines() {
        let mut chars = line.chars();
        // * Read the wire's label.
        let label = array::from_fn(|_| chars.next().unwrap());
        // * Skip ": ".
        let mut chars = chars.skip(2);
        // * Read the wire's value.
        let value = match chars.next().unwrap() {
            '0' => false,
            '1' => true,
            _ => panic!("Invalid wire input!"),
        };
        wires.insert(Wire { label }, value);
    }

    for line in input_gates.lines() {
        let mut chars = line.chars();
        // * Read the label `in1`.
        let in1 = array::from_fn(|_| chars.next().unwrap());
        // * Skip " ".
        let mut chars = chars.skip(1);
        // * Read the operator `op`.
        let op = match chars.next().unwrap() {
            'A' => Operator::AND,
            'O' => Operator::OR,
            'X' => Operator::XOR,
            _ => panic!("Invalid gate input!"),
        };

        // * Skip the rest of the operator + " ".
        let mut chars = if op == Operator::OR {
            chars.skip(2)
        } else {
            chars.skip(3)
        };
        // * Read the label `in2`.
        let in2 = array::from_fn(|_| chars.next().unwrap());
        // * Skip " -> ".
        let mut chars = chars.skip(4);
        // * Read the label `out`.
        let out = array::from_fn(|_| chars.next().unwrap());
        gates.push(Gate::new(in1, in2, op, out));
    }

    return (wires, gates);
}

pub fn solve_pt1() -> u64 {
    let (mut wires, gates) = load_inputs();
    let mut queue = VecDeque::from(gates);

    while let Some(gate) = queue.pop_front() {
        if wires.contains_key(&gate.out) {
            // * `out` has already been set.
            continue;
        }
        if let Some((&in1, &in2)) = Option::zip(wires.get(&gate.in1), wires.get(&gate.in2)) {
            let res = gate.op.apply(in1, in2);
            wires.insert(gate.out, res);
        } else {
            // * The inputs for this gate haven't been set yet.
            queue.push_back(gate);
            continue;
        }
    }

    let mut bits = wires.into_iter().filter(|(w, _)| w.label[0] == 'z').collect::<Vec<_>>();
    bits.sort_by_cached_key(|(w, _)| w.label);

    let mut res = 0;
    for (i, (_, b)) in bits.into_iter().enumerate() {
        res |= (b as u64) << (i as u64);
    }
    return res;
}

pub fn solve_pt2() -> String {
    // ! I solved this puzzle manually, but may attempt to write some proper code later on: https://www.reddit.com/r/adventofcode/comments/1hl698z/comment/m3kg20o
    // * TLDR: The gates form a ripple carry adder, so you can go through the gates and sort them into components of the adder based on their inputs & operator.
    // * You can then deductively tag the the wires based on their usage in the gates (e.g. "hta" is tagged as "A03 (hta)", some form of macro can be useful for mass-tagging).
    // * Finally, you can use those tags to determine which output wires have been swapped.
    // ? https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder
    unimplemented!();
}