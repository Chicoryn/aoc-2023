use std::{collections::HashMap, io::{self, BufRead}};

use aoc_2023::prelude::lcm;
use sscanf::sscanf;

#[derive(sscanf::FromScanf)]
#[sscanf(format = "{name} = ({left}, {right})")]
struct Node {
    name: String,
    left: String,
    right: String,
}

struct Network {
    nodes: HashMap<String, Node>,
}

impl Network {
    fn parse(lines: &mut impl Iterator<Item=String>) -> Network {
        Self {
            nodes: lines
                .skip_while(|line| line.is_empty())
                .filter_map(|line| sscanf!(line, "{}", Node).ok())
                .map(|node| (node.name.clone(), node))
                .collect::<HashMap<_, _>>()
        }
    }

    fn nodes(&self) -> impl Iterator<Item=&str> {
        self.nodes.keys().map(String::as_str)
    }

    fn follow(&self, node: &str, direction: char) -> Option<&str> {
        let node = self.nodes.get(node)?;
        let next_node = match direction {
            'L' => &node.left,
            'R' => &node.right,
            _ => unreachable!(),
        };

        Some(next_node)
    }
}

struct Puzzle {
    directions: Vec<char>,
    network: Network,
}

impl Puzzle {
    fn parse(mut lines: impl Iterator<Item=String>) -> Puzzle {
        let directions = lines.next().unwrap().chars().collect::<Vec<_>>();
        let network = Network::parse(&mut lines);

        Self { directions, network }
    }

    fn follow_directions(&self, start_suffix: &str, end_suffix: &str) -> u64 {
        let mut current_nodes = self.network.nodes().filter(|node| node.ends_with(start_suffix)).collect::<Vec<_>>();
        let mut visited_at = vec! [HashMap::<(&str, usize), usize>::new(); current_nodes.len()];
        let mut cycle_length = vec! [0; current_nodes.len()];

        for (step, (direction_index, &direction)) in self.directions.iter().enumerate().cycle().enumerate() {
            if current_nodes.iter().all(|node| node.ends_with(end_suffix)) {
                return step as u64;
            } else if cycle_length.iter().all(|&len| len > 0) {
                return lcm(&cycle_length);
            }

            for (i, node) in current_nodes.iter_mut().enumerate() {
                if node.ends_with(end_suffix) && cycle_length[i] == 0 {
                    if let Some(&at) = visited_at[i].get(&(node, direction_index)) {
                        cycle_length[i] = (step - at) as u64;
                    }

                    visited_at[i].insert((node, direction_index), step);
                }

                *node = self.network.follow(node, direction).unwrap();
            }
        }

        0
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let puzzle = Puzzle::parse(lines.iter().cloned());

    println!("{}", puzzle.follow_directions("AAA", "ZZZ"));
    println!("{}", puzzle.follow_directions("A", "Z"));
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&'static str; 9] = [
        "RL",
        "",
        "AAA = (BBB, CCC)",
        "BBB = (DDD, EEE)",
        "CCC = (ZZZ, GGG)",
        "DDD = (DDD, DDD)",
        "EEE = (EEE, EEE)",
        "GGG = (GGG, GGG)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    const LINES_2: [&'static str; 5] = [
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    const LINES_3: [&'static str; 10] = [
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
    ];

    #[test]
    fn _01() {
        assert_eq!(
            Puzzle::parse(LINES.iter().map(|line| line.to_string())).follow_directions("AAA", "ZZZ"),
            2
        );
        assert_eq!(
            Puzzle::parse(LINES_2.iter().map(|line| line.to_string())).follow_directions("AAA", "ZZZ"),
            6
        );
    }

    #[test]
    fn _02() {
        assert_eq!(
            Puzzle::parse(LINES_3.iter().map(|line| line.to_string())).follow_directions("A", "Z"),
            6
        );
    }
}
