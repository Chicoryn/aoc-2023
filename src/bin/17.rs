use std::{collections::{HashMap, VecDeque}, ops::RangeInclusive, io::{self, BufRead}};

struct Map {
    grid: HashMap<(i32, i32), i32>,
    max: (i32, i32),
}

impl Map {
    fn parse<'a>(lines: impl Iterator<Item=&'a str>) -> Self {
        let grid = lines.enumerate()
            .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, ch)| ((row as i32, col as i32), ch)))
            .map(|(point, ch)| (point, ch.to_digit(10).unwrap() as i32))
            .collect::<HashMap<_, _>>();

        Self {
            grid: grid.clone(),
            max: grid.iter().fold((0, 0), |acc, ((row, col), _)| (acc.0.max(*row + 1), acc.1.max(*col + 1))),
        }
    }

    fn min_heat(&self, valid_distances: RangeInclusive<i32>) -> i32 {
        let mut remaining = VecDeque::from([ ((0, 0), (-1, 0), 0), ((0, 0), (0, -1), 0) ]);
        let mut best_so_far = HashMap::new();

        while let Some((point, prev_direction, heat_loss)) = remaining.pop_front() {
            if point.0 < 0 || point.1 < 0 || point.0 >= self.max.0 || point.1 >= self.max.1 {
                continue;
            } else if best_so_far.get(&(point, prev_direction)).copied().unwrap_or(i32::MAX) <= heat_loss {
                continue;
            } else {
                best_so_far.insert((point, prev_direction), heat_loss);
            }

            for distance in valid_distances.clone().into_iter() {
                for &direction in &[(-prev_direction.1, 0), (prev_direction.1, 0), (0, -prev_direction.0), (0, prev_direction.0)] {
                    if direction == (0, 0) {
                        continue;
                    }

                    let next_point = (point.0 + distance * direction.0, point.1 + distance * direction.1);
                    let additional_heat_loss = (1..=distance)
                        .map(|d| self.grid.get(&(point.0 + d * direction.0, point.1 + d * direction.1)).copied().unwrap_or(0))
                        .sum::<i32>();

                    remaining.push_back((next_point, direction, heat_loss + additional_heat_loss));
                }
            }
        }

        best_so_far.iter()
            .filter_map(|(&(point, _), &heat_loss)| {
                if point == (self.max.0 - 1, self.max.1 - 1) {
                    Some(heat_loss)
                } else {
                    None
                }
            })
            .min()
            .expect("machine parts factory is unreachable!")
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(Result::ok).collect::<Vec<_>>();
    let map = Map::parse(lines.iter().map(|s| s.as_str()));

    println!("{}", map.min_heat(1..=3));
    println!("{}", map.min_heat(4..=10));
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 13] = [
        "2413432311323",
        "3215453535623",
        "3255245654254",
        "3446585845452",
        "4546657867536",
        "1438598798454",
        "4457876987766",
        "3637877979653",
        "4654967986887",
        "4564679986453",
        "1224686865563",
        "2546548887735",
        "4322674655533",
    ];

    const LINES_2: [&str; 5] = [
        "111111111111",
        "999999999991",
        "999999999991",
        "999999999991",
        "999999999991",
    ];

    #[test]
    fn _01() {
        let map = Map::parse(LINES.iter().copied());

        assert_eq!(map.min_heat(1..=3), 102);
    }

    #[test]
    fn _02() {
        let map = Map::parse(LINES.iter().copied());

        assert_eq!(map.min_heat(4..=10), 94);
    }

    #[test]
    fn _02_another_example() {
        let map = Map::parse(LINES_2.iter().copied());

        assert_eq!(map.min_heat(4..=10), 71);
    }
}
