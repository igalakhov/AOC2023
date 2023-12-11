use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::fmt::Display;

struct Problem11 {
    grid: Vec<Vec<char>>,
}

fn solve_with_expansion_factor(grid: &Vec<Vec<char>>, factor: usize) -> usize {
    let empty_rows = (0..grid.len())
        .filter(|i| (0..grid[0].len()).all(|j| grid[*i][j] == '.'))
        .collect_vec();
    let empty_cols = (0..grid[0].len())
        .filter(|j| (0..grid.len()).all(|i| grid[i][*j] == '.'))
        .collect_vec();

    let coords = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|(i, j)| grid[*i][*j] == '#')
        .map(|(i, j)| {
            (
                i + (factor - 1) * empty_rows.iter().filter(|r| *r < &i).count(),
                j + (factor - 1) * empty_cols.iter().filter(|c| *c < &j).count(),
            )
        })
        .collect_vec();

    (coords
        .iter()
        .cartesian_product(coords.iter())
        .map(|((i1, j1), (i2, j2))| (i1.abs_diff(*i2)) + (j1.abs_diff(*j2)))
        .sum::<usize>())
        / 2
}

impl Problem for Problem11 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        report_first(&solve_with_expansion_factor(&self.grid, 2));
        report_second(&solve_with_expansion_factor(&self.grid, 1000000));
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            grid: lines.iter().map(|line| line.chars().collect()).collect(),
        }
    }
}

fn main() {
    run_problem::<Problem11>("inputs/11.txt");
}
