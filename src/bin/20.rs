use std::{collections::{VecDeque, HashMap}, io::{self, BufRead}};

use sscanf::scanf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop { on: bool },
    Conjunction { memory: HashMap<String, Pulse> },
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    module_type: ModuleType,
    destination: Vec<String>,
}

impl Module {
    fn parse(s: &str) -> Option<Self> {
        if let Ok(destination) = scanf!(s, "broadcaster -> {String}") {
            Some(Self {
                name: "broadcaster".to_string(),
                module_type: ModuleType::Broadcaster,
                destination: destination.split(", ").map(|s| s.trim().to_string()).collect(),
            })
        } else if let Ok((name, destination)) = scanf!(s, "%{String} -> {String}") {
            Some(Self {
                name,
                module_type: ModuleType::FlipFlop { on: false },
                destination: destination.split(", ").map(|s| s.trim().to_string()).collect(),
            })
        } else if let Ok((name, destination)) = scanf!(s, "&{String} -> {String}") {
            Some(Self {
                name,
                module_type: ModuleType::Conjunction { memory: HashMap::new() },
                destination: destination.split(", ").map(|s| s.trim().to_string()).collect(),
            })
        } else {
            None
        }
    }

    fn parse_all(lines: impl Iterator<Item=String>) -> HashMap<String, Self> {
        connect(lines.filter_map(|s| Self::parse(&s)).map(|m| (m.name.clone(), m)).collect())
    }

    fn add_input(&mut self, name: String) {
        match &mut self.module_type {
            ModuleType::Conjunction { memory } => {
                memory.entry(name).or_insert(Pulse::Low);
            },
            _ => { /* pass */}
        }
    }

    fn receive(&mut self, from: String, signal: Pulse) -> Option<Pulse> {
        match &mut self.module_type {
            ModuleType::Broadcaster => { Some(signal) }
            ModuleType::FlipFlop { on: _ } if signal == Pulse::High => { None }
            ModuleType::FlipFlop { on } if *on => { *on = false; Some(Pulse::Low) },
            ModuleType::FlipFlop { on } if !*on => { *on = true; Some(Pulse::High) },
            ModuleType::Conjunction { memory } => {
                memory.insert(from, signal);
                if memory.values().all(|&s| s == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            },
            _ => { unreachable!("unregonized module type and / or configuration: {:?}", self) }
        }
    }
}

fn connect(mut modules: HashMap<String, Module>) -> HashMap<String, Module> {
    let module_names = modules.values().map(|module| module.name.clone()).collect::<Vec<_>>();

    for name in module_names.into_iter() {
        let destinations = modules.get(&name).unwrap().destination.clone();

        for destination in &destinations {
            if let Some(module) = modules.get_mut(destination) {
                module.add_input(name.clone());
            }
        }
    }

    modules
}

fn process(modules: &mut HashMap<String, Module>, signal: Pulse, terminate_at: impl Fn(&str, Pulse) -> bool) -> Option<(usize, usize)> {
    let mut remaining = VecDeque::from([("".to_string(), "broadcaster".to_string(), signal)]);
    let mut count = (0, 0);

    while let Some((from, to, signal)) = remaining.pop_front() {
        if terminate_at(&from, signal) {
            return None;
        }

        if signal == Pulse::High {
            count.1 += 1;
        } else {
            count.0 += 1;
        }

        if let Some((output, destinations)) = modules.get_mut(&to).and_then(|module| module.receive(from, signal).map(|output| (output, &module.destination))) {
            for destination in destinations {
                remaining.push_back((to.clone(), destination.clone(), output));
            }
        }
    }

    Some(count)
}

fn process_n(modules: &mut HashMap<String, Module>, signal: Pulse, n: usize) -> usize {
    let count = (0..n).fold((0, 0), |acc, _| {
        let result = process(modules, signal, |_, _| false).unwrap();

        (acc.0 + result.0, acc.1 + result.1)
    });

    count.0 * count.1
}

fn dependencies(modules: &HashMap<String, Module>, module: &str) -> Vec<String> {
    let mut remaining = vec! [module.to_string()];

    while remaining.len() == 1 {
        let name = remaining.pop().unwrap();

        remaining.extend(modules.values().filter_map(|module| {
            if module.destination.contains(&name) {
                Some(module.name.clone())
            } else {
                None
            }
        }));
    }

    remaining
}

fn process_until_terminate(modules: &mut HashMap<String, Module>, signal: Pulse, terminate_at: impl Fn(&str, Pulse) -> bool) -> usize {
    (1..).filter(move |_| process(modules, signal, |to, signal| terminate_at(to, signal)).is_none()).next().unwrap()
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let modules = Module::parse_all(lines.into_iter());

    println!("{}", process_n(&mut modules.clone(), Pulse::Low, 1000));
    println!("{}", dependencies(&modules, "rx").into_iter().map(|dep| process_until_terminate(&mut modules.clone(), Pulse::Low, |to, signal| to == dep && signal == Pulse::High)).product::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES_1: [&str; 5] = [
        "broadcaster -> a, b, c",
        "%a -> b",
        "%b -> c",
        "%c -> inv",
        "&inv -> a",
    ];

    const LINES_2: [&str; 5] = [
        "broadcaster -> a",
        "%a -> inv, con",
        "&inv -> b",
        "%b -> con",
        "&con -> output",
    ];

    #[test]
    fn _01_1() {
        let modules = Module::parse_all(LINES_1.iter().map(|s| s.to_string()));

        assert_eq!(process(&mut modules.clone(), Pulse::Low, |_, _| false), Some((8, 4)));
        assert_eq!(process_n(&mut modules.clone(), Pulse::Low, 1000), 32000000);
    }

    #[test]
    fn _01_2() {
        let mut modules = Module::parse_all(LINES_2.iter().map(|s| s.to_string()));

        assert_eq!(process_n(&mut modules, Pulse::Low, 1000), 11687500);
    }
}
