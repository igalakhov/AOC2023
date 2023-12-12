use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::fmt::Display;

struct Problem12 {
    pats: Vec<(String, Vec<usize>)>,
}

fn solve_case_memo(
    pat: &String,
    sizes: &Vec<usize>,
    pi: usize,
    si: usize,
    memo: &mut Vec<Vec<usize>>,
    seen: &mut Vec<Vec<bool>>,
) -> usize {
    if pi >= pat.len() {
        if si == sizes.len() {
            return 1;
        } else {
            return 0;
        }
    }

    if seen[pi][si] {
        return memo[pi][si];
    }
    seen[pi][si] = true;
    let mut ret = 0;

    if pat.as_bytes()[pi] != b'#' {
        ret += solve_case_memo(pat, sizes, pi + 1, si, memo, seen);
    }

    if si != sizes.len()
        && pi + sizes[si] <= pat.len()
        && pat.as_bytes()[pi..pi + sizes[si]]
            .iter()
            .all(|c| c != &b'.')
        && (pi + sizes[si] == pat.len() || pat.as_bytes()[pi + sizes[si]] != b'#')
    {
        ret += solve_case_memo(pat, sizes, pi + sizes[si] + 1, si + 1, memo, seen);
    }

    memo[pi][si] = ret;
    ret
}

fn solve_case(pat: &String, sizes: &Vec<usize>) -> usize {
    solve_case_memo(
        pat,
        sizes,
        0,
        0,
        &mut vec![vec![0; sizes.len() + 1]; pat.len()],
        &mut vec![vec![false; sizes.len() + 1]; pat.len()],
    )
}

impl Problem for Problem12 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        report_first(
            &self
                .pats
                .iter()
                .map(|(pat, sizes)| solve_case(pat, sizes))
                .sum::<usize>(),
        );

        report_second(
            &self
                .pats
                .clone()
                .into_iter()
                .map(|(pat, sizes)| {
                    let pat_len = pat.len();
                    let sizes_len = sizes.len();

                    solve_case(
                        &(0..5)
                            .flat_map(|_| pat.chars().chain("?".chars()))
                            .take(4 + 5 * pat_len)
                            .collect(),
                        &sizes.into_iter().cycle().take(5 * sizes_len).collect_vec(),
                    )
                })
                .sum::<usize>(),
        );
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            pats: lines
                .into_iter()
                .map(|line| {
                    let (pat, nums) = line.split(" ").collect_tuple().unwrap();
                    (
                        pat.to_string(),
                        nums.split(",")
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem12>("inputs/12.txt");
}
