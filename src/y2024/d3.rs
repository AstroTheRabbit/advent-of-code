const INPUT: &str = include_str!("../../inputs/y2024/d3.txt");

pub fn solve_pt1() -> u32 {
    let mut res = 0;
    for substr in INPUT.split("mul(") {
        let mut chars = substr.chars();
        let mut num1 = String::new();
        let mut invalid_char = false;
        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                num1.push(c);
            } else {
                invalid_char = c != ',';
                break;
            }
        }
        if invalid_char {
            continue;
        }
        let mut num2 = String::new();
        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                num2.push(c);
            } else {
                invalid_char = c != ')';
                break;
            }
        }
        if invalid_char {
            continue;
        }
        let n1 = num1.parse::<u32>().unwrap();
        let n2 = num2.parse::<u32>().unwrap();
        res += n1 * n2;
    }
    return res;
}

pub fn solve_pt2() -> u32 {
    let mut res = 0;
    let mut mul_enabled = true;
    let mut stack = String::new();

    let mut chars = INPUT.chars().rev().collect::<Vec<_>>();

    while let Some(c) = chars.pop() {
        stack.push(c);

        if "do()".starts_with(&stack) {
            if stack == "do()" {
                mul_enabled = true;
                stack.clear();
            }
        } else if "don't()".starts_with(&stack) {
            if stack == "don't()" {
                mul_enabled = false;
                stack.clear();
            }
        } else if "mul(".starts_with(&stack) {
            if stack == "mul(" {
                let mut mul_stack = String::new();
                let mut found_seperator = false;
                let mut finished = false;
                let mut num_len = 0u32;
    
                while let Some(c) = chars.pop() {
                    mul_stack.push(c);
                    if c.is_ascii_digit() {
                        num_len += 1;
                    } else if c == ',' {
                        if !(!found_seperator && num_len > 0) {
                            // * Found an incorrect second seperator, or the first number of the instruction is missing.
                            break;
                        }
                        num_len = 0;
                        found_seperator = true;
                    } else if c == ')' {
                        if found_seperator && num_len > 0 {
                            // * 'mul' instruction was complete and closed correctly.
                            finished = true;
                        }
                        break;
                    } else {
                        // * `c` is an invalid character.
                        break;
                    }
                }
    
                if finished {
                    // * 'mul' instruction was parsed correctly, adding result based on `mul_enabled`...
                    if mul_enabled {
                        mul_stack.pop(); // * Remove ')' at end of instruction.
                        let (num1, num2) = mul_stack.split_once(',').unwrap();
                        res += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
                    }
                    
                } else {
                    // * 'mul' instruction could not be parsed correctly, returning chars...
                    for c in mul_stack.chars() {
                        chars.push(c);
                    }
                }
                stack.clear();
            }
        } else {
            // * Stack cannot result in a valid instruction, clearing ...
            stack.clear();
        }
    }
    return res;
}