use std::{io::{self, BufRead}, str::FromStr};

use rand::prelude::*;

struct Component {
    name: String,
    connected_to: Vec<String>,
}

impl FromStr for Component {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");

        let name = parts.next().ok_or(())?.trim().to_string();
        let connected_to = parts.next().ok_or(())?.split_whitespace().map(|s| s.to_string()).collect();

        Ok(Component {
            name,
            connected_to,
        })
    }
}

impl Component {
    fn edges(&self) -> Vec<(String, String)> {
        self.connected_to.iter()
            .map(move |other| (self.name.clone(), other.clone()))
            .collect()
    }
}

fn contract(mut vertices: Vec<String>, mut edges: Vec<(String, String)>) -> Option<usize> {
    while vertices.len() > 2 {
        let edge = edges.choose(&mut thread_rng()).unwrap().clone();
        let new_vertex = format!("{}-{}", edge.0, edge.1);

        edges.retain_mut(|(a, b)| {
            if *a == edge.0 || *a == edge.1 {
                *a = new_vertex.clone();
            }

            if *b == edge.0 || *b == edge.1 {
                *b = new_vertex.clone();
            }

            *a != *b
        });

        vertices.retain(|v| *v != edge.0 && *v != edge.1);
        vertices.push(new_vertex);
    }

    if edges.len() == 3 {
        let a_size = vertices[0].chars().filter(|ch| *ch == '-').count() + 1;
        let b_size = vertices[1].chars().filter(|ch| *ch == '-').count() + 1;

        Some(a_size * b_size)
    } else {
        None
    }
}

fn min_cut(edges: &[(String, String)]) -> usize {
    let mut nodes = edges.iter().flat_map(|(a, b)| vec![a.clone(), b.clone()]).collect::<Vec<_>>();
    nodes.sort_unstable();
    nodes.dedup();

    loop {
        if let Some(size) = contract(nodes.clone(), edges.to_vec()) {
            return size;
        }
    }
}

fn main() {
    let lines = io::stdin().lock().lines().filter_map(Result::ok).collect::<Vec<_>>();
    let edges = lines.into_iter().filter_map(|line| line.parse::<Component>().ok()).flat_map(|c| c.edges()).collect::<Vec<_>>();

    println!("{}", min_cut(&edges));
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 13] = [
        "jqt: rhn xhk nvd",
        "rsh: frs pzl lsr",
        "xhk: hfx",
        "cmg: qnr nvd lhk bvb",
        "rhn: xhk bvb hfx",
        "bvb: xhk hfx",
        "pzl: lsr hfx nvd",
        "qnr: nvd",
        "ntq: jqt hfx bvb xhk",
        "nvd: lhk",
        "lsr: lhk",
        "rzs: qnr cmg lsr rsh",
        "frs: qnr lhk lsr",
    ];

    #[test]
    fn _01() {
        let edges = LINES.iter().filter_map(|line| line.parse::<Component>().ok()).flat_map(|c| c.edges()).collect::<Vec<_>>();

        assert_eq!(min_cut(&edges), 54);
    }
}
