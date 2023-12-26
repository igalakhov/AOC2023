use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::{collections::HashSet, fmt::Display};

struct Problem10 {
    start_pos: (usize, usize),
    grid: Vec<Vec<char>>,
}

fn expand(c: char) -> Vec<Vec<char>> {
    let s = match c {
        '|' => ".X..X..X.",
        '-' => "...XXX...",
        '7' => "...XX..X.",
        'J' => ".X.XX....",
        'L' => ".X..XX...",
        'F' => "....XX.X.",
        '.' | 'S' => ".........",
        _ => todo!(),
    };

    s.chars()
        .chunks(3)
        .into_iter()
        .map(|c| c.collect())
        .collect()
}

fn get_endpoints((si, sj): &(usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    vec![(-2, 0), (2, 0), (0, 2), (0, -2)]
        .into_iter()
        .map(|(i, j)| (si.wrapping_add_signed(i), sj.wrapping_add_signed(j)))
        .filter(|(i, j)| (0..grid.len()).contains(i) && (0..grid[0].len()).contains(j))
        .filter(|(i, j)| grid[*i][*j] == 'X')
        .collect()
}

fn get_furthest_point_and_loop(
    start_pos: &(usize, usize),
    grid: &Vec<Vec<char>>,
) -> (i64, Vec<(usize, usize)>) {
    let (mut ci, mut cj) = get_endpoints(start_pos, grid).first().unwrap();

    let mut path_len = 0;
    let mut visited = HashSet::new();

    loop {
        path_len += 1;
        visited.insert((ci, cj));
        if let Some((ni, nj)) = vec![(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(di, dj)| (ci.wrapping_add_signed(di), cj.wrapping_add_signed(dj)))
            .find(|(i, j)| grid[*i][*j] == 'X' && !visited.contains(&(*i, *j)))
        {
            (ci, cj) = (ni, nj);
        } else {
            break;
        }
    }

    (((path_len / 3) + 1) / 2, visited.into_iter().collect_vec())
}

fn get_contained_points(
    start_pos: &(usize, usize),
    mut grid: Vec<Vec<char>>,
    loop_points: Vec<(usize, usize)>,
) -> usize {
    let (si, sj) = start_pos;
    let endpoints = get_endpoints(start_pos, &grid);

    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let mut loop_points_set = HashSet::new();
    loop_points_set.extend(loop_points);
    for (i, j) in (0..grid_height).cartesian_product(0..grid_width) {
        if !loop_points_set.contains(&(i, j)) {
            grid[i][j] = '.';
        }
    }

    grid[*si][*sj] = 'X';
    endpoints.iter().for_each(|(ei, ej)| {
        grid[(si + ei) / 2][(sj + ej) / 2] = 'X';
    });

    let mut visited = HashSet::new();

    for (si, sj) in vec![(1, 1), (-1, -1), (-1, 1), (1, -1)]
        .into_iter()
        .map(|(di, dj)| (si.wrapping_add_signed(di), sj.wrapping_add_signed(dj)))
    {
        visited.clear();
        let mut queue = vec![(si, sj)];
        while let Some((ci, cj)) = queue.pop() {
            if !(0..grid.len()).contains(&ci)
                || !(0..grid[0].len()).contains(&cj)
                || grid[ci][cj] == 'X'
                || visited.contains(&(ci, cj))
            {
                continue;
            }
            visited.insert((ci, cj));
            queue.extend(
                vec![(-1, 0), (1, 0), (0, 1), (0, -1)]
                    .into_iter()
                    .map(|(di, dj)| (ci.wrapping_add_signed(di), cj.wrapping_add_signed(dj))),
            )
        }
        if !visited.contains(&(0, 0)) {
            break;
        }
    }

    visited
        .into_iter()
        .filter(|(i, j)| (i - 1) % 3 == 0 && (j - 1) % 3 == 0)
        .count()
}

impl Problem for Problem10 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        let (part_1, loop_points) = get_furthest_point_and_loop(&self.start_pos, &self.grid);
        report_first(&part_1);

        report_second(&get_contained_points(
            &self.start_pos,
            self.grid.clone(),
            loop_points,
        ));
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            start_pos: lines
                .iter()
                .enumerate()
                .flat_map(|(i, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(move |(j, c)| (c == 'S').then_some((i, j)))
                })
                .map(|(i, j)| (3 * i + 1, 3 * j + 1))
                .nth(0)
                .unwrap(),

            grid: lines
                .into_iter()
                .flat_map(|line| {
                    (0..3)
                        .map(|i| {
                            line.chars()
                                .flat_map(move |c| expand(c)[i].clone())
                                .collect()
                        })
                        .collect_vec()
                })
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem10>("inputs/10.txt");
}
