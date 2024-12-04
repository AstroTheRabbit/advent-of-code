const INPUT: &str = include_str!("../../inputs/y2024/d4.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    PX,
    PY,
    NX,
    NY,

    PXPY,
    PXNY,
    NXPY,
    NXNY,
}

impl Dir {
    pub const ALL_DIRS: [Self; 8] = [
        Self::PX,
        Self::PY,
        Self::NX,
        Self::NY,
        Self::PXPY,
        Self::PXNY,
        Self::NXPY,
        Self::NXNY,
    ];

    // * Returns bools matching (px, py, nx, ny).
    #[rustfmt::skip]
    pub fn get_checks(&self) -> (bool, bool, bool, bool) {
        match self {
            //            PX     PY     NX     NY
            Dir::PX   => (true , false, false, false),
            Dir::PY   => (false, true , false, false),
            Dir::NX   => (false, false, true , false),
            Dir::NY   => (false, false, false,  true),
            Dir::PXPY => (true , true , false, false),
            Dir::PXNY => (true , false, false,  true),
            Dir::NXPY => (false, true , true , false),
            Dir::NXNY => (false, false, true ,  true),
        }
    }
}

struct Grid {
    pub chars: Vec<Vec<char>>,
}

impl Grid {
    pub fn load() -> Self {
        Self {
            chars: INPUT.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    // * Get and return the char at (`x`, `y`) [optionally with the ], returning the new coordinate as well.
    pub fn get_dir(
        &self,
        mut x: usize,
        mut y: usize,
        dir: Option<Dir>,
    ) -> Option<(char, usize, usize)> {
        let (px, py, nx, ny) = match dir {
            Some(d) => d.get_checks(),
            None => (false, false, false, false),
        };
        if (nx && x == 0) || (ny && y == 0) {
            // ? Since this code only uses unsigned ints (and also because you can index a Vec below 0), this check is required.
            return None;
        }
        if px {
            x += 1;
        } else if nx {
            x -= 1;
        }
        if py {
            y += 1;
        } else if ny {
            y -= 1;
        }
        if let Some(line) = self.chars.get(y) {
            if let Some(&c) = line.get(x) {
                return Some((c, x, y));
            }
        }
        return None;
    }
}

pub fn solve_pt1() -> u32 {
    let grid = Grid::load();
    let mut res = 0;

    for y0 in 0..grid.chars.len() {
        for x0 in 0..grid.chars[y0].len() {
            if grid.get_dir(x0, y0, None).is_none_or(|(c, ..)| c != 'X') {
                // * character at (x0, y0) isn't an 'X'.
                continue;
            }
            for dir in Dir::ALL_DIRS {
                let mut x = x0;
                let mut y = y0;
                let mut found = true;
                for cc in ['M', 'A', 'S'] {
                    // * Check each subsequent character in direction `dir` for the rest of "XMAS".
                    match grid.get_dir(x, y, Some(dir)) {
                        Some((c, new_x, new_y)) if c == cc => {
                            x = new_x;
                            y = new_y;
                        }
                        _ => {
                            found = false;
                            break;
                        }
                    }
                }
                if found {
                    res += 1;
                }
            }
        }
    }
    return res;
}

pub fn solve_pt2() -> u32 {
    let grid = Grid::load();
    let mut res = 0;

    for y in 0..grid.chars.len() {
        for x in 0..grid.chars[y].len() {
            if grid.get_dir(x, y, None).is_none_or(|(c, ..)| c != 'A') {
                // * character at (x0, y0) cannot be the centre of an "X-MAS".
                continue;
            }
            let c_pxpy = match grid.get_dir(x, y, Some(Dir::PXPY)) {
                Some((c, ..)) => c,
                None => continue,
            };
            let c_pxny = match grid.get_dir(x, y, Some(Dir::PXNY)) {
                Some((c, ..)) => c,
                None => continue,
            };
            let c_nxpy = match grid.get_dir(x, y, Some(Dir::NXPY)) {
                Some((c, ..)) => c,
                None => continue,
            };
            let c_nxny = match grid.get_dir(x, y, Some(Dir::NXNY)) {
                Some((c, ..)) => c,
                None => continue,
            };

            // ? Notice: the Y-axis in this context increases as you read each line (i.e. PY is "down" when printed on the screen).
            // * An "X-MAS" can take the following forms:
            // * NXNY.PXNY => M.S | S.M | M.M | S.S
            // * ....C.... => .A. | .A. | .A. | .A.
            // * NXPY.PXPY => M.S | S.M | S.S | M.M

            match (c_nxny, c_pxny, c_nxpy, c_pxpy) {
                ('M', 'S', 'M', 'S')
                | ('S', 'M', 'S', 'M')
                | ('M', 'M', 'S', 'S')
                | ('S', 'S', 'M', 'M') => res += 1,
                _ => continue,
            }
        }
    }

    return res;
}
