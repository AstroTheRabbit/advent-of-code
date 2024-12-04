const INPUT: &str = include_str!("../../inputs/y2023/d5.txt");

#[derive(Debug)]
struct Category {
    pub _name: String,
    pub maps: Vec<Map>,
}

impl Category {
    pub fn new(name: String) -> Self {
        Self {
            _name: name,
            maps: Vec::new(),
        }
    }

    pub fn map_value(&self, input: u64) -> u64 {
        for map in &self.maps {
            if let Some(res) = map.try_map_value(input) {
                return res;
            }
        }
        return input;
    }
}

#[derive(Debug)]
struct Map {
    pub destination_start: u64,
    pub source_start: u64,
    pub range: u64,
}

impl Map {
    pub fn try_map_value(&self, input: u64) -> Option<u64> {
        if input >= self.source_start && input < self.source_start + self.range {
            let diff = input - self.source_start;
            return Some(self.destination_start + diff);
        } else {
            return None;
        }
    }
}

fn load_inputs() -> (Vec<u64>, Vec<Category>) {
    // * Initial seed inputs.
    let mut res_inputs = Vec::new();
    // * Maps between each category.
    let mut res_maps = Vec::new();

    let mut lines = INPUT.lines();
    let mut inputs_chars = lines.next().unwrap().chars().skip(7);
    loop {
        let input_str = inputs_chars
            .by_ref()
            .take_while(char::is_ascii_digit)
            .collect::<String>();
        if input_str.len() == 0 {
            break;
        }
        res_inputs.push(input_str.parse().unwrap());
    }

    // * Skip empty line between inputs and maps.
    let mut lines = lines.skip(1);
    while let Some(name_line) = lines.next() {
        // * e.g. "seed-to-soil map:" becomes "seed-to-soil".
        let name = name_line.split_whitespace().next().unwrap().to_string();
        let mut category = Category::new(name);
        while let Some(map_line) = lines.next() {
            let nums = map_line.split_whitespace().collect::<Vec<_>>();
            if nums.len() < 3 {
                // * Reached the end of this category's maps.
                break;
            }

            // * Parse each number on this line.
            let destination_start = nums[0].parse().unwrap();
            let source_start = nums[1].parse().unwrap();
            let range = nums[2].parse().unwrap();
            category.maps.push(Map {
                destination_start,
                source_start,
                range,
            });
        }
        res_maps.push(category);
    }

    return (res_inputs, res_maps);
}

pub fn solve_pt1() -> u64 {
    let (inputs, categories) = load_inputs();

    let mut res = u64::MAX;
    for mut input in inputs {
        for cat in &categories {
            input = cat.map_value(input);
        }
        if input < res {
            res = input;
        }
    }

    return res;
}

#[derive(Debug)]
struct Interval {
    pub start: u64,
    pub end: u64,
    pub category_idx: usize,
}

impl Interval {
    pub fn new(start: u64, end: u64, idx: usize) -> Self {
        Self {
            start,
            end,
            category_idx: idx,
        }
    }
}

pub fn solve_pt2() -> u64 {
    // ? Thanks to https://www.youtube.com/watch?v=EGQgUYx-2gE for explaining the non-bruteforce 'interval' method.
    let (inputs, categories) = load_inputs();
    let mut res = u64::MAX;

    let mut stack = Vec::new();
    for range in inputs.chunks_exact(2) {
        stack.push(Interval {
            start: range[0],
            end: range[0] + range[1],
            category_idx: 0,
        });
    }

    while let Some(mut interval) = stack.pop() {
        if interval.category_idx >= categories.len() {
            // * This interval has passed through every category.
            if interval.start < res {
                res = interval.start;
            }
            continue;
        }

        let mut found_map = false;
        for map in &categories[interval.category_idx].maps {
            let map_start = map.source_start;
            let map_end = map.source_start + map.range;

            if interval.start >= map_end || interval.end <= map_start {
                // * The current interval is outside the bounds of this map, and so this map can be skipped.
                continue;
            }

            if interval.start < map_start {
                // * 'Clip' the start of the interval if it sits outside of this map's lower bound.
                stack.push(Interval::new(
                    interval.start,
                    map_start,
                    interval.category_idx,
                ));
                interval.start = map_start;
            }
            if interval.end > map_end {
                // * 'Clip' the end of the interval if it sits outside of this map's upper bound.
                stack.push(Interval::new(map_end, interval.end, interval.category_idx));
                interval.end = map_end;
            }
            // * Pass the rest of the interval through the map to the next category.
            stack.push(Interval::new(
                interval.start + map.destination_start - map.source_start,
                interval.end + map.destination_start - map.source_start,
                interval.category_idx + 1,
            ));
            found_map = true;
            break;
        }

        if !found_map {
            // * This interval was able to pass through this map without being altered.
            stack.push(Interval::new(
                interval.start,
                interval.end,
                interval.category_idx + 1,
            ));
        }
    }
    return res;
}
