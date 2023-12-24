use aoc2023::{run_problem, Problem};
use geo_types::{Line, Point};
use itertools::Itertools;
use line_intersection::LineInterval;
use std::fmt::Display;

struct Problem24 {
    stones: Vec<((f64, f64, f64), (f64, f64, f64))>,
}

fn intersect_2d(
    ((x1, y1, _), (vx1, vy1, _)): ((f64, f64, f64), (f64, f64, f64)),
    ((x2, y2, _), (vx2, vy2, _)): ((f64, f64, f64), (f64, f64, f64)),
) -> Option<(f64, f64)> {
    let (m1, b1) = (vy1 / vx1, -x1 * (vy1 / vx1) + y1);
    let (m2, b2) = (vy2 / vx2, -x2 * (vy2 / vx2) + y2);

    if m1 == m2 {
        return None;
    }

    let x = (b2 - b1) / (m1 - m2);
    let y = m1 * x + b1;

    let t1 = (x - x1) / vx1;
    let t2 = (x - x2) / vx2;

    return if t1 >= 0.0 && t2 >= 0.0 {
        Some((x, y))
    } else {
        None
    };
}

impl Problem for Problem24 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        report_first(
            &(0..self.stones.len())
                .flat_map(|i| (i + 1..self.stones.len()).map(move |j| (i, j)))
                .filter(|(i, j)| {
                    const MI: f64 = 200000000000000.0;
                    const MA: f64 = 400000000000000.0;
                    intersect_2d(self.stones[*i].into(), self.stones[*j].into())
                        .map_or_else(|| false, |(x, y)| x >= MI && y >= MI && x <= MA && y <= MA)
                })
                .count(),
        );
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            stones: lines
                .into_iter()
                .map(|line| {
                    let (pos, vel) = line.split(" @ ").collect_tuple().unwrap();
                    let parse = |p: &str| {
                        p.replace(" ", "")
                            .split(",")
                            .map(|x| x.parse::<f64>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    };
                    (parse(pos), parse(vel))
                })
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem24>("inputs/24.txt");
}
