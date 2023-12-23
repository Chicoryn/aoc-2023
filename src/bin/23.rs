use std::{collections::{HashSet, HashMap, BTreeSet}, io::{self, BufRead}};

use aoc_2023::prelude::CooMatrix2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    None,
}

struct Hike {
    tiles: CooMatrix2D<char>,
}

impl Hike {
    fn parse(lines: impl Iterator<Item=String>) -> Self {
        Self {
            tiles: lines
                .enumerate()
                .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, c)| ((row as i32, col as i32), c)).collect::<Vec<_>>())
                .collect(),
        }
    }

    fn starting_point(&self) -> (i32, i32) {
        self.tiles.cols()
            .map(|col| (self.tiles.min().0, col))
            .find(|&pos| self.tiles.get(pos) == Some(&'.'))
            .unwrap()
    }

    fn end_point(&self) -> (i32, i32) {
        self.tiles.cols()
            .map(|col| (self.tiles.max().0 - 1, col))
            .find(|&pos| self.tiles.get(pos) == Some(&'.'))
            .unwrap()
    }

    fn direction(&self, from: (i32, i32), to: (i32, i32)) -> Direction {
        let delta = (to.0 - from.0, to.1 - from.1);
        let tile = self.tiles.get(to).unwrap();

        match (delta, tile) {
            ((-1,  0), '^') => Direction::Down,
            (( 1,  0), '^') => Direction::Up,
            ((-1,  0), 'v') => Direction::Up,
            (( 1,  0), 'v') => Direction::Down,
            (( 0, -1), '<') => Direction::Down,
            (( 0,  1), '<') => Direction::Up,
            (( 0, -1), '>') => Direction::Up,
            (( 0,  1), '>') => Direction::Down,
            _ => Direction::None
        }
    }

    fn neighbours(&self, point: (i32, i32)) -> impl Iterator<Item=(i32, i32)> + '_ {
        let (row, col) = point;

        [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ].into_iter().filter(move |&pos| self.tiles.get(pos).map(|&ch| ch != '#').unwrap_or(false))
    }

    fn is_downhill(&self, from: (i32, i32), to: (i32, i32)) -> bool {
        match self.direction(from, to) {
            Direction::Down | Direction::None => true,
            _ => false,
        }
    }

    fn reachable(
        &self,
        from: (i32, i32),
        is_valid: impl Fn((i32, i32), (i32, i32)) -> bool + Clone,
        visited: &HashSet<(i32, i32)>
    ) -> BTreeSet<(i32, i32)>
    {
        debug_assert!(!visited.contains(&from));

        let mut reachable = BTreeSet::new();
        let mut to_visit = vec! [from];

        while let Some(point) = to_visit.pop() {
            if !visited.contains(&point) && reachable.insert(point) {
                to_visit.extend(self.neighbours(point).filter(|&to| is_valid(point, to)));
            }
        }

        reachable
    }

    fn longest_path_(
        &self,
        starting_point: (i32, i32),
        end_point: (i32, i32),
        is_valid: impl Fn((i32, i32), (i32, i32)) -> bool + Clone,
        mut visited: HashSet<(i32, i32)>,
        so_far: &mut HashMap<((i32, i32), BTreeSet<(i32, i32)>), Option<usize>>,
    ) -> Option<usize>
    {
        let reachable = self.reachable(starting_point, is_valid.clone(), &visited);
        let mut current_point = starting_point;

        if !reachable.contains(&end_point) {
            return None;
        } else if let Some(&max_distance) = so_far.get(&(starting_point, reachable.clone())) {
            return max_distance;
        }

        let mut count = 0;

        loop {
            if current_point == end_point {
                so_far.insert((starting_point, reachable), Some(count));
                return Some(count);
            } else if !visited.insert(current_point) {
                unreachable!("loop invariant");
            }

            let neighbours = self.neighbours(current_point)
                .filter(|&point| !visited.contains(&point))
                .filter(|&point| is_valid(current_point, point))
                .collect::<Vec<_>>();

            if neighbours.len() == 1 {
                current_point = neighbours[0];
                count += 1;
            } else {
                let max_distance = neighbours.into_iter()
                    .filter_map(|point| self.longest_path_(point, end_point, is_valid.clone(), visited.clone(), so_far))
                    .max().map(|max_distance| max_distance + count + 1);

                so_far.insert((starting_point, reachable), max_distance);
                return max_distance;
            }
        }
    }

    fn longest_path(&self, is_valid: impl Fn((i32, i32), (i32, i32)) -> bool + Clone) -> usize {
        self.longest_path_(self.starting_point(), self.end_point(), is_valid, HashSet::new(), &mut HashMap::new())
            .expect("no path found")
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let hike = Hike::parse(lines);

    println!("{}", hike.longest_path(|from, to| hike.is_downhill(from, to)));
    println!("{}", hike.longest_path(|_, _| true)); // not 4838
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 23] = [
        "#.#####################",
        "#.......#########...###",
        "#######.#########.#.###",
        "###.....#.>.>.###.#.###",
        "###v#####.#v#.###.#.###",
        "###.>...#.#.#.....#...#",
        "###v###.#.#.#########.#",
        "###...#.#.#.......#...#",
        "#####.#.#.#######.#.###",
        "#.....#.#.#.......#...#",
        "#.#####.#.#.#########v#",
        "#.#...#...#...###...>.#",
        "#.#.#v#######v###.###v#",
        "#...#.>.#...>.>.#.###.#",
        "#####v#.#.###v#.#.###.#",
        "#.....#...#...#.#.#...#",
        "#.#########.###.#.#.###",
        "#...###...#...#...#.###",
        "###.###.#.###v#####v###",
        "#...#...#.#.>.>.#.>.###",
        "#.###.###.#.###.#.#v###",
        "#.....###...###...#...#",
        "#####################.#",
    ];

    #[test]
    fn _01() {
        let hike = Hike::parse(LINES.iter().map(|s| s.to_string()));

        assert_eq!(hike.longest_path(|from, to| hike.is_downhill(from, to)), 94);
    }

    #[test]
    fn _02() {
        assert_eq!(Hike::parse(LINES.iter().map(|s| s.to_string())).longest_path(|_, _| true), 154);
    }
}
