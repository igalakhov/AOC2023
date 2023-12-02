use aoc2023::{run_problem, Problem};
use std::{collections::HashMap, fmt::Display};

struct Problem2 {
    games: HashMap<i64, Vec<HashMap<String, i64>>>,
}

impl Problem for Problem2 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        let max_amounts: HashMap<String, i64> = vec![("red", 12), ("green", 13), ("blue", 14)]
            .into_iter()
            .map(|(name, amt)| (name.to_string(), amt))
            .collect();

        report_first(
            &self
                .games
                .iter()
                .filter_map(|(id, showings)| {
                    showings
                        .iter()
                        .all(|showing| {
                            max_amounts
                                .iter()
                                .all(|(name, max_amt)| showing.get(name).unwrap_or(&0) <= max_amt)
                        })
                        .then(|| *id)
                })
                .sum::<i64>(),
        );

        report_second(
            &self
                .games
                .iter()
                .map(|(_, showings)| {
                    max_amounts
                        .iter()
                        .map(|(color, _)| {
                            showings
                                .iter()
                                .map(|showing| showing.get(color).unwrap_or(&0))
                                .max()
                                .unwrap()
                        })
                        .product::<i64>()
                })
                .sum::<i64>(),
        )
    }

    fn parse(lines: Vec<String>) -> Self {
        Problem2 {
            games: lines
                .clone()
                .into_iter()
                .map(|line| {
                    let parts: Vec<_> = line.split([':', ';']).map(str::to_string).collect();

                    let game_id = parts[0]
                        .split(' ')
                        .into_iter()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap();

                    let showings: Vec<_> = parts[1..]
                        .into_iter()
                        .map(|showing| {
                            showing
                                .trim()
                                .split(", ")
                                .map(|show| {
                                    let parts: Vec<_> = show.split(" ").collect();
                                    (
                                        parts[1].to_string(),
                                        parts[0].to_string().parse::<i64>().unwrap(),
                                    )
                                })
                                .collect::<HashMap<_, _>>()
                        })
                        .collect();
                    (game_id, showings)
                })
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem2>("inputs/2.txt");
}
