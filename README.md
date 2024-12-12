## Advent of Code
My solutions to the puzzles hosted on [Advent of Code](https://adventofcode.com/about) (AoC):
- Solutions are sorted by year, then day (e.g "src/y2024/d2.rs").
- Each "d*.rs" file contains a `pub fn solve_pt1()` and `pub fn solve_pt2()`, which are the 'main' functions for each of the day's respective parts.
- Semi-self-imposed challenge: no crates are used other than that of the standard library (e.g. no using the [regex](https://docs.rs/regex/latest/regex/) crate to make some of the puzzles *significantly* quicker/easier).
- Each "d*.rs" is also (nearly) completely self-contained, and can be run without requiring other files (the only exception is the custom `include_input!` macro, which can easily be changed/replaced if you want to run this code yourself).
