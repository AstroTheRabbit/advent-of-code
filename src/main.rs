pub mod y2023;
pub mod y2024;

fn main() {
    println!("Hello, Advent of Code!");
    solve_2023();
    // solve_2024();
}

fn solve_2023() {
    println!("Solution to 2023 day 1, part 1 is: {}", y2023::d1::solve_pt1());
    println!("Solution to 2023 day 1, part 1 is: {}", y2023::d1::solve_pt2());
}

fn solve_2024() {
    println!("Solution to 2024 day 1, part 1 is: {}", y2024::d1::solve_pt1());
    println!("Solution to 2024 day 1, part 2 is: {}", y2024::d1::solve_pt2());
}
