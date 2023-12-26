use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::fmt::Display;

struct Problem9 {
    sequences: Vec<Vec<i64>>,
}

fn get_seqs(init: &[i64]) -> Vec<Vec<i64>> {
    let mut seqs = vec![init.to_owned()];

    while !seqs.last().unwrap().iter().all(|x| x == &0) {
        seqs.push(
            seqs.last()
                .unwrap()
                .clone()
                .into_iter()
                .tuple_windows()
                .map(|(x, y)| y - x)
                .collect(),
        )
    }

    seqs
}

fn predict_next(seq: &[i64]) -> i64 {
    get_seqs(seq).iter().map(|s| s.last().unwrap()).sum::<i64>()
}

fn predict_prev(seq: &[i64]) -> i64 {
    get_seqs(seq)
        .into_iter()
        .map(|s| *s.first().unwrap())
        .enumerate()
        .map(|(idx, num)| if idx % 2 == 0 { num } else { -num })
        .sum::<i64>()
}

impl Problem for Problem9 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        report_first(&self.sequences.iter().map(|v| predict_next(v)).sum::<i64>());

        report_second(&self.sequences.iter().map(|v| predict_prev(v)).sum::<i64>());
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            sequences: lines
                .into_iter()
                .map(|line| line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect())
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem9>("inputs/9.txt");
}
