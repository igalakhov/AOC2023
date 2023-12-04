use aoc2023::{run_problem, Problem};
use std::fmt::Display;

struct Problem4 {
    cards: Vec<(Vec<i64>, Vec<i64>)>,
}

impl Problem for Problem4 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        let card_scores: Vec<_> = self
            .cards
            .iter()
            .map(|(numbers, winning)| {
                numbers
                    .iter()
                    .filter_map(|num| winning.contains(num).then_some(1))
                    .sum::<i64>()
            })
            .collect();

        report_first(
            &card_scores
                .iter()
                .map(|score| if *score == 0 { 0 } else { 1 << (score - 1) })
                .sum::<i64>(),
        );

        let num_cards = self.cards.len();
        let mut copies = vec![1; self.cards.len()];

        for i in 0..num_cards {
            for j in (i + 1)..(i + (card_scores[i] as usize) + 1) {
                copies[j] += copies[i];
            }
        }

        report_second(&copies.iter().sum::<usize>());
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            cards: lines
                .into_iter()
                .map(|line| {
                    let mut parts = line
                        .split(':')
                        .nth(1)
                        .unwrap()
                        .split('|')
                        .map(|s| {
                            s.trim()
                                .replace("  ", " ")
                                .split(" ")
                                .map(|c| c.parse::<i64>().unwrap())
                                .collect::<Vec<_>>()
                                .into_iter()
                                .rev()
                                .collect()
                        })
                        .collect::<Vec<_>>();

                    (parts.pop().unwrap(), parts.pop().unwrap())
                })
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem4>("inputs/4.txt");
}
