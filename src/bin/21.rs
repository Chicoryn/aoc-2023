use std::{collections::{VecDeque, HashSet}, io::{self, BufRead}};

use aoc_2023::prelude::*;

struct Garden {
    data: CooMatrix2D<char>,
}

impl Garden {
    fn parse(lines: impl Iterator<Item=String>) -> Self {
        Self {
            data: lines.enumerate()
                .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, ch)| ((row as i32, col as i32), ch)).collect::<Vec<_>>())
                .collect()
        }
    }

    fn starting_point(&self) -> (i32, i32) {
        self.data.iter()
            .find_map(|(key, &ch)| if ch == 'S' { Some(key) } else { None })
            .expect("no starting point found")
    }

    fn neighbours(&self, point: (i32, i32)) -> impl Iterator<Item=(i32, i32)> + '_ {
        debug_assert_eq!(self.data.min(), (0, 0));

        [
            (point.0 - 1, point.1),
            (point.0 + 1, point.1),
            (point.0, point.1 - 1),
            (point.0, point.1 + 1)
        ].into_iter()
            .filter(|&neighbour| {
                let new_neighbour = (neighbour.0.rem_euclid(self.data.max().0), neighbour.1.rem_euclid(self.data.max().1));

                match self.data.get(new_neighbour) {
                    Some(&ch) => ch != '#',
                    None => false
                }
            })
    }

    fn reachable_(&self, from: (i32, i32), n: usize) -> usize {
        let mut to_visit = VecDeque::from([ from ]);
        let mut all_neighbours = VecDeque::new();
        let mut visited = HashSet::new();

        for _ in 0..n {
            while let Some(point) = to_visit.pop_front() {
                for neighbour in self.neighbours(point) {
                    if visited.insert(neighbour) {
                        all_neighbours.push_back(neighbour);
                    }
                }
            }

            visited.clear();
            std::mem::swap(&mut to_visit, &mut all_neighbours);
        }

        to_visit.len()
    }

    fn reachable(&self, n: usize) -> usize {
        self.reachable_(self.starting_point(), n)
    }

    fn differences(&self, numbers: &[usize]) -> Vec<Vec<usize>> {
        let mut differences: Vec<Vec<usize>> = vec! [numbers.to_vec()];

        while differences.last().unwrap().iter().any(|&x| x != 0) {
            let last = differences.last().unwrap();
            let next = last.iter().zip(last.iter().skip(1)).map(|(&x, &y)| y - x).collect::<Vec<_>>();

            differences.push(next);
        }

        differences
    }

    fn next_number(&self, numbers: &[usize]) -> usize {
        self.differences(numbers).into_iter()
            .rev()
            .filter_map(|diff| diff.last().copied())
            .sum()
    }

    fn polynomical_reachable(&self, n: usize) -> usize {
        assert_eq!((n - 65) % 131, 0);

        let mut xs = vec! [ self.reachable(1 * 131 + 65) ];
        let mut correct = false;

        while xs.len() < (n - 65) / 131 {
            let next = self.next_number(&xs);

            if correct {
                xs.push(next);
            } else {
                let actual = self.reachable((xs.len() + 1) * 131 + 65);

                correct = actual == next;
                xs.push(actual);
            }
        }

        xs.last().copied().unwrap()
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let garden = Garden::parse(lines.into_iter());

    println!("{}", garden.reachable(64));
    println!("{}", garden.polynomical_reachable(26501365));
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 11] = [
        "...........",
        ".....###.#.",
        ".###.##..#.",
        "..#.#...#..",
        "....#.#....",
        ".##..S####.",
        ".##..#...#.",
        ".......##..",
        ".##.#.####.",
        ".##..##.##.",
        "..........."
    ];

    #[test]
    fn _01() {
        let garden = Garden::parse(LINES.iter().map(|s| s.to_string()));

        assert_eq!(garden.reachable(6), 16);
    }

    #[test]
    #[ignore = "too slow"]
    fn _02() {
        let garden = Garden::parse(LINES.iter().map(|s| s.to_string()));

        assert_eq!(garden.reachable(6), 16);
        assert_eq!(garden.reachable(10), 50);
        assert_eq!(garden.reachable(50), 1594);
        assert_eq!(garden.reachable(100), 6536);
        assert_eq!(garden.reachable(500), 167004);
        assert_eq!(garden.reachable(1000), 668697);
        assert_eq!(garden.reachable(5000), 16733044);
    }
}
