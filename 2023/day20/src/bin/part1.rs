#![warn(clippy::pedantic)]

use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Debug)]
enum ModuleKind {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>), // boolean indicates is_high for input
}

#[derive(Debug)]
struct Pulse<'a> {
    src: String,
    is_low: bool,
    dest: &'a str,
}

#[derive(Debug)]
struct Module<'a> {
    name: String,
    kind: ModuleKind,
    destinations: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn from_line(s: &'a str) -> Self {
        if s.contains('%') {
            let re = Regex::new(r"%(.+) -> (.+)").unwrap();
            let captures = re.captures(s).unwrap();
            let name = captures.get(1).unwrap().as_str().to_string();
            let kind = ModuleKind::FlipFlop(false);
            let destinations = captures.get(2).unwrap().as_str().split(", ").collect();
            Self {
                name,
                kind,
                destinations,
            }
        } else if s.contains('&') {
            let re = Regex::new(r"&(.+) -> (.+)").unwrap();
            let captures = re.captures(s).unwrap();
            let name = captures.get(1).unwrap().as_str().to_string();
            let destinations: Vec<&str> = captures.get(2).unwrap().as_str().split(", ").collect();
            let kind = ModuleKind::Conjunction(HashMap::new());
            Self {
                name,
                kind,
                destinations,
            }
        } else {
            let re = Regex::new(r"(.+) -> (.+)").unwrap();
            let captures = re.captures(s).unwrap();
            let name = captures.get(1).unwrap().as_str().to_string();
            let destinations = captures.get(2).unwrap().as_str().split(", ").collect();
            let kind = ModuleKind::Broadcast;
            Self {
                name,
                kind,
                destinations,
            }
        }
    }

    fn handle_pulse<F>(&mut self, pulse: Pulse, mut output_new_pulse: F)
    where
        F: FnMut(Pulse<'a>),
    {
        match &mut self.kind {
            ModuleKind::Broadcast => {
                let is_low = pulse.is_low;
                for dest in &self.destinations {
                    output_new_pulse(Pulse {
                        src: self.name.to_string(),
                        is_low,
                        dest,
                    });
                }
            }
            ModuleKind::FlipFlop(state) => {
                if pulse.is_low {
                    *state = !*state;
                    let is_low = !*state;
                    for dest in &self.destinations {
                        output_new_pulse(Pulse {
                            src: self.name.to_string(),
                            is_low,
                            dest,
                        });
                    }
                }
            }
            ModuleKind::Conjunction(inputs) => {
                inputs.insert(pulse.src, !pulse.is_low);
                let is_low = inputs.values().all(|is_high| *is_high);
                for dest in &self.destinations {
                    output_new_pulse(Pulse {
                        src: self.name.to_string(),
                        is_low,
                        dest,
                    });
                }
            }
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut modules: HashMap<String, Module> = file
        .lines()
        .map(Module::from_line)
        .map(|m| (m.name.clone(), m))
        .collect();

    // Get the inputs for each of the modules.
    let mut inputs: HashMap<String, HashSet<String>> = HashMap::new();
    for module in modules.values() {
        for dest in &module.destinations {
            inputs
                .entry((*dest).to_string())
                .or_default()
                .insert(module.name.to_string());
        }
    }

    // Set low-pulse memory for all inputs of all conjunction modules.
    for module in modules.values_mut() {
        if let ModuleKind::Conjunction(states) = &mut module.kind {
            for input in inputs.get(&module.name).unwrap() {
                states.insert(input.to_string(), false);
            }
        }
    }

    let mut low_count = 0;
    let mut high_count = 0;
    let mut button_presses = 1000;
    let mut queue: VecDeque<Pulse> = VecDeque::new();
    loop {
        if let Some(pulse) = queue.pop_front() {
            if let Some(module) = modules.get_mut(pulse.dest) {
                module.handle_pulse(pulse, |p| {
                    if p.is_low {
                        low_count += 1;
                    } else {
                        high_count += 1;
                    }
                    queue.push_back(p);
                });
            }
        } else if button_presses > 0 {
            button_presses -= 1;
            queue.push_back(Pulse {
                src: "button".into(),
                is_low: true,
                dest: "broadcaster",
            });
            low_count += 1;
        } else {
            break;
        }
    }

    let result = low_count * high_count;
    println!("{result}");
}
