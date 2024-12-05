use crate::include_input;
use std::ops::Mul;

const INPUT: &str = include_input!("2024", "4");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dir {
    pub x: isize,
    pub y: isize,
}

impl Dir {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const NONE: Self = Self::new(0, 0);

    pub const PX: Self = Self::new(1, 0);
    pub const PY: Self = Self::new(0, 1);
    pub const NX: Self = Self::new(-1, 0);
    pub const NY: Self = Self::new(0, -1);

    pub const PXPY: Self = Self::new(1, 1);
    pub const PXNY: Self = Self::new(1, -1);
    pub const NXPY: Self = Self::new(-1, 1);
    pub const NXNY: Self = Self::new(-1, -1);

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
}

impl Mul<Dir> for isize {
    type Output = Dir;

    fn mul(self, rhs: Dir) -> Self::Output {
        return Dir::new(self * rhs.x, self * rhs.y);
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

    pub fn get_dir(&self, x: usize, y: usize, dir: Dir) -> Option<char> {
        if let Some((x, y)) = Option::zip(x.checked_add_signed(dir.x), y.checked_add_signed(dir.y))
        {
            if let Some(line) = self.chars.get(y) {
                if let Some(&c) = line.get(x) {
                    return Some(c);
                }
            }
        }
        return None;
    }
}

pub fn solve_pt1() -> u32 {
    let grid = Grid::load();
    let mut res = 0;

    for y in 0..grid.chars.len() {
        for x in 0..grid.chars[y].len() {
            if grid.get_dir(x, y, Dir::NONE).is_none_or(|c| c != 'X') {
                // * character at (x, y) isn't an 'X'.
                continue;
            }
            for dir in Dir::ALL_DIRS {
                if grid.get_dir(x, y, 1 * dir).is_none_or(|c| c != 'M') {
                    continue;
                }
                if grid.get_dir(x, y, 2 * dir).is_none_or(|c| c != 'A') {
                    continue;
                }
                if grid.get_dir(x, y, 3 * dir).is_none_or(|c| c != 'S') {
                    continue;
                }
                res += 1;
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
            if grid.get_dir(x, y, Dir::NONE).is_none_or(|c| c != 'A') {
                // * character at (x0, y0) cannot be the centre of an "X-MAS" pattern.
                continue;
            }
            let c_pxpy = match grid.get_dir(x, y, Dir::PXPY) {
                Some(c) => c,
                None => continue,
            };
            let c_pxny = match grid.get_dir(x, y, Dir::PXNY) {
                Some(c) => c,
                None => continue,
            };
            let c_nxpy = match grid.get_dir(x, y, Dir::NXPY) {
                Some(c) => c,
                None => continue,
            };
            let c_nxny = match grid.get_dir(x, y, Dir::NXNY) {
                Some(c) => c,
                None => continue,
            };

            // ? Notice: the Y-axis in this context increases as you read each line (i.e. PY is "down" when printed on the screen).
            // * An "X-MAS" can take the following forms:
            // * NXNY.PXNY => M.S | S.M | M.M | S.S
            // * ....C.... => .A. | .A. | .A. | .A.
            // * NXPY.PXPY => M.S | S.M | S.S | M.M

            let xmas_pattern = [c_nxny, c_pxny, c_nxpy, c_pxpy];
            if [
                ['M', 'S', 'M', 'S'],
                ['S', 'M', 'S', 'M'],
                ['M', 'M', 'S', 'S'],
                ['S', 'S', 'M', 'M'],
            ]
            .contains(&xmas_pattern)
            {
                res += 1;
            }
        }
    }
    return res;
}
