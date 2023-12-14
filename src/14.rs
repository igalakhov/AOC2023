use aoc2023::{run_problem, Problem};
use std::{collections::HashMap, fmt::Display};

struct Problem14 {
    grid: Vec<Vec<char>>,
}

fn tilt_up(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();

    for j in 0..cols {
        for i in 0..rows {
            if grid[i][j] != 'O' {
                continue;
            }
            let mut falls_to = i;
            while falls_to > 0 && grid[falls_to - 1][j] == '.' {
                falls_to -= 1;
            }
            grid[i][j] = '.';
            grid[falls_to][j] = 'O';
        }
    }

    grid
}

fn cycle(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _ in 0..4 {
        grid = tilt_up(grid);

        grid = (0..grid[0].len())
            .map(|j| {
                let ret = (0..grid.len())
                    .map(|i| grid[grid.len() - i - 1][j])
                    .collect();
                ret
            })
            .collect();
    }

    grid
}

impl Problem for Problem14 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        report_first(
            &tilt_up(self.grid.clone())
                .into_iter()
                .enumerate()
                .map(|(idx, row)| {
                    (self.grid.len() - idx) * row.into_iter().filter(|c| c == &'O').count()
                })
                .sum::<usize>(),
        );

        let mut cur = self.grid.clone();
        let mut seen = HashMap::new();
        let mut cycles_done = 0;
        let mut cycle_size = None;
        let mut residues = HashMap::new();
        let mut target_residue = None;
        loop {
            cur = cycle(cur);
            let cur_string = cur.iter().flatten().collect::<String>();
            if seen.contains_key(&cur_string) {
                cycle_size = Some(cycles_done - seen.get(&cur_string).unwrap());
                target_residue = Some(1000000000 % cycle_size.unwrap());
            }
            seen.insert(cur_string, cycles_done);
            cycles_done += 1;

            if let Some(sz) = cycle_size {
                let ans = cur
                    .iter()
                    .enumerate()
                    .map(|(idx, row)| {
                        (self.grid.len() - idx) * row.into_iter().filter(|c| c == &&'O').count()
                    })
                    .sum::<usize>();

                residues.insert(cycles_done % sz, ans);

                if cycles_done % sz == target_residue.unwrap() {
                    break;
                }
            }
        }

        report_second(&(residues.get(&(target_residue.unwrap())).unwrap()));
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            grid: lines.iter().map(|line| line.chars().collect()).collect(),
        }
    }
}

fn main() {
    run_problem::<Problem14>("inputs/14.txt");
}
