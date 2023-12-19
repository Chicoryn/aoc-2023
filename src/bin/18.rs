use std::{io::{self, BufRead}, ops::RangeInclusive};

use aoc_2023::prelude::*;
use rayon::iter::{ParallelBridge, ParallelIterator};
use sscanf::scanf;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "U" => Some(Self::Up),
            "D" => Some(Self::Down),
            "L" => Some(Self::Left),
            "R" => Some(Self::Right),
            _ => None,
        }
    }

    fn from_hex(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Right),
            "1" => Some(Self::Down),
            "2" => Some(Self::Left),
            "3" => Some(Self::Up),
            _ => return None,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Up => (-1, 0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DigPlan {
    direction: Direction,
    distance: i32,
    color: String,
}

impl DigPlan {
    fn parse(s: &str) -> Option<Self> {
        let (direction, distance, color) = scanf!(s, "{String} {i32} (#{String})").ok()?;
        let direction = Direction::from_str(&direction)?;

        Some (Self { direction, distance, color })
    }

    fn into_hex_plan(&self) -> Option<Self> {
        let distance = i32::from_str_radix(&self.color[0..5], 16).ok()?;
        let direction = Direction::from_hex(&self.color[5..])?;
        let color = self.color.clone();

        Some(Self { direction, distance, color })
    }

    fn parse_all<'a>(lines: impl Iterator<Item=&'a str>) -> Vec<Self> {
        lines.filter_map(|line| Self::parse(line)).collect()
    }

    fn direction(&self) -> (i32, i32) {
        self.direction.delta()
    }
}

#[derive(Clone, Debug)]
struct Line {
    start_point: (i32, i32),
    end_point: (i32, i32),
    direction: Direction,
}

impl Line {
    fn max(&self) -> (i32, i32) {
        (self.start_point.0.max(self.end_point.0), self.start_point.1.max(self.end_point.1))
    }

    fn min(&self) -> (i32, i32) {
        (self.start_point.0.min(self.end_point.0), self.start_point.1.min(self.end_point.1))
    }
}

struct Trench {
    lines: Vec<Line>,
    min: (i32, i32),
    max: (i32, i32),
}

impl Trench {
    fn dig(plans: impl Iterator<Item=DigPlan>) -> Self {
         let (_, mut lines) = plans
             .fold(
                 ((0, 0), vec! []),
                 move |(start_point, mut lines), plan| {
                    let direction = plan.direction();
                    let distance = plan.distance;
                    let end_point = (start_point.0 + direction.0 * distance, start_point.1 + direction.1 * distance);

                    lines.push(Line { start_point, end_point, direction: plan.direction.clone() });
                    (end_point, lines)
                 }
             );
        lines.sort_unstable_by_key(|line| line.min().1);

        Self {
            lines: lines.clone(),
            min: lines.iter().fold((i32::MAX, i32::MAX), |acc, line| (acc.0.min(line.min().0), acc.1.min(line.min().1))),
            max: lines.iter().fold((i32::MIN, i32::MIN), |acc, line| (acc.0.max(line.max().0), acc.1.max(line.max().1))),
        }
    }

    fn rows(&self) -> RangeInclusive<i32> {
        self.min.0..=self.max.0
    }

    fn volume_in_row(&self, row: i32) -> usize {
        let horizontal_lines = self.lines.iter()
            .filter(|line| line.direction == Direction::Left || line.direction == Direction::Right)
            .filter(|line| row >= line.min().0 && row <= line.max().0)
            .map(|line| line.min().1..line.max().1)
            .collect::<RangeSet<i32>>();
        let vertical_lines: Vec<Line> = self.lines.iter()
            .filter(|line| line.direction == Direction::Up || line.direction == Direction::Down)
            .filter(|line| row >= line.min().0 && row <= line.max().0)
            .fold(
                vec! [],
                |mut acc, line| {
                    if let Some(last_line) = acc.last_mut() {
                        if last_line.direction != line.direction {
                            acc.push(line.clone());
                        }
                    } else {
                        acc.push(line.clone());
                    }

                    acc
                }
            );

        let mut count = 0;
        let mut last_intersect = i32::MIN;

        for (num_intersect, line) in vertical_lines.into_iter().enumerate() {
            if last_intersect < line.min().1 {
                if num_intersect % 2 == 1 {
                    count += (line.min().1 - last_intersect - 1).max(0) as usize;
                }

                count += (line.max().1 - line.min().1 + 1) as usize;
                last_intersect = line.max().1;
            }

            if let Some(horizontal_line) = horizontal_lines.get(&line.min().1) {
                count += horizontal_line.len();
                last_intersect = horizontal_line.end;
            }
        }

        count
    }

    fn volume(&self) -> usize {
        self.rows()
            .par_bridge()
            .map(move |row| self.volume_in_row(row))
            .sum()
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let plan = DigPlan::parse_all(lines.iter().map(|s| s.as_str()));

    println!("{}", Trench::dig(plan.iter().cloned()).volume());
    println!("{}", Trench::dig(plan.iter().filter_map(|plan| plan.into_hex_plan())).volume());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 14] = [
        "R 6 (#70c710)",
        "D 5 (#0dc571)",
        "L 2 (#5713f0)",
        "D 2 (#d2c081)",
        "R 2 (#59c680)",
        "D 2 (#411b91)",
        "L 5 (#8ceee2)",
        "U 2 (#caa173)",
        "L 1 (#1b58a2)",
        "U 2 (#caa171)",
        "R 2 (#7807d2)",
        "U 3 (#a77fa3)",
        "L 2 (#015232)",
        "U 2 (#7a21e3)",
    ];

    #[test]
    fn _01() {
        let plan = DigPlan::parse_all(LINES.iter().copied());
        let trench = Trench::dig(plan.into_iter());

        assert_eq!(trench.volume(), 62);
    }

    #[test]
    fn _02() {
        let plan = DigPlan::parse_all(LINES.iter().copied());
        let trench = Trench::dig(plan.into_iter().filter_map(|plan| plan.into_hex_plan()));

        assert_eq!(trench.volume(), 952408144115);
    }
}
