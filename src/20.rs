use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone, Debug)]
enum ModuleKind {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
}

#[derive(Clone, Debug)]
struct Module {
    kind: ModuleKind,
    outputs: Vec<String>,
}

#[derive(Clone, Debug)]
struct System {
    on_push: Vec<String>,
    modules: HashMap<String, Module>,
    presses: HashMap<String, usize>,
    num_presses: usize,
    low_pulses: usize,
    high_pulses: usize,
}

struct Problem20 {
    system: System,
}

impl System {
    fn push_button(&mut self) {
        self.num_presses += 1;
        self.low_pulses += 1;
        let mut to_process = VecDeque::new();

        self.on_push.iter().for_each(|pushed| {
            to_process.push_front((pushed.to_string(), "broadcaster".to_string(), Pulse::Low))
        });

        while let Some((cur, from, pulse)) = to_process.pop_back() {
            match pulse {
                Pulse::Low => self.low_pulses += 1,
                Pulse::High => {
                    self.presses.insert(from.to_string(), self.num_presses);

                    self.high_pulses += 1
                }
            }

            if !self.modules.contains_key(&cur) {
                continue;
            }

            let cur_module = self.modules.get_mut(&cur).unwrap();

            let pulse = match (&mut cur_module.kind, &pulse) {
                (ModuleKind::FlipFlop(state), Pulse::Low) => {
                    *state = !*state;
                    let pulse = if *state { Pulse::High } else { Pulse::Low };
                    Some(pulse)
                }
                (ModuleKind::Conjunction(inputs), _) => {
                    assert!(inputs.contains_key(&from));
                    inputs.insert(from.to_string(), pulse.clone());
                    let pulse = if inputs
                        .iter()
                        .map(|(_, v)| v)
                        .all(|pulse| pulse == &Pulse::High)
                    {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    Some(pulse)
                }
                _ => None,
            };

            pulse.into_iter().for_each(|pulse| {
                cur_module
                    .outputs
                    .clone()
                    .into_iter()
                    .for_each(|name| to_process.push_front((name, cur.clone(), pulse.clone())))
            });
        }
    }
}

fn lcm(mut m: usize, mut n: usize) -> usize {
    let p = m * n;
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    p / n
}

impl Problem for Problem20 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        let mut first_system = self.system.clone();

        (0..1000).for_each(|_| first_system.push_button());
        report_first(&(first_system.low_pulses * first_system.high_pulses));

        let mut second_system = self.system.clone();

        let targets = self
            .system
            .modules
            .iter()
            .filter(|(_, module)| module.outputs.contains(&"rx".into()))
            .map(|t| t.0)
            .map(|tg| {
                self.system
                    .modules
                    .iter()
                    .filter(|(_, module)| module.outputs.contains(tg))
                    .map(|t| t.0)
                    .collect_vec()
            })
            .nth(0)
            .unwrap();

        while !targets
            .iter()
            .all(|target| second_system.presses.contains_key(*target))
        {
            second_system.push_button();
        }

        report_second(
            &targets
                .into_iter()
                .map(|t| *second_system.presses.get(t).unwrap())
                .fold(1, lcm),
        );
    }

    fn parse(lines: Vec<String>) -> Self {
        let mut modules = HashMap::new();
        let mut on_push = vec![];

        for line in lines {
            let (name, outputs) = line.split(" -> ").collect_tuple().unwrap();
            let outputs = outputs
                .split(", ")
                .map(|output| output.to_string())
                .collect_vec();
            if name == "broadcaster" {
                on_push = outputs;
            } else {
                let kind = if name.as_bytes()[0] == b'%' {
                    ModuleKind::FlipFlop(false)
                } else {
                    ModuleKind::Conjunction(Default::default())
                };
                modules.insert(name[1..].to_string(), Module { kind, outputs });
            }
        }

        let mut transmissions = vec![];

        for push in &on_push {
            transmissions.push(("broadcast".to_string(), push.to_string()));
        }

        modules.iter().for_each(|(name, module)| {
            for output in &module.outputs {
                transmissions.push((name.to_string(), output.to_string()))
            }
        });

        for (from, to) in transmissions {
            if !modules.contains_key(&to) {
                continue;
            }
            if let ModuleKind::Conjunction(inputs) = &mut modules.get_mut(&to).unwrap().kind {
                inputs.insert(from, Pulse::Low);
            }
        }

        Self {
            system: System {
                on_push,
                modules,
                presses: Default::default(),
                num_presses: 0,
                low_pulses: 0,
                high_pulses: 0,
            },
        }
    }
}

fn main() {
    run_problem::<Problem20>("inputs/20.txt");
}
