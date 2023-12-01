use std::fmt::Display;
use std::fs::read_to_string;

pub trait Problem {
    fn parse(lines: Vec<String>) -> Self;

    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> ();
}

pub fn run_problem<T: Problem>(filename: &str) {
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .into_iter()
        .map(|x| String::from(x))
        .collect();

    let problem = T::parse(lines);

    let report_first = |x: &dyn Display| {
        println!("First answer: {}", x);
    };

    let report_second = |x: &dyn Display| {
        println!("Second answer: {}", x);
    };

    problem.solve(report_first, report_second);
}
