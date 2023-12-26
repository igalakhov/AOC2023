use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

struct Problem21 {
    grid: Vec<Vec<char>>,
}

impl Problem for Problem21 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        let mut q = VecDeque::from_iter(vec![(
            0,
            (0..self.grid.len())
                .cartesian_product(0..self.grid[0].len())
                .filter(|(i, j)| self.grid[*i][*j] == 'S')
                .nth(0)
                .unwrap(),
        )]);

        let mut visited = HashSet::new();
        let mut distances = vec![];

        while let Some((d, (ci, cj))) = q.pop_front() {
            if !(0..self.grid.len()).contains(&ci)
                || !(0..self.grid[0].len()).contains(&cj)
                || self.grid[ci][cj] == '#'
                || visited.contains(&(ci, cj))
            {
                continue;
            }

            if !visited.contains(&(ci, cj)) {
                visited.insert((ci, cj));
                distances.push(d);
            }

            q.extend(vec![
                (d + 1, (ci + 1, cj)),
                (d + 1, (ci, cj + 1)),
                (d + 1, (ci.wrapping_sub(1), cj)),
                (d + 1, (ci, cj.wrapping_sub(1))),
            ])
        }

        report_first(
            &distances
                .iter()
                .filter(|v| **v <= 64 && **v % 2 == 0)
                .count(),
        );

        // credit: https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-2
        let even_corners = distances
            .iter()
            .filter(|v| **v % 2 == 0 && **v > 65)
            .count();
        let odd_corners = distances
            .iter()
            .filter(|v| **v % 2 == 1 && **v > 65)
            .count();

        let even_full = distances.iter().filter(|v| **v % 2 == 0).count();
        let odd_full = distances.iter().filter(|v| **v % 2 == 1).count();
        let n = 202300;

        report_second(
            &(((n + 1) * (n + 1)) * odd_full + (n * n) * even_full + n * even_corners
                - (n + 1) * odd_corners),
        );
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            grid: lines
                .into_iter()
                .map(|line| line.chars().collect())
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem21>("inputs/21.txt");
}
