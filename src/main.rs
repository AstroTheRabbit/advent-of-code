pub mod y2024;

fn main() {
    println!("Hello, Advent of Code!");
    solve_2024();
}

fn solve_2024() {
    println!("Solution to 2024 day 1, part 1 is: {}.", y2024::d1::solve_pt1());
    println!("Solution to 2024 day 1, part 2 is: {}.", y2024::d1::solve_pt2());
}
