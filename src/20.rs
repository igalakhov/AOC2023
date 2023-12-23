use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
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
    presses: HashMap<String, Option<usize>>,
    num_presses: usize,
    low_pulses: usize,
    high_pulses: usize,
}

struct Problem20 {
    system: System,
}

impl System {
    fn propagate(&mut self, cur: &str, from: &str, pulse: Pulse) {
        match pulse {
            Pulse::Low => self.low_pulses += 1,
            Pulse::High => self.high_pulses += 1,
        }

        if !self.modules.contains_key(cur) {
            return;
        }

        let cur_module = self.modules.get_mut(cur).unwrap();

        let pulse = match (&mut cur_module.kind, &pulse) {
            (ModuleKind::FlipFlop(state), Pulse::Low) => {
                *state = !*state;
                let pulse = if *state { Pulse::High } else { Pulse::Low };
                Some(pulse)
            }
            (ModuleKind::Conjunction(inputs), _) => {
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

        if let Some(pulse) = pulse {
            if pulse == Pulse::High {
                self.presses.insert(cur.to_string(), Some(self.num_presses));
            }
            for name in cur_module.outputs.clone() {
                self.propagate(&name, cur, pulse.clone());
            }
        }
    }

    fn push_button(&mut self) {
        self.num_presses += 1;
        for pushed in self.on_push.clone() {
            self.propagate(&pushed, "broadcast", Pulse::Low);
        }
        self.low_pulses += 1;
    }
}

impl Problem for Problem20 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        let mut high_pulses = 0;
        let mut low_pulses = 0;

        let mut first_system = self.system.clone();

        for _ in 0..1000 {
            first_system.push_button();
        }
        report_first(&(first_system.low_pulses * first_system.high_pulses));

        let mut second_system = self.system.clone();
        let targets = vec!["lk", "zv", "sp", "xt"];
        let mut periods = HashMap::new();

        let mut last_press = HashMap::new();

        while !targets.iter().all(|target| periods.contains_key(*target)) {
            second_system.push_button();
            for t in &targets {
                let val = second_system.presses.get(*t).and_then(|val| *val);

                if val.is_some() {
                    if last_press.contains_key(*t) {
                        let pd = val.unwrap() - last_press.get(*t).unwrap();
                        if pd != 0 {
                            periods.insert(t.to_string(), pd);
                        }
                    }

                    last_press.insert(t.to_string(), val.unwrap());
                }
            }
        }

        println!("{periods:?}");

        let mut cycle = 1;
        for (_, v) in periods {
            cycle = num::integer::lcm(cycle, v);
        }
        report_second(&cycle);
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
            match &mut modules.get_mut(&to).unwrap().kind {
                ModuleKind::Conjunction(inputs) => {
                    inputs.insert(from, Pulse::Low);
                }
                _ => (),
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
