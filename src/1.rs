use aoc2023::{run_problem, Problem};
use std::fmt::Display;

struct Problem1 {}

impl Problem for Problem1 {
    fn parse(lines: Vec<String>) -> Self {
        Problem1 {}
    }

    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
    }
}

fn main() {
    run_problem::<Problem1>("inputs/1.txt");
}
