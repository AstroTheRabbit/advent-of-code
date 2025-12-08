use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::include_input;

const INPUT: &str = include_input!("2025", "8");

#[derive(Debug, Clone, Copy)]
struct Pos {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl Pos {
    pub const fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    pub fn dist_squared(a: Self, b: Self) -> u64 {
        // * Square of the Euclidean distance. Used for sorting a list of positions.
        let d = Pos::new(
            u64::abs_diff(a.x, b.x),
            u64::abs_diff(a.y, b.y), 
            u64::abs_diff(a.z, b.z),
        );
        return (d.x * d.x) + (d.y * d.y) + (d.z * d.z);
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

struct Circuits {
    map: HashMap<usize, usize>,
    sizes: HashMap<usize, usize>,
    connections: HashSet<(usize, usize)>,
    next_circuit_id: usize,
}

impl Circuits {
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            sizes: HashMap::with_capacity(capacity),
            connections: HashSet::with_capacity(capacity),
            next_circuit_id: 0,
        }
    }

    pub fn are_connected(&self, ja: usize, jb: usize) -> bool {
        return self.connections.contains(&(ja, jb));
    }

    pub fn are_connected_indirect(&self, ja: usize, jb: usize) -> bool {
        let ma = self.map.get(&ja);
        let mb = self.map.get(&jb);
        if let (Some(ma), Some(mb)) = (ma, mb) {
            return ma == mb;
        } else {
            return false;
        }
    }

    pub fn get_sizes_sorted(&self, n: usize) -> impl Iterator<Item = usize> + use<'_> {
        let mut res = self
            .sizes
            .values()
            .collect::<Vec<_>>();
        res.sort_unstable();
        return res
            .into_iter()
            .rev()
            .take(n)
            .copied();
    }

    pub fn connect(&mut self, ja: usize, jb: usize) {
        self.connections.insert((ja, jb));
        let ma = self.map.get(&ja).copied();
        let mb = self.map.get(&jb).copied();

        match (ma, mb) {
            (None, None) => self.new_circuit(ja, jb),
            (Some(ca), None) => self.add_to_circuit(ca, jb),
            (None, Some(cb)) => self.add_to_circuit(cb, ja),
            (Some(ca), Some(cb)) => self.merge_circuits(ca, cb),
        }
    }

    fn new_circuit(&mut self, ja: usize, jb: usize) {
        let id = self.next_circuit_id;
        self.next_circuit_id += 1;
        self.map.insert(ja, id);
        self.map.insert(jb, id);
        self.sizes.insert(id, 2);
    }

    fn add_to_circuit(&mut self, c: usize, j: usize) {
        self.map.insert(j, c);
        if let Some(s) = self.sizes.get_mut(&c) {
            *s += 1;
        } else {
            panic!("Missing circuit {} from `sizes`!", c);
        }
    }

    fn merge_circuits(&mut self, ca: usize, cb: usize) {
        if ca == cb {
            // * `ca` and `cb` are the same; no need to merge!
            return;
        }

        // * Merge `cb` into `ca`.
        for c in self.map.values_mut() {
            if *c == cb {
                *c = ca;
            }
        }
        let count = self.sizes.remove(&cb).unwrap();
        if let Some(s) = self.sizes.get_mut(&ca) {
            *s += count;
        } else {
            panic!("Missing circuit {} from `sizes`!", cb);
        }

    }
}

fn load_positions() -> Vec<Pos> {
    let mut positions = Vec::new();
    for l in INPUT.lines() {
        let s = l.split(',').collect::<Vec<_>>();
        let x = s[0].parse().unwrap();
        let y = s[1].parse().unwrap();
        let z = s[2].parse().unwrap();
        positions.push(Pos::new(x, y, z));
    }
    return positions;
}

pub fn solve_pt1() -> u64 {
    // * How many connections should we make?
    const CONNECTION_COUNT: usize = 1000; // 10 for the puzzle's example.
    // * How many of the largest circuits should we multiply together to get the solution?
    const LARGEST_COUNT: usize = 3;

    let positions = load_positions();
    let len = positions.len();

    // TODO: Pre-sort the positions by distance, then start connecting.
    
    let mut circuits = Circuits::new(CONNECTION_COUNT);

    for _ in 0..CONNECTION_COUNT {
        let mut closest_pair = None;
        let mut closest_dist = u64::MAX;

        for ja in 0..len {
            for jb in (ja + 1)..len {
                if circuits.are_connected(ja, jb) {
                    continue;
                }

                let pa = positions[ja];
                let pb = positions[jb];
                let dist = Pos::dist_squared(pa, pb);

                if dist < closest_dist {
                    closest_dist = dist;
                    closest_pair = Some((ja, jb));
                }
            }
        }

        if let Some((ja, jb)) = closest_pair {
            circuits.connect(ja, jb);
        } else {
            panic!("Failed to find the closest pair!");
        }
    }

    return circuits
        .get_sizes_sorted(LARGEST_COUNT)
        .product::<usize>()
        .try_into()
        .unwrap();

}

pub fn solve_pt2() -> u64 {
    let positions = load_positions();
    let len = positions.len();

    // TODO: Pre-sort the positions by distance, then start connecting.

    let mut circuits = Circuits::new(len);
    let mut last_connection = None;

    loop {
        let mut closest_pair = None;
        let mut closest_dist = u64::MAX;

        for ja in 0..len {
            for jb in (ja + 1)..len {
                if circuits.are_connected_indirect(ja, jb) {
                    continue;
                }

                let pa = positions[ja];
                let pb = positions[jb];
                let dist = Pos::dist_squared(pa, pb);

                if dist < closest_dist {
                    closest_dist = dist;
                    closest_pair = Some((ja, jb));
                }
            }
        }

        if let Some((ja, jb)) = closest_pair {
            circuits.connect(ja, jb);
            last_connection = closest_pair;
        } else {
            // * All the circuits have been connected!
            break;
        }
    }

    if let Some((ja, jb)) = last_connection {
        let pa = positions[ja];
        let pb = positions[jb];
        return pa.x * pb.x;
    } else {
        panic!("No connections were made!");
    }
}