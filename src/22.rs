use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone)]
struct BlockCollection {
    blocks: HashMap<usize, Vec<(usize, usize, usize)>>,
    points: HashMap<(usize, usize, usize), usize>,
}

struct Problem22 {
    collection: BlockCollection,
}

fn parse_block(block: String) -> Vec<(usize, usize, usize)> {
    let ((x1, y1, z1), (x2, y2, z2)) = block
        .split('~')
        .map(|cords| {
            cords
                .split(',')
                .map(|cord| cord.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    (x1.min(x2)..x1.max(x2) + 1)
        .cartesian_product(y1.min(y2)..y1.max(y2) + 1)
        .cartesian_product(z1.min(z2)..z1.max(z2) + 1)
        .map(|((x, y), z)| (x, y, z))
        .collect_vec()
}

fn fits(
    collection: &BlockCollection,
    points: &[(usize, usize, usize)],
    ignore_name: usize,
) -> bool {
    points.iter().all(|point| {
        let belongs_to = collection.points.get(point);

        belongs_to.is_none() || belongs_to.unwrap() == &ignore_name
    })
}

fn get_droppable_block(collection: &BlockCollection) -> Option<usize> {
    for (name, points) in &collection.blocks {
        if points.iter().any(|(_, _, z)| *z == 1) {
            continue;
        }

        let new_points = points.iter().map(|(x, y, z)| (*x, *y, z - 1)).collect_vec();

        if fits(collection, &new_points, *name) {
            return Some(*name);
        }
    }

    None
}

fn drop_block(collection: &mut BlockCollection, name: usize) {
    let existing_points = collection.blocks.get(&name).unwrap().clone();
    for pt in &existing_points {
        collection.points.remove(pt);
    }
    collection.blocks.remove(&name);
    let new_points = existing_points
        .into_iter()
        .map(|(x, y, z)| (x, y, z - 1))
        .collect_vec();
    for pt in new_points.clone() {
        collection.points.insert(pt, name);
    }
    collection.blocks.insert(name, new_points);
}

fn remove_block(collection: BlockCollection, name: usize) -> BlockCollection {
    BlockCollection {
        blocks: collection
            .blocks
            .into_iter()
            .filter(|(other_name, _)| other_name != &name)
            .collect(),
        points: collection
            .points
            .into_iter()
            .filter(|(_, other_name)| other_name != &name)
            .collect(),
    }
}

impl Problem for Problem22 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        let mut collection = self.collection.clone();

        while let Some(name) = get_droppable_block(&collection) {
            drop_block(&mut collection, name);
        }

        let drops = collection
            .blocks
            .keys()
            .map(|candidate| {
                let mut removed = remove_block(collection.clone(), *candidate);

                let mut dropped = HashSet::new();

                while let Some(name) = get_droppable_block(&removed) {
                    drop_block(&mut removed, name);
                    dropped.insert(name);
                }

                dropped.len()
            })
            .collect_vec();

        report_first(&drops.iter().filter(|v| **v == 0).count());
        report_second(&drops.iter().sum::<usize>());
    }

    fn parse(lines: Vec<String>) -> Self {
        let blocks: HashMap<_, _> = lines
            .into_iter()
            .enumerate()
            .map(|(idx, block)| (idx, parse_block(block)))
            .collect();
        Self {
            collection: BlockCollection {
                blocks: blocks.clone(),
                points: blocks
                    .clone()
                    .into_iter()
                    .flat_map(|(name, points)| points.into_iter().map(move |point| (point, name)))
                    .collect(),
            },
        }
    }
}

fn main() {
    run_problem::<Problem22>("inputs/22.txt");
}
