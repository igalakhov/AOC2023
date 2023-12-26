use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

struct Problem19 {
    workflows: HashMap<String, Workflow>,
    items: Vec<HashMap<String, i64>>,
}

#[derive(Debug)]
struct Workflow {
    steps: Vec<Step>,
}

#[derive(Debug)]
struct Step {
    kind: StepKind,
    jump_to: String,
}

#[derive(Debug)]
enum StepKind {
    Unconditional,
    LessThan(String, i64),
    GreaterThan(String, i64),
}

fn parse_step(s: &str) -> Step {
    if !s.contains(':') {
        return Step {
            kind: StepKind::Unconditional,
            jump_to: s.to_string(),
        };
    }

    let (cmp, jump_to) = s.split(':').collect_tuple().unwrap();

    let mut kind = None;

    if cmp.contains('<') {
        let (name, val) = cmp.split('<').collect_tuple().unwrap();

        kind = Some(StepKind::LessThan(name.to_string(), val.parse().unwrap()))
    }

    if s.contains('>') {
        let (name, val) = cmp.split('>').collect_tuple().unwrap();

        kind = Some(StepKind::GreaterThan(
            name.to_string(),
            val.parse().unwrap(),
        ))
    }

    Step {
        kind: kind.unwrap(),
        jump_to: jump_to.to_string(),
    }
}

fn parse_item(s: String) -> HashMap<String, i64> {
    s[1..s.len() - 1]
        .split(',')
        .map(|v| {
            let (name, val) = v.split('=').collect_tuple().unwrap();

            (name.to_string(), val.parse().unwrap())
        })
        .collect()
}

fn parse_workflow(s: String) -> (String, Workflow) {
    let (name, steps) = s.split('{').collect_tuple().unwrap();
    let steps = steps[..steps.len() - 1]
        .to_string()
        .split(',')
        .map(parse_step)
        .collect();

    (name.to_string(), Workflow { steps })
}

impl Problem19 {
    fn run_item(&self, item: &HashMap<String, i64>, name: &str) -> bool {
        if name == "A" {
            return true;
        }
        if name == "R" {
            return false;
        }
        for step in &self.workflows[name].steps {
            match &step.kind {
                StepKind::Unconditional => {
                    return self.run_item(item, &step.jump_to);
                }
                StepKind::LessThan(var, val) => {
                    if &item[var] < val {
                        return self.run_item(item, &step.jump_to);
                    }
                }
                StepKind::GreaterThan(var, val) => {
                    if &item[var] > val {
                        return self.run_item(item, &step.jump_to);
                    }
                }
            }
        }
        unreachable!();
    }

    fn run_item_multi(
        &self,
        mut item: HashMap<String, (i64, i64)>,
        name: &str,
    ) -> Vec<HashMap<String, (i64, i64)>> {
        if name == "A" {
            return vec![item];
        }
        if name == "R" {
            return vec![];
        }
        let mut ret = vec![];

        for step in &self.workflows[name].steps {
            match &step.kind {
                StepKind::Unconditional => {
                    ret.extend(self.run_item_multi(item.clone(), &step.jump_to));
                }
                StepKind::LessThan(var, val) => {
                    let (cl, ch) = item[var];
                    item.insert(var.clone(), (cl, val - 1));
                    ret.extend(self.run_item_multi(item.clone(), &step.jump_to));
                    item.insert(var.clone(), (*val, ch));
                }
                StepKind::GreaterThan(var, val) => {
                    let (cl, ch) = item[var];
                    item.insert(var.clone(), (val + 1, ch));
                    ret.extend(self.run_item_multi(item.clone(), &step.jump_to));
                    item.insert(var.clone(), (cl, *val));
                }
            }
        }
        ret
    }
}

impl Problem for Problem19 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        report_first(
            &self
                .items
                .iter()
                .filter(|item| self.run_item(item, "in"))
                .map(|item| item.iter().map(|(_, v)| v).sum::<i64>())
                .sum::<i64>(),
        );

        let mega_item = vec![
            ("x", (1, 4000)),
            ("m", (1, 4000)),
            ("a", (1, 4000)),
            ("s", (1, 4000)),
        ]
        .into_iter()
        .map(|(name, interval)| (name.to_string(), interval))
        .collect();

        report_second(
            &self
                .run_item_multi(mega_item, "in")
                .iter()
                .map(|item| {
                    item.iter()
                        .map(|(_, (l, r))| (r - l + 1).max(0))
                        .product::<i64>()
                })
                .sum::<i64>(),
        )
    }

    fn parse(lines: Vec<String>) -> Self {
        let mut workflows = vec![];
        let mut items = vec![];
        let mut is_part = false;
        for line in lines {
            if line.is_empty() {
                is_part = true;
                continue;
            }

            if is_part {
                items.push(parse_item(line));
            } else {
                workflows.push(parse_workflow(line));
            }
        }
        Self {
            workflows: workflows.into_iter().collect(),
            items,
        }
    }
}

fn main() {
    run_problem::<Problem19>("inputs/19.txt");
}
