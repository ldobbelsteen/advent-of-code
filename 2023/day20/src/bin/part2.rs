#![warn(clippy::pedantic)]

use num::integer::lcm;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Debug, PartialEq)]
enum ModuleKind<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>), // boolean indicates is_high for input
}

#[derive(Debug)]
struct Pulse<'a> {
    src: &'a str,
    is_low: bool,
    dest: &'a str,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    kind: ModuleKind<'a>,
    destinations: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn from_line(s: &'a str) -> Self {
        if s.contains('%') {
            let re = Regex::new(r"%(.+) -> (.+)").unwrap();
            let captures = re.captures(s).unwrap();
            let name = captures.get(1).unwrap().as_str();
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
            let name = captures.get(1).unwrap().as_str();
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
            let name = captures.get(1).unwrap().as_str();
            let destinations = captures.get(2).unwrap().as_str().split(", ").collect();
            let kind = ModuleKind::Broadcast;
            Self {
                name,
                kind,
                destinations,
            }
        }
    }

    fn handle_pulse<F>(&mut self, pulse: &Pulse<'a>, mut output_new_pulse: F)
    where
        F: FnMut(Pulse<'a>),
    {
        match &mut self.kind {
            ModuleKind::Broadcast => {
                let is_low = pulse.is_low;
                for dest in &self.destinations {
                    output_new_pulse(Pulse {
                        src: self.name,
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
                            src: self.name,
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
                        src: self.name,
                        is_low,
                        dest,
                    });
                }
            }
        }
    }
}

fn lcm_of_slice(ns: &[u64]) -> u64 {
    let mut iter = ns.iter();
    let mut result = *iter.next().unwrap();
    for next in iter {
        result = lcm(result, *next);
    }
    result
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut modules: HashMap<&str, Module> = file
        .lines()
        .map(Module::from_line)
        .map(|m| (m.name, m))
        .collect();

    // Get the inputs for each of the modules.
    let mut inputs: HashMap<&str, HashSet<&str>> = HashMap::new();
    for module in modules.values() {
        for dest in &module.destinations {
            inputs.entry(*dest).or_default().insert(module.name);
        }
    }

    // Set low-pulse memory for all inputs of all conjunction modules.
    for module in modules.values_mut() {
        if let ModuleKind::Conjunction(states) = &mut module.kind {
            for input in inputs.get(&module.name).unwrap() {
                states.insert(input, false);
            }
        }
    }

    let target_name = "rx";
    let sources_names = inputs.get(target_name).unwrap();
    assert!(sources_names.len() == 1);

    // The only incoming module to the target, and it should be a conjunction.
    let subtarget_name = sources_names.iter().next().unwrap();
    let subtarget = modules.get(subtarget_name).unwrap();

    // Get the modules incoming to the subtarget and watch them to keep track
    // of the periodicities of high pulses coming from them. Each of the watched
    // modules is mapped to the last high pulse time and the last high pulse interval.
    // If two subsequent intervals are the same, we assume that is the period.
    let mut watched: HashMap<&str, (Option<u64>, Option<u64>)> = HashMap::new();
    let mut periods: HashMap<&str, u64> = HashMap::new();
    if let ModuleKind::Conjunction(inputs) = &subtarget.kind {
        for w in inputs.keys() {
            watched.insert(w, (None, None));
        }
    } else {
        panic!("only target source is not a conjunction")
    }

    let mut button_presses = 0;
    let mut queue: VecDeque<Pulse> = VecDeque::new();
    while periods.len() < watched.len() {
        if let Some(pulse) = queue.pop_front() {
            // High pulse to subtarget detected. Handle finding the periods.
            if pulse.dest == *subtarget_name && !pulse.is_low {
                for (watch, (last_high, last_interval)) in &mut watched {
                    if pulse.src == *watch {
                        if let Some(last_high) = last_high {
                            let current_interval = button_presses - *last_high;
                            if let Some(last_interval) = last_interval {
                                if *last_interval == current_interval {
                                    if let Some(prev) = periods.insert(watch, current_interval) {
                                        assert!(prev == current_interval);
                                    }
                                }
                            }
                            *last_interval = Some(current_interval);
                        }
                        *last_high = Some(button_presses);
                    }
                }
            }

            if let Some(module) = modules.get_mut(pulse.dest) {
                module.handle_pulse(&pulse, |p| queue.push_back(p));
            }
        } else {
            button_presses += 1;
            queue.push_back(Pulse {
                src: "button",
                is_low: true,
                dest: "broadcaster",
            });
        }
    }

    println!(
        "{}",
        lcm_of_slice(&periods.values().copied().collect::<Vec<u64>>())
    );
}
