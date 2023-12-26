use aoc2023::{run_problem, Problem};
use std::fmt::Display;

struct Problem13 {
    grids: Vec<Vec<Vec<char>>>,
}

fn part_one_contribution(grid: &Vec<Vec<char>>, ignore: usize) -> usize {
    for j in 1..grid[0].len() {
        let to_check = j.min(grid[0].len() - j);
        if (0..to_check).all(|o| (0..grid.len()).all(|i| grid[i][j - o - 1] == grid[i][j + o]))
            && j != ignore
        {
            return j;
        }
    }

    for i in 1..grid.len() {
        let to_check = i.min(grid.len() - i);

        if (0..to_check).all(|o| (0..grid[0].len()).all(|j| grid[i - o - 1][j] == grid[i + o][j]))
            && 100 * i != ignore
        {
            return 100 * i;
        }
    }

    0
}

impl Problem for Problem13 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        report_first(
            &self
                .grids
                .iter()
                .map(|grid| part_one_contribution(grid, 0))
                .sum::<usize>(),
        );

        report_second(
            &self
                .grids
                .iter()
                .map(|grid| {
                    let first_answer = part_one_contribution(grid, 0);
                    let mut grid_copy = grid.clone();
                    let n = grid.len();
                    let m = grid[0].len();
                    for i in 0..n {
                        for j in 0..m {
                            if grid[i][j] == '#' {
                                grid_copy[i][j] = '.';
                            } else {
                                grid_copy[i][j] = '#';
                            }

                            let new_answer = part_one_contribution(&grid_copy, first_answer);
                            if new_answer != 0 {
                                return new_answer;
                            }
                            grid_copy[i][j] = grid[i][j];
                        }
                    }
                    panic!()
                })
                .sum::<usize>(),
        )
    }

    fn parse(lines: Vec<String>) -> Self {
        let mut grids = vec![];
        let mut cur_grid = vec![];

        for line in lines {
            if line.is_empty() {
                grids.push(cur_grid);
                cur_grid = vec![];
            } else {
                cur_grid.push(line.chars().collect())
            }
        }

        grids.push(cur_grid);

        Self { grids }
    }
}

fn main() {
    run_problem::<Problem13>("inputs/13.txt");
}
