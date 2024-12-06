## Advent of Code
My solutions to the puzzles hosted during the [Advent of Code](https://adventofcode.com/about) (AoC):
- Solutions are sorted by year, then day (e.g "src/y2024/d2.rs").
- Each "d*.rs" file contains a `pub fn solve_pt1()` and `pub fn solve_pt2()`, which are the 'main' functions for each of the day's respective parts.
- Semi-self-imposed challenge: no crates are used other than that of the standard library (e.g. no using the [regex](https://docs.rs/regex/latest/regex/) crate to make some of the puzzles *significantly* quicker/easier).
- If you want to run these solutions, you'll need to create an "inputs" folder next to the "src" folder that stores your inputs in a "y*/d*.txt" format (or change main.rs' `include_input!` macro to suit your needs).
