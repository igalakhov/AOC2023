use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::cmp;
use std::fmt::Display;

#[derive(Clone, Debug)]
struct Range {
    len: i64,
    src: i64,
    dest: i64,
} // [src, src + len - 1] -> [dest, dest + len - 1]

impl Range {
    fn map(&self, val: i64) -> Option<i64> {
        if val >= self.src && val < self.src + self.len {
            Some(self.dest + (val - self.src))
        } else {
            None
        }
    }
}

fn map_over_range(ranges: &Vec<Range>, val: i64) -> i64 {
    for range in ranges {
        if let Some(val) = range.map(val) {
            return val;
        }
    }
    val
}

fn map_interval_over_ranges(ranges: Vec<Range>, l: i64, r: i64) -> Vec<(i64, i64)> {
    if r < l {
        return vec![];
    }

    if ranges.is_empty() {
        return vec![(l, r)];
    }

    let first = ranges[0].clone();
    let rest = ranges.into_iter().skip(1).collect::<Vec<_>>();

    let il = cmp::max(l, first.src);
    let ir = cmp::min(r, first.src + first.len - 1);

    let mut ret = vec![];

    if ir >= il {
        ret.push((il + first.dest - first.src, ir + first.dest - first.src));

        for it in map_interval_over_ranges(rest.clone(), l, il - 1) {
            ret.push(it);
        }

        for it in map_interval_over_ranges(rest, ir + 1, r) {
            ret.push(it);
        }
    } else {
        for it in map_interval_over_ranges(rest, l, r) {
            ret.push(it);
        }
    }

    ret
}

struct Problem5 {
    seeds: Vec<i64>,
    mappings: Vec<Vec<Range>>,
}

impl Problem for Problem5 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        let mut cur = self.seeds.clone();

        for range in &self.mappings {
            cur = cur.iter().map(|val| map_over_range(range, *val)).collect();
        }

        report_first(&cur.iter().min().unwrap());

        let mut ints = self
            .seeds
            .iter()
            .tuples()
            .map(|(l, len)| (*l, *l + len - 1))
            .collect::<Vec<_>>();

        for mapping in &self.mappings {
            ints = ints
                .into_iter()
                .flat_map(|(l, r)| map_interval_over_ranges(mapping.clone(), l, r))
                .collect::<Vec<_>>();
        }

        report_second(&ints.iter().map(|(l, _)| *l).min().unwrap());
    }

    fn parse(lines: Vec<String>) -> Self {
        let seeds = lines[0]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let num_lines = lines.len();
        let mut i = 2;

        let mut mappings = vec![];

        while i < num_lines {
            let mut j = i + 1;
            while j < num_lines && !lines[j].is_empty() {
                j += 1;
            }

            let mapping = lines[i + 1..j]
                .iter()
                .map(|s| {
                    let md = s
                        .split(' ')
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();

                    Range {
                        len: md[2],
                        src: md[1],
                        dest: md[0],
                    }
                })
                .collect::<Vec<_>>();

            mappings.push(mapping);

            i = j + 1;
        }

        Self { seeds, mappings }
    }
}

fn main() {
    run_problem::<Problem5>("inputs/5.txt");
}
