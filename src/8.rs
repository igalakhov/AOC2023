use aoc2023::{run_problem, Problem};
use regex::Regex;
use std::{collections::HashMap, fmt::Display};

struct Problem8 {
    path: String,
    graph: HashMap<String, (String, String)>,
}

fn lcm(mut m: i64, mut n: i64) -> i64 {
    let p = m * n;
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    p / n
}

impl Problem for Problem8 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        {
            let mut steps = 0;
            let mut cur = "AAA";
            while cur != "ZZZ" {
                let (left, right) = self.graph.get(cur).unwrap();
                if self.path.as_bytes()[steps % self.path.len()] == b'R' {
                    cur = right;
                } else {
                    cur = left;
                }
                steps += 1;
            }

            report_first(&steps);
        }
        {
            let mut part2 = 1;

            for (start, _) in &self.graph {
                if start.as_bytes()[2] != b'A' {
                    continue;
                }

                let mut steps = 0;
                let mut cur = start;

                while cur.as_bytes()[2] != b'Z' {
                    let (left, right) = self.graph.get(cur).unwrap();
                    if self.path.as_bytes()[steps % self.path.len()] == b'R' {
                        cur = right;
                    } else {
                        cur = left;
                    }
                    steps += 1;
                }

                part2 = lcm(part2, steps as i64);
            }

            report_second(&part2);
        }
    }

    fn parse(lines: Vec<String>) -> Self {
        let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

        Self {
            path: lines[0].clone(),
            graph: lines[2..]
                .into_iter()
                .map(|m| re.captures(m).unwrap().extract())
                .map(|(_, [name, left, right])| {
                    (name.to_string(), (left.to_string(), right.to_string()))
                })
                .collect::<HashMap<_, _>>(),
        }
    }
}

fn main() {
    run_problem::<Problem8>("inputs/8.txt");
}
