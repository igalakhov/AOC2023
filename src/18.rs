use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

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

fn parse_line(value: &String) -> (Instruction, Instruction) {
    println!("{value}");
    let (dir, len, col) = value.split(" ").collect_tuple().unwrap();

    (
        Instruction {
            dir: dir.into(),
            len: len.parse::<isize>().unwrap(),
        },
        Instruction {
            dir: dir.into(),
            len: len.parse::<isize>().unwrap(),
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
    let mut lines: HashMap<isize, Vec<(isize, isize)>> = Default::default();
    let (mut i, mut j) = (0, 0);

    let mut points = vec![];

    for ins in instructions {
        match ins.dir {
            Dir::Right => {
                lines.entry(i).or_default().push((j, j + ins.len));
                j += ins.len;
            }
            Dir::Left => {
                lines.entry(i).or_default().push((j - ins.len, j));
                j -= ins.len;
            }
            Dir::Up => {
                (i - ins.len + 1..i).for_each(|i| {
                    lines.entry(i).or_default().push((j, j));
                });

                i -= ins.len;
            }
            Dir::Down => {
                (i + 1..i + ins.len).for_each(|i| {
                    lines.entry(i).or_default().push((j, j));
                });
                i += ins.len;
            }
        }
        points.push((i, j));
    }

    let area = points
        .iter()
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - y1 * x2)
        .sum::<isize>()
        .abs()
        / 2;

    let on_perimiter = instructions.iter().map(|ins| ins.len - 1).sum::<isize>();

    let inside = (area + 1) - on_perimiter / 2;

    println!("{area} {on_perimiter} {inside}");

    inside * 2

    // println!("{:?}", lines[&0]);
    //
    // lines
    //     .iter_mut()
    //     .map(|(i, ints)| {
    //         ints.sort_by_key(|(l, _)| *l);
    //
    //         let ret = ints.iter().map(|(l, r)| r - l + 1).sum::<isize>()
    //             + ints
    //                 .iter()
    //                 .tuples()
    //                 .map(|((_, r1), (l1, _))| l1 - r1 - 1)
    //                 .sum::<isize>();
    //
    //         println!("{i} {ints:?} {ret}");
    //         ret
    //     })
    //     .sum::<isize>()
}

impl Problem for Problem18 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
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
