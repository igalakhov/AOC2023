use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::fmt::Display;

struct Problem3 {
    grid: Vec<Vec<char>>,
}

impl Problem for Problem3 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        let mut first_answer = 0;
        let mut gear_mapping: Vec<((i32, i32), i32)> = vec![];

        for i in 0..self.grid.len() {
            let mut j = 0;
            while j < self.grid[0].len() {
                if !self.grid[i][j].is_numeric() {
                    j += 1;
                    continue;
                }

                let start_idx = j;

                while j < self.grid[0].len() && self.grid[i][j].is_numeric() {
                    j += 1;
                }

                let number = self.grid[i][start_idx..j]
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();

                let gears = (i as i32 - 1..i as i32 + 2)
                    .cartesian_product(start_idx as i32 - 1..j as i32 + 1)
                    .filter(|(i, j)| {
                        (0..self.grid.len() as i32).contains(i)
                            && (0..self.grid[0].len() as i32).contains(j)
                    })
                    .filter(|(i, j)| {
                        let val = self.grid[*i as usize][*j as usize];
                        !val.is_numeric() && val != '.'
                    })
                    .collect::<Vec<_>>();

                if !gears.is_empty() {
                    first_answer += number;
                }

                for (gi, gj) in gears {
                    if self.grid[gi as usize][gj as usize] == '*' {
                        gear_mapping.push(((gi, gj), number))
                    }
                }
            }
        }

        gear_mapping.sort();

        report_first(&first_answer);
        report_second(
            &gear_mapping
                .iter()
                .group_by(|(gear, _)| gear)
                .into_iter()
                .filter_map(|(_, nums)| {
                    let nums: Vec<_> = nums.map(|(_, num)| num).collect();
                    (nums.len() == 2).then(|| nums[0] * nums[1])
                })
                .sum::<i32>(),
        )
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            grid: lines.iter().map(|s| s.chars().collect()).collect(),
        }
    }
}

fn main() {
    run_problem::<Problem3>("inputs/3.txt");
}
