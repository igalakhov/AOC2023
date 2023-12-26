use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::fmt::Display;

struct Problem18 {
    short_instructions: Vec<Instruction>,
    long_instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    dir: Dir,
    len: isize,
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn parse_line(value: &str) -> (Instruction, Instruction) {
    let (dir, len, col) = value.split(' ').collect_tuple().unwrap();

    (
        Instruction {
            dir: dir.into(),
            len: len.parse::<isize>().unwrap(),
        },
        Instruction {
            dir: match col.as_bytes()[col.len() - 2] {
                b'0' => Dir::Right,
                b'1' => Dir::Down,
                b'2' => Dir::Left,
                b'3' => Dir::Up,
                _ => panic!(),
            },
            len: isize::from_str_radix(&col[2..col.len() - 2], 16).unwrap(),
        },
    )
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => unimplemented!(),
        }
    }
}

fn get_area(instructions: &Vec<Instruction>) -> isize {
    let (mut i, mut j) = (0, 0);

    let mut points = vec![];

    for ins in instructions {
        match ins.dir {
            Dir::Right => j += ins.len,
            Dir::Left => j -= ins.len,
            Dir::Up => i -= ins.len,
            Dir::Down => i += ins.len,
        }
        points.push((i, j));
    }

    points
        .iter()
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - y1 * x2)
        .sum::<isize>()
        .abs()
        / 2
        + instructions.iter().map(|ins| ins.len).sum::<isize>() / 2
        + 1
}

impl Problem for Problem18 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        report_first(&get_area(&self.short_instructions));
        report_second(&get_area(&self.long_instructions));
    }

    fn parse(lines: Vec<String>) -> Self {
        let (short_instructions, long_instructions) =
            lines.iter().map(|line| parse_line(line)).unzip();
        Self {
            short_instructions,
            long_instructions,
        }
    }
}

fn main() {
    run_problem::<Problem18>("inputs/18.txt");
}
