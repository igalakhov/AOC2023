use aoc2023::{run_problem, Problem};
use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

struct Problem17 {
    grid: Vec<Vec<i32>>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Ord, PartialOrd)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn adj_dirs(dir: Dir) -> Vec<Dir> {
    match dir {
        Dir::Up | Dir::Down => vec![Dir::Left, Dir::Right],
        Dir::Left | Dir::Right => vec![Dir::Up, Dir::Down],
    }
}

fn adj(
    grid: &Vec<Vec<i32>>,
    (i, j): (usize, usize),
    dir: Dir,
    min_move: usize,
    max_move: usize,
) -> Vec<((usize, usize), Dir, i32)> {
    let mut ret = vec![];

    let (di, dj) = match dir {
        Dir::Up => (-1, 0),
        Dir::Down => (1, 0),
        Dir::Left => (0, -1),
        Dir::Right => (0, 1),
    };

    let (mut ci, mut cj) = (i, j);
    let mut cost = 0;
    for num_moved in 0..max_move {
        ci = ci.wrapping_add_signed(di);
        cj = cj.wrapping_add_signed(dj);
        if !(0..grid.len()).contains(&ci) || !(0..grid[0].len()).contains(&cj) {
            break;
        }
        cost += grid[ci][cj];

        if num_moved + 1 >= min_move {
            ret.extend(adj_dirs(dir).into_iter().map(|ndir| ((ci, cj), ndir, cost)))
        }
    }

    ret
}

fn shortest_path(grid: &Vec<Vec<i32>>, min_move: usize, max_move: usize) -> i32 {
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    heap.push((0, (0, 0), Dir::Right));

    while heap.len() > 0 {
        let (c, loc, dir) = heap.pop().unwrap();
        if visited.contains(&(loc, dir)) {
            continue;
        }

        visited.insert((loc, dir));

        if loc == (grid.len() - 1, grid[0].len() - 1) {
            return -c;
        }

        for (nloc, ndir, cost) in adj(&grid, loc, dir, min_move, max_move) {
            heap.push((c - cost, nloc, ndir));
        }
    }
    unreachable!()
}

impl Problem for Problem17 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        report_first(&shortest_path(&self.grid, 1, 3));
        report_second(&shortest_path(&self.grid, 4, 10));
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            grid: lines
                .into_iter()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as i32)
                        .collect()
                })
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem17>("inputs/17.txt");
}
