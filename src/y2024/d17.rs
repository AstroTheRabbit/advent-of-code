use crate::include_input;
use std::num::ParseIntError;

const INPUT: &str = include_input!("2024", "17");

#[derive(Debug)]
struct Computer {
    pub program: Vec<u8>,
    pub ip: usize,
    pub reg_a: u64,
    pub reg_b: u64,
    pub reg_c: u64,
}

impl Computer {
    fn read_num(chars: &mut impl Iterator<Item = char>) -> Result<u64, ParseIntError> {
        return chars
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse();
    }

    pub fn load() -> Self {
        let chars = INPUT.chars();
        // * Skip "Register A: ", read `reg_a`.
        let mut chars = chars.skip(12);
        let reg_a = Self::read_num(&mut chars).unwrap();
        // * Skip "Register B: ", read `reg_b`.
        let mut chars = chars.skip(12);
        let reg_b = Self::read_num(&mut chars).unwrap();
        // * Skip "Register C: ", read `reg_c`.
        let mut chars = chars.skip(12);
        let reg_c = Self::read_num(&mut chars).unwrap();

        // * Skip "\nProgram: ", read `program`.
        let mut chars = chars.skip(10);
        let mut program = Vec::new();
        while let Ok(n) = Self::read_num(&mut chars) {
            program.push(n as u8);
        }
        return Self {
            program,
            ip: 0,
            reg_a,
            reg_b,
            reg_c,
        };
    }

    #[inline]
    pub fn read_prgm(&mut self) -> u8 {
        let res = self.program[self.ip];
        self.ip += 1;
        return res;
    }

    pub fn combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid combo operand ({})!", operand),
        }
    }

    pub fn run_inst(&mut self, inst: Instruction, operand: u8) -> Option<u8> {
        match inst {
            // * reg_a / (2 ^ comb(op)) => reg_a
            Instruction::Adv => {
                let comb = self.combo(operand);
                self.reg_a /= 2u64.pow(comb as u32);
                return None;
            }
            // * reg_b XOR op => reg_b
            Instruction::Bxl => {
                self.reg_b ^= operand as u64;
                return None;
            }
            // * comb(op) % 8 => reg_b
            Instruction::Bst => {
                self.reg_b = self.combo(operand) % 8;
                return None;
            }
            // * reg_a != 0 ? op => ip
            Instruction::Jnz => {
                if self.reg_a != 0 {
                    self.ip = operand as usize;
                }
                return None;
            }
            // * reg_b XOR reg_c => reg_b
            Instruction::Bxc => {
                self.reg_b ^= self.reg_c;
                return None;
            }
            // * comb(op) % 8 => out
            Instruction::Out => {
                let comb = self.combo(operand) % 8;
                return Some(comb as u8);
            }
            // * reg_a / (2 ^ comb(op)) => reg_b
            Instruction::Bdv => {
                let comb = self.combo(operand);
                self.reg_b = self.reg_a / 2u64.pow(comb as u32);
                return None;
            }
            // * reg_a / (2 ^ comb(op)) => reg_c
            Instruction::Cdv => {
                let comb = self.combo(operand);
                self.reg_c = self.reg_a / 2u64.pow(comb as u32);
                return None;
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid opcode!"),
        }
    }
}

pub fn solve_pt1() -> String {
    let mut comp = Computer::load();
    let mut out = Vec::new();

    while comp.ip < comp.program.len() - 1 {
        let opcode = comp.read_prgm();
        let operand = comp.read_prgm();
        if let Some(num) = comp.run_inst(opcode.into(), operand) {
            out.push(num.to_string());
        }
    }
    return out.join(",");
}
