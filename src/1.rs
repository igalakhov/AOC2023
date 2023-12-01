use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use itertools::MinMaxResult;
use std::fmt::Display;

struct Problem1 {
    lines: Vec<String>,
}

impl Problem for Problem1 {
    fn parse(lines: Vec<String>) -> Self {
        Problem1 { lines }
    }

    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        report_first(
            &(self
                .lines
                .iter()
                .map(|x: &String| {
                    x.clone()
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<Vec<char>>()
                })
                .map(|v: Vec<char>| {
                    format!("{}{}", v.first().unwrap(), v.last().unwrap())
                        .parse::<i64>()
                        .unwrap()
                })
                .sum::<i64>()),
        );

        report_second(
            &self
                .lines
                .iter()
                .map(|line| {
                    match vec![
                        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                    ]
                    .into_iter()
                    .map(str::to_string)
                    .zip(1..10)
                    .map(|(i, n)| (i.to_string(), n))
                    .chain((1..10).map(|i| (format!("{}", i), i)))
                    .map(|(word, value)| {
                        line.match_indices(word.as_str())
                            .map(move |(idx, _)| (idx, value.clone()))
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .minmax_by_key(|(idx, _)| *idx)
                    {
                        MinMaxResult::OneElement((_, d)) => 11 * d,
                        MinMaxResult::MinMax((_, d1), (_, d2)) => 10 * d1 + d2,
                        MinMaxResult::NoElements => panic!(),
                    }
                })
                .sum::<i64>(),
        );
    }
}

fn main() {
    run_problem::<Problem1>("inputs/1.txt");
}
