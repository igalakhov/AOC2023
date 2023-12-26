use aoc2023::{run_problem, Problem};
use std::{collections::HashSet, fmt::Display};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn apply(i: usize, j: usize, dir: Dir) -> (usize, usize) {
    match dir {
        Dir::Up => (i.wrapping_sub(1), j),
        Dir::Down => (i + 1, j),
        Dir::Left => (i, j.wrapping_sub(1)),
        Dir::Right => (i, j + 1),
    }
}

fn visit(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<((usize, usize), Dir)>,
    (i, j): (usize, usize),
    dir: Dir,
) {
    if !(0..grid.len()).contains(&i)
        || !(0..grid[0].len()).contains(&j)
        || visited.contains(&((i, j), dir))
    {
        return;
    }

    visited.insert(((i, j), dir));

    match (grid[i][j], dir) {
        ('|', Dir::Left | Dir::Right) => {
            visit(grid, visited, (i, j), Dir::Up);
            visit(grid, visited, (i, j), Dir::Down);
        }
        ('-', Dir::Up | Dir::Down) => {
            visit(grid, visited, (i, j), Dir::Left);
            visit(grid, visited, (i, j), Dir::Right);
        }
        ('/', Dir::Right) => visit(grid, visited, apply(i, j, Dir::Up), Dir::Up),
        ('/', Dir::Left) => visit(grid, visited, apply(i, j, Dir::Down), Dir::Down),
        ('/', Dir::Up) => visit(grid, visited, apply(i, j, Dir::Right), Dir::Right),
        ('/', Dir::Down) => visit(grid, visited, apply(i, j, Dir::Left), Dir::Left),
        ('\\', Dir::Left) => visit(grid, visited, apply(i, j, Dir::Up), Dir::Up),
        ('\\', Dir::Right) => visit(grid, visited, apply(i, j, Dir::Down), Dir::Down),
        ('\\', Dir::Down) => visit(grid, visited, apply(i, j, Dir::Right), Dir::Right),
        ('\\', Dir::Up) => visit(grid, visited, apply(i, j, Dir::Left), Dir::Left),

        _ => visit(grid, visited, apply(i, j, dir), dir),
    }
}

fn visited_from_start(grid: &Vec<Vec<char>>, loc: (usize, usize), dir: Dir) -> usize {
    let mut visited = HashSet::new();

    visit(grid, &mut visited, loc, dir);

    visited
        .into_iter()
        .map(|(loc, _)| loc)
        .collect::<HashSet<_>>()
        .len()
}

struct Problem16 {
    grid: Vec<Vec<char>>,
}

impl Problem for Problem16 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        report_first(&visited_from_start(&self.grid, (0, 0), Dir::Right));

        report_second(
            &(0..self.grid.len())
                .flat_map(|i| {
                    vec![
                        ((i, 0), Dir::Right),
                        ((i, self.grid[0].len() - 1), Dir::Left),
                    ]
                })
                .chain(
                    (0..self.grid[0].len()).flat_map(|j| {
                        vec![((0, j), Dir::Down), ((self.grid.len() - 1, j), Dir::Up)]
                    }),
                )
                .map(|(loc, dir)| visited_from_start(&self.grid, loc, dir))
                .max()
                .unwrap(),
        )
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
    run_problem::<Problem16>("inputs/16.txt");
}
