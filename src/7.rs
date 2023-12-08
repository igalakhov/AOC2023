use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::fmt::Display;

struct Problem7 {
    hands_with_wildcards: Vec<(Hand, i64)>,
    hands_no_wildcards: Vec<(Hand, i64)>,
}

#[derive(Debug, Clone)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Into<i64> for HandKind {
    fn into(self) -> i64 {
        match self {
            Self::FiveOfAKind => 6,
            Self::FourOfAKind => 5,
            Self::FullHouse => 4,
            Self::ThreeOfAKind => 3,
            Self::TwoPair => 2,
            Self::OnePair => 1,
            Self::HighCard => 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    kind: HandKind,
    cards: (i64, i64, i64, i64, i64),
}

fn no_wildcards(str: &str) -> Hand {
    let kind = {
        let mut chars = str.to_string().chars().collect::<Vec<_>>();
        chars.sort();
        let fq = chars
            .into_iter()
            .group_by(|x| *x)
            .into_iter()
            .map(|(_, group)| group.collect_vec().len())
            .collect::<Vec<_>>();

        if fq.contains(&5) {
            HandKind::FiveOfAKind
        } else if fq.contains(&4) {
            HandKind::FourOfAKind
        } else if fq.contains(&3) && fq.contains(&2) {
            HandKind::FullHouse
        } else if fq.contains(&3) {
            HandKind::ThreeOfAKind
        } else if fq.iter().filter(|x| **x == 2).count() == 2 {
            HandKind::TwoPair
        } else if fq.contains(&2) {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    };

    let cards = str
        .to_string()
        .chars()
        .map(|c| {
            if c.is_numeric() {
                c.to_digit(10).unwrap() as i64
            } else if c == 'T' {
                10
            } else if c == 'J' {
                11
            } else if c == 'Q' {
                12
            } else if c == 'K' {
                13
            } else {
                15
            }
        })
        .collect_tuple()
        .unwrap();

    Hand { kind, cards }
}

fn with_wildcards(str: &str) -> Hand {
    let kind = {
        let mut chars = str
            .to_string()
            .chars()
            .filter(|c| c != &'J')
            .collect::<Vec<_>>();
        chars.sort();
        let num_wildcards = str.to_string().chars().filter(|c| c == &'J').count();
        let mut fq = chars
            .into_iter()
            .group_by(|x| *x)
            .into_iter()
            .map(|(_, group)| group.collect_vec().len())
            .chain(vec![0])
            .collect::<Vec<_>>();
        let (max_idx, _) = fq.iter().enumerate().max_by_key(|(_, val)| *val).unwrap();
        fq[max_idx] += num_wildcards;

        if fq.contains(&5) {
            HandKind::FiveOfAKind
        } else if fq.contains(&4) {
            HandKind::FourOfAKind
        } else if fq.contains(&3) && fq.contains(&2) {
            HandKind::FullHouse
        } else if fq.contains(&3) {
            HandKind::ThreeOfAKind
        } else if fq.iter().filter(|x| **x == 2).count() == 2 {
            HandKind::TwoPair
        } else if fq.contains(&2) {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    };

    let cards = str
        .to_string()
        .chars()
        .map(|c| {
            if c.is_numeric() {
                c.to_digit(10).unwrap() as i64
            } else if c == 'T' {
                10
            } else if c == 'J' {
                1
            } else if c == 'Q' {
                12
            } else if c == 'K' {
                13
            } else {
                15
            }
        })
        .collect_tuple()
        .unwrap();

    Hand { kind, cards }
}

fn solve_problem(hands: &Vec<(Hand, i64)>) -> i64 {
    let mut hands = hands.clone();
    hands.sort_by_key(|(hand, _)| (Into::<i64>::into(hand.kind.clone()), hand.cards));

    hands
        .iter()
        .enumerate()
        .map(|(num, (_, val))| (num as i64 + 1) * val)
        .sum::<i64>()
}

impl Problem for Problem7 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        report_first(&solve_problem(&self.hands_no_wildcards));
        report_second(&solve_problem(&self.hands_with_wildcards));
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            hands_no_wildcards: lines
                .iter()
                .map(|line| {
                    let parts = line.split(" ").collect::<Vec<_>>();

                    (no_wildcards(parts[0]), parts[1].parse::<i64>().unwrap())
                })
                .collect(),
            hands_with_wildcards: lines
                .iter()
                .map(|line| {
                    let parts = line.split(" ").collect::<Vec<_>>();

                    (with_wildcards(parts[0]), parts[1].parse::<i64>().unwrap())
                })
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem7>("inputs/7.txt");
}
