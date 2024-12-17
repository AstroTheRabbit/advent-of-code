pub mod y2023;
pub mod y2024;

#[macro_export]
macro_rules! include_input {
    ($year:expr, $day:expr) => {
        include_str!(concat!("../../inputs/y", $year, "/d", $day, ".txt"))
    };
}

fn main() {
    println!("Hello, Advent of Code!");
    // println!("2023 Solutions");
    // println!("  day 1, part 1: {}", y2023::d1::solve_pt1());
    // println!("  day 1, part 2: {}", y2023::d1::solve_pt2());
    // println!("  day 2, part 1: {}", y2023::d2::solve_pt1());
    // println!("  day 2, part 2: {}", y2023::d2::solve_pt2());
    // println!("  day 3, part 1: {}", y2023::d3::solve_pt1());
    // println!("  day 3, part 2: {}", y2023::d3::solve_pt2());
    // println!("  day 4, part 1: {}", y2023::d4::solve_pt1());
    // // println!("  day 4, part 2: {}", y2023::d4::solve_pt2()); // ! Slow
    // println!("  day 5, part 1: {}", y2023::d5::solve_pt1());
    // println!("  day 5, part 2: {}", y2023::d5::solve_pt2());
    // println!("  day 6, part 1: {}", y2023::d6::solve_pt1());
    // println!("  day 6, part 2: {}", y2023::d6::solve_pt2());

    // println!("2024 Solutions");
    // println!("  day 1, part 1: {}", y2024::d1::solve_pt1());
    // println!("  day 1, part 2: {}", y2024::d1::solve_pt2());
    // println!("  day 2, part 1: {}", y2024::d2::solve_pt1());
    // println!("  day 2, part 2: {}", y2024::d2::solve_pt2());
    // println!("  day 3, part 1: {}", y2024::d3::solve_pt1());
    // println!("  day 3, part 2: {}", y2024::d3::solve_pt2());
    // println!("  day 4, part 1: {}", y2024::d4::solve_pt1());
    // println!("  day 4, part 2: {}", y2024::d4::solve_pt2());
    // println!("  day 5, part 1: {}", y2024::d5::solve_pt1());
    // println!("  day 5, part 2: {}", y2024::d5::solve_pt2());
    // println!("  day 6, part 1: {}", y2024::d6::solve_pt1());
    // // println!("  day 6, part 2: {}", y2024::d6::solve_pt2()); // ! Slow
    // // println!("  day 7, part 1: {}", y2024::d7::solve_pt1()); // ! Slow
    // // println!("  day 7, part 2: {}", y2024::d7::solve_pt2()); // ! Slow
    // println!("  day 8, part 1: {}", y2024::d8::solve_pt1());
    // println!("  day 8, part 2: {}", y2024::d8::solve_pt2());
    // println!("  day 9, part 1: {}", y2024::d9::solve_pt1());
    // println!("  day 9, part 2: {}", y2024::d9::solve_pt2());
    // println!("  day 10, part 1: {}", y2024::d10::solve_pt1());
    // println!("  day 10, part 2: {}", y2024::d10::solve_pt2());
    // println!("  day 11, part 1: {}", y2024::d11::solve_pt1());
    // println!("  day 11, part 2: {}", y2024::d11::solve_pt2());
    // println!("  day 12, part 1: {}", y2024::d12::solve_pt1());
    // println!("  day 12, part 2: {}", y2024::d12::solve_pt2());
    // println!("  day 13, part 1: {}", y2024::d13::solve_pt1());
    // println!("  day 13, part 2: {}", y2024::d13::solve_pt2());
    // println!("  day 14, part 1: {}", y2024::d14::solve_pt1());
    // println!("  day 14, part 2: {}", y2024::d14::solve_pt2());
    // println!("  day 15, part 1: {}", y2024::d15::solve_pt1());
    // println!("  day 15, part 2: {}", y2024::d15::solve_pt2());
    println!("  day 17, part 1: {}", y2024::d17::solve_pt1());
    // println!("  day 17, part 2: {}", y2024::d17::solve_pt2());
}
