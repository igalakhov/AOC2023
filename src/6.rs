use aoc2023::{run_problem, Problem};
use std::fmt::Display;

struct Problem6 {
    races: Vec<(f64, f64)>,
    concat_races: (f64, f64),
}

fn num_solutions(t: f64, d: f64) -> i64 {
    // (t-x)*x > d
    let a = 1.0;
    let b = -t;
    let c = d;

    let r1 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a) + 0.00000001;
    let r2 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a) - 0.00000001;

    (r2.floor() as i64) - (r1.ceil() as i64) + 1
}

impl Problem for Problem6 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        report_first(
            &self
                .races
                .iter()
                .map(|(t, d)| num_solutions(*t, *d))
                .product::<i64>(),
        );

        report_second(&num_solutions(self.concat_races.0, self.concat_races.1));
    }

    fn parse(lines: Vec<String>) -> Self {
        let parse_ints = |line: &String| {
            line.split(':')
                .nth(1)
                .unwrap()
                .to_string()
                .trim()
                .replace("  ", " ")
                .replace("  ", " ")
                .replace("  ", " ")
                .split(' ')
                .map(|s| s.parse::<f64>().unwrap())
                .collect::<Vec<_>>()
        };

        Self {
            races: parse_ints(&lines[0])
                .into_iter()
                .zip(parse_ints(&lines[1]))
                .collect(),
            concat_races: (
                lines[0]
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .trim()
                    .replace([' ', ' '], "")
                    .parse::<f64>()
                    .unwrap(),
                lines[1]
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .trim()
                    .replace([' ', ' '], "")
                    .parse::<f64>()
                    .unwrap(),
            ),
        }
    }
}

fn main() {
    run_problem::<Problem6>("inputs/6.txt");
}
