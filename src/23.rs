use aoc2023::{run_problem, Problem};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

struct Problem23 {
    grid: Vec<Vec<char>>,
}

fn fill(
    grid: &Vec<Vec<char>>,
    (i, j): (usize, usize),
    regions: &mut [Vec<i32>],
    name: i32,
) -> HashSet<(usize, usize)> {
    let mut q = vec![(i, j)];
    let mut visited = HashSet::new();

    while let Some((ci, cj)) = q.pop() {
        if !(0..grid.len()).contains(&ci)
            || !(0..grid[0].len()).contains(&cj)
            || grid[ci][cj] != '.'
            || visited.contains(&(ci, cj))
        {
            continue;
        }

        visited.insert((ci, cj));

        q.extend(vec![
            (ci + 1, cj),
            (ci, cj + 1),
            (ci.wrapping_sub(1), cj),
            (ci, cj.wrapping_sub(1)),
        ]);
    }

    visited.iter().for_each(|(i, j)| regions[*i][*j] = name);

    visited
}

fn longest_path_brute_force(
    graph: &HashMap<i32, Vec<i32>>,
    areas: &HashMap<i32, usize>,
    start: i32,
    target: i32,
) -> usize {
    let mut visited = HashSet::new();
    let mut lens = vec![];

    fn helper(
        cur: i32,
        target: i32,
        graph: &HashMap<i32, Vec<i32>>,
        visited: &mut HashSet<i32>,
        lens: &mut Vec<usize>,
        areas: &HashMap<i32, usize>,
        mut len: usize,
    ) {
        if visited.contains(&cur) {
            return;
        }
        visited.insert(cur);

        len += areas[&cur];
        if cur == target {
            lens.push(len);
        } else {
            for neighbor in graph.get(&cur).unwrap_or(&vec![]) {
                helper(*neighbor, target, graph, visited, lens, areas, len);
            }
        }

        visited.remove(&cur);
    }

    helper(start, target, graph, &mut visited, &mut lens, areas, 0);

    lens.into_iter().max().unwrap()
}

impl Problem for Problem23 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        let mut regions = vec![vec![-1; self.grid[0].len()]; self.grid.len()];
        let mut tiles = HashMap::new();
        let mut next_name = 1;
        let mut areas = HashMap::new();
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == '.' && regions[i][j] == -1 {
                    let visited = fill(&self.grid, (i, j), &mut regions, next_name);
                    areas.insert(next_name, visited.len());
                    tiles.insert(next_name, visited);
                    next_name += 1;
                }
            }
        }

        *areas.get_mut(&1).unwrap() -= 1;

        let target = *regions
            .iter()
            .last()
            .unwrap()
            .iter()
            .filter(|v| v != &&-1)
            .nth(0)
            .unwrap();

        let mut graph_loops: HashMap<i32, Vec<i32>> = Default::default();
        let mut graph_no_loops: HashMap<i32, Vec<i32>> = Default::default();

        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == '>' {
                    graph_no_loops
                        .entry(regions[i][j - 1])
                        .or_default()
                        .push(next_name);
                    graph_no_loops
                        .entry(next_name)
                        .or_default()
                        .push(regions[i][j + 1]);
                    graph_loops
                        .entry(regions[i][j - 1])
                        .or_default()
                        .push(next_name);
                    graph_loops
                        .entry(next_name)
                        .or_default()
                        .push(regions[i][j - 1]);
                    graph_loops
                        .entry(next_name)
                        .or_default()
                        .push(regions[i][j + 1]);
                    graph_loops
                        .entry(regions[i][j + 1])
                        .or_default()
                        .push(next_name);
                    areas.insert(next_name, 1);
                    next_name += 1;
                }
                if self.grid[i][j] == 'v' {
                    graph_no_loops
                        .entry(regions[i - 1][j])
                        .or_default()
                        .push(next_name);
                    graph_no_loops
                        .entry(next_name)
                        .or_default()
                        .push(regions[i + 1][j]);
                    graph_loops
                        .entry(regions[i - 1][j])
                        .or_default()
                        .push(next_name);
                    graph_loops
                        .entry(next_name)
                        .or_default()
                        .push(regions[i + 1][j]);
                    graph_loops
                        .entry(next_name)
                        .or_default()
                        .push(regions[i - 1][j]);
                    graph_loops
                        .entry(regions[i + 1][j])
                        .or_default()
                        .push(next_name);
                    areas.insert(next_name, 1);
                    next_name += 1;
                }
            }
        }

        report_first(&longest_path_brute_force(
            &graph_no_loops,
            &areas,
            1,
            target,
        ));
        report_second(&longest_path_brute_force(&graph_loops, &areas, 1, target));
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
    run_problem::<Problem23>("inputs/23.txt");
}
