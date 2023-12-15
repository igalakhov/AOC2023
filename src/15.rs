use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

struct Problem15 {
    codes: Vec<String>,
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |h, c| (17 * (h + c as usize)) % 256)
}

impl Problem for Problem15 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        report_first(&self.codes.iter().map(|code| hash(&code)).sum::<usize>());

        let mut boxes: HashMap<usize, Vec<(String, usize)>> = Default::default();

        for code in &self.codes {
            if code.chars().last().unwrap() == '-' {
                let to_remove = code.split("-").nth(0).unwrap();
                boxes
                    .entry(hash(to_remove))
                    .or_insert(vec![])
                    .retain(|(name, _)| name != to_remove);
            } else {
                let (to_add, length) = code.split("=").collect_tuple().unwrap();

                let entry = boxes.entry(hash(to_add)).or_insert(vec![]);

                if !entry.iter().any(|(name, _)| name == to_add) {
                    entry.retain(|(name, _)| name != to_add);
                    entry.push((to_add.to_string(), length.parse::<usize>().unwrap()));
                } else {
                    entry.iter_mut().for_each(|(name, len)| {
                        if name == to_add {
                            *len = length.parse::<usize>().unwrap();
                        }
                    });
                }
            }
        }

        report_second(
            &boxes
                .iter()
                .map(|(box_num, lenses)| {
                    lenses
                        .iter()
                        .enumerate()
                        .map(|(idx, (_, fl))| (box_num + 1) * (idx + 1) * fl)
                        .sum::<usize>()
                })
                .sum::<usize>(),
        );
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            codes: lines
                .into_iter()
                .flat_map(|line| line.split(",").map(str::to_string).collect_vec())
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem15>("inputs/15.txt");
}
