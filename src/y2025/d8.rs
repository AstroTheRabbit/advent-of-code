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

    pub fn same_circuit(&self, ja: usize, jb: usize) -> bool {
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
    // ? 10 for the example input, 1000 for the actual input.
    const CONNECTION_COUNT: usize = 1000;
    // * How many of the largest circuits should we multiply together to get the solution?
    const LARGEST_COUNT: usize = 3;

    let positions = load_positions();
    let len = positions.len();
    
    let mut pairs = Vec::with_capacity(len);
    let mut circuits = Circuits::new(CONNECTION_COUNT);
    
    for ja in 0..len {
        for jb in (ja + 1)..len {
            pairs.push((ja, jb));
        }
    }

    // * Sort the pairs from least distance to most distance.
    pairs.sort_by_cached_key(|(a, b)| Pos::dist_squared(positions[*a], positions[*b]));

    for (ja, jb) in pairs.into_iter().take(CONNECTION_COUNT) {
        circuits.connect(ja, jb);
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
    
    let mut pairs = Vec::with_capacity(len);
    let mut circuits = Circuits::new(len);
    
    for ja in 0..len {
        for jb in (ja + 1)..len {
            pairs.push((ja, jb));
        }
    }

    // * Sort the pairs from least distance to most distance.
    pairs.sort_by_cached_key(|(a, b)| Pos::dist_squared(positions[*a], positions[*b]));

    let mut last_connection = None;

    for (ja, jb) in pairs {
        if !circuits.same_circuit(ja, jb) {
            circuits.connect(ja, jb);
            last_connection = Some((ja, jb));
        }
    }

    if let Some((ja, jb)) = last_connection {
        let pa = positions[ja];
        let pb = positions[jb];
        return pa.x * pb.x;
    } else {
        unreachable!("No connections were made!");
    }
}