const INPUT: &str = include_str!("../../inputs/y2024/d2.txt");

fn load_reports() -> Vec<Vec<u32>> {
    let mut res = Vec::new();
    for line in INPUT.lines() {
        let mut report = Vec::new();
        let mut buffer = String::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                buffer.push(c);
            } else if buffer.len() > 0 {
                let num = buffer.parse::<u32>().unwrap();
                report.push(num);
                buffer.clear();
            }
        }
        if buffer.len() > 0 {
            let num = buffer.parse::<u32>().unwrap();
            report.push(num);
        }
        res.push(report);
    }
    return res;
}

pub fn solve_pt1() -> u32 {
    let reports = load_reports();
    let mut res = 0;

    for report in reports {
        if report.len() < 2 {
            // ? This never occurs in any provided inputs, but it's still good to check!
            continue;
        }

        let is_increasing = report[1] > report[0];
        let mut passed = true;
        for pair in report.windows(2) {
            let diff = if is_increasing {
                pair[1].saturating_sub(pair[0])
            } else {
                pair[0].saturating_sub(pair[1])
            };
            if diff < 1 || diff > 3 {
                passed = false;
                break;
            }
        }
        if passed {
            res += 1;
        }
    }
    return res;
}

pub fn solve_pt2() -> u32 {
    let reports = load_reports();
    let mut res = 0;

    for report in reports {
        if report.len() < 2 {
            // ? This never occurs in any provided inputs, but it's still good to check!
            continue;
        }

        let mut passed = true;
        let mut is_increasing = report[1] > report[0];
        for pair in report.windows(2) {
            let diff = if is_increasing {
                pair[1].saturating_sub(pair[0])
            } else {
                pair[0].saturating_sub(pair[1])
            };
            if diff < 1 || diff > 3 {
                passed = false;
                break;
            }
        }

        if !passed {
            // * Failed the regular check, now checking if removing a single level can pass.
            for idx in 0..report.len() {
                let mut report = report.clone();
                report.remove(idx);

                passed = true;
                is_increasing = report[1] > report[0];
                for pair in report.windows(2) {
                    let diff = if is_increasing {
                        pair[1].saturating_sub(pair[0])
                    } else {
                        pair[0].saturating_sub(pair[1])
                    };
                    if diff < 1 || diff > 3 {
                        passed = false;
                        break;
                    }
                }
                if passed {
                    break;
                }
            }
        }

        if passed {
            res += 1;
        }
    }
    return res;
}
