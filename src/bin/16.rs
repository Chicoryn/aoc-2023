use std::{io::{self, BufRead}, collections::{HashMap, HashSet}};

struct Contraption {
    grid: HashMap<(i32, i32), char>,
    max: (i32, i32),
}

impl Contraption {
    fn parse<'a>(lines: impl Iterator<Item=&'a str>) -> Self {
        let grid = lines.enumerate()
            .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, ch)| ((row as i32, col as i32), ch)))
            .filter(|(_, ch)| *ch != '.')
            .collect::<HashMap<_, _>>();

        Self {
            grid: grid.clone(),
            max: grid.iter().fold((0, 0), |acc, ((row, col), _)| (acc.0.max(*row + 1), acc.1.max(*col + 1))),
        }
    }

    fn best_energized(&self) -> usize {
        let rows = (0..self.max.0).flat_map(|row| [ ((row, 0), (0, 1)), ((row, self.max.1 - 1), (0, -1)) ]);
        let cols = (0..self.max.1).flat_map(|col| [ ((0, col), (1, 0)), ((self.max.0 - 1, col), (-1, 0)) ]);

        rows.chain(cols)
            .map(|(point, direction)| self.energized(point, direction))
            .max()
            .unwrap_or(0)
    }

    fn energized(&self, starting_point: (i32, i32), direction: (i32, i32)) -> usize {
        let mut energized = HashMap::new();
        let mut visited = HashSet::new();
        let mut remaining = vec! [ (starting_point, direction) ];

        while let Some((point, direction)) = remaining.pop() {
            if point.0 < 0 || point.1 < 0 {
                continue;
            } else if point.0 >= self.max.0 || point.1 >= self.max.1 {
                continue;
            } else if visited.contains(&(point, direction)) {
                continue;
            }

            *energized.entry(point).or_insert(0) += 1;
            visited.insert((point, direction));

            match self.grid.get(&point).copied().unwrap_or('.') {
                '/' => {
                    remaining.push(((point.0 - direction.1, point.1 - direction.0), (-direction.1, -direction.0)));
                },
                '\\' => {
                    remaining.push(((point.0 + direction.1, point.1 + direction.0), (direction.1, direction.0)));
                },
                '-' if direction.0 != 0 => {
                    remaining.push(((point.0, point.1 - 1), (0, -1)));
                    remaining.push(((point.0, point.1 + 1), (0, 1)));
                },
                '|' if direction.1 != 0 => {
                    remaining.push(((point.0 - 1, point.1), (-1, 0)));
                    remaining.push(((point.0 + 1, point.1), (1, 0)));
                },
                _ => {
                    remaining.push(((point.0 + direction.0, point.1 + direction.1), direction));
                },
            }
        }

        energized.into_values().count()
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let contraption = Contraption::parse(lines.iter().map(|s| s.as_str()));

    println!("{}", contraption.energized((0, 0), (0, 1)));
    println!("{}", contraption.best_energized());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 10] = [
        ".|...\\....",
        "|.-.\\.....",
        ".....|-...",
        "........|.",
        "..........",
        ".........\\",
        "..../.\\\\..",
        ".-.-/..|..",
        ".|....-|.\\",
        "..//.|....",
    ];

    #[test]
    fn _01() {
        let contraption = Contraption::parse(LINES.iter().copied());

        assert_eq!(contraption.energized((0, 0), (0, 1)), 46);
    }

    #[test]
    fn _02() {
        let contraption = Contraption::parse(LINES.iter().copied());

        assert_eq!(contraption.best_energized(), 51);
    }
}
