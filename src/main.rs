pub mod y2023;
pub mod y2024;

fn main() {
    println!("Hello, Advent of Code!");
    println!("2023 Solutions");
    println!("  day 1, part 1: {}", y2023::d1::solve_pt1());
    println!("  day 1, part 2: {}", y2023::d1::solve_pt2());
    println!("  day 2, part 1: {}", y2023::d2::solve_pt1());
    println!("  day 2, part 2: {}", y2023::d2::solve_pt2());
    println!("  day 3, part 1: {}", y2023::d3::solve_pt1());
    println!("  day 3, part 2: {}", y2023::d3::solve_pt2());
    println!("  day 4, part 1: {}", y2023::d4::solve_pt1());
    println!("  day 4, part 2: {}", y2023::d4::solve_pt2());
    println!("  day 5, part 1: {}", y2023::d5::solve_pt1());
    println!("  day 5, part 2: {}", y2023::d5::solve_pt2());
    println!("  day 6, part 1: {}", y2023::d6::solve_pt1());
    println!("  day 6, part 2: {}", y2023::d6::solve_pt2());

    println!("2024 Solutions");
    println!("  day 1, part 1: {}", y2024::d1::solve_pt1());
    println!("  day 1, part 2: {}", y2024::d1::solve_pt2());
    println!("  day 2, part 1: {}", y2024::d2::solve_pt1());
    println!("  day 2, part 2: {}", y2024::d2::solve_pt2());
}