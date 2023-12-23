use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
};

struct Problem23 {
    grid: Vec<Vec<char>>,
}

fn fill(
    grid: &Vec<Vec<char>>,
    (i, j): (usize, usize),
    regions: &mut Vec<Vec<i32>>,
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

fn longest_path(
    graph: &HashMap<i32, Vec<i32>>,
    areas: &HashMap<i32, usize>,
    cur: i32,
    target: i32,
    memo: &mut HashMap<i32, Option<(usize, Vec<i32>)>>,
) -> Option<(usize, Vec<i32>)> {
    if memo.contains_key(&cur) {
        return memo[&cur].clone();
    }

    if cur == target {
        return Some((areas[&cur], vec![cur]));
    }

    let mut ret = graph
        .get(&cur)
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|neighbor| longest_path(graph, areas, *neighbor, target, memo))
        .max();

    ret.iter_mut().for_each(|(v, path)| {
        *v += areas[&cur];
        path.push(cur);
    });

    memo.insert(cur, ret.clone());

    ret
}

impl Problem for Problem23 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
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

        let mut graph: HashMap<i32, Vec<i32>> = Default::default();

        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == '>' {
                    graph.entry(regions[i][j - 1]).or_default().push(next_name);
                    graph.entry(next_name).or_default().push(regions[i][j + 1]);
                    areas.insert(next_name, 1);
                    next_name += 1;
                }
                if self.grid[i][j] == 'v' {
                    graph.entry(regions[i - 1][j]).or_default().push(next_name);
                    graph.entry(next_name).or_default().push(regions[i + 1][j]);
                    areas.insert(next_name, 1);
                    next_name += 1;
                }
                if self.grid[i][j] == '<' {
                    graph.entry(regions[i][j + 1]).or_default().push(next_name);
                    graph.entry(next_name).or_default().push(regions[i][j - 1]);
                    areas.insert(next_name, 1);
                    next_name += 1;
                }
                if self.grid[i][j] == '^' {
                    graph.entry(regions[i + 1][j]).or_default().push(next_name);
                    graph.entry(next_name).or_default().push(regions[i - 1][j]);
                    areas.insert(next_name, 1);
                    next_name += 1;
                }
            }
        }

        let mut memo = HashMap::new();

        if let Some((len, path)) = longest_path(&graph, &areas, 1, target, &mut memo) {
            // let mut grid_copy = self.grid.clone();
            // let mut ll = 0;
            // path.iter().for_each(|p| {
            //     if let Some(pts) = tiles.get(p) {
            //         ll += pts.len();
            //         pts.iter().for_each(|(i, j)| {
            //             grid_copy[*i][*j] = 'O';
            //         })
            //     } else {
            //         ll += 1;
            //     }
            // });
            //
            // for i in grid_copy {
            //     println!("{}", i.iter().collect::<String>());
            // }
            // println!("{path:?} {ll}");
            report_first(&len);
        }
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
