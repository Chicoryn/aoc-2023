use std::{io::{self, BufRead}, collections::HashMap, fmt::{self, Display}, hash::{Hasher, Hash}};

#[derive(Clone, PartialEq, Eq)]
struct Platform {
    rocks: HashMap<(i32, i32), char>,
    upper_bound: (i32, i32),
}

impl Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lower_bound = self.lower_bound();
        let upper_bound = self.upper_bound();

        for row in lower_bound.0..=upper_bound.0 {
            for col in lower_bound.1..=upper_bound.1 {
                write!(f, "{}", self.rocks.get(&(row, col)).unwrap_or(&'.'))?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Hash for Platform {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let lower_bound = self.lower_bound();
        let upper_bound = self.upper_bound();

        for row in lower_bound.0..=upper_bound.0 {
            for col in lower_bound.1..=upper_bound.1 {
                match self.rocks.get(&(row, col)) {
                    Some(&ch) => state.write_u8(ch as u8),
                    None => state.write_u8(0),
                }
            }
        }
    }
}

impl Platform {
    fn parse(lines: impl Iterator<Item=String>) -> Self {
        let rocks = lines.enumerate()
            .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, ch)| ((row as i32, col as i32), ch)).collect::<Vec<_>>())
            .filter(|&(_, ch)| ch != '.')
            .collect::<HashMap<_, _>>();

        Self {
            rocks: rocks.clone(),
            upper_bound: (
                rocks.keys().map(|&(row, _)| row).max().unwrap_or(0),
                rocks.keys().map(|&(_, col)| col).max().unwrap_or(0)
            )
        }
    }

    fn lower_bound(&self) -> (i32, i32) {
        (0, 0)
    }

    fn upper_bound(&self) -> (i32, i32) {
        self.upper_bound
    }

    fn ray_trace(&self, cubes: &HashMap<(i32, i32), char>, origin: (i32, i32), delta: (i32, i32)) -> (i32, i32) {
        let mut curr_position = origin.clone();
        let lower_bound = self.lower_bound();
        let upper_bound = self.upper_bound();

        loop {
            let new_position = (
                (curr_position.0 + delta.0).max(lower_bound.0).min(upper_bound.0),
                (curr_position.1 + delta.1).max(lower_bound.1).min(upper_bound.1)
            );

            if new_position == curr_position || cubes.contains_key(&new_position) {
                break
            } else {
                curr_position = new_position;
            }
        }

        curr_position
    }

    fn tilted(&self, delta: (i32, i32)) -> Self {
        let mut round_rocks = self.rocks.iter()
            .filter_map(|(&position, &ch)| if ch == 'O' { Some(position) } else { None })
            .collect::<Vec<_>>();
        round_rocks.sort_unstable_by_key(|&(row, col)| (row * -delta.0, col * -delta.1));

        let cubes = self.rocks.iter()
            .filter_map(|(&position, &ch)| if ch == '#' { Some((position, ch)) } else { None })
            .collect::<HashMap<_, _>>();
        let rocks = round_rocks.into_iter()
            .fold(cubes, |mut rocks, (row, col)| {
                rocks.insert(self.ray_trace(&rocks, (row, col), delta), 'O');
                rocks
            });

        Self {
            rocks,
            upper_bound: self.upper_bound(),
        }
    }

    fn cycle_once(&self) -> Self {
        self.tilted((-1, 0))
            .tilted((0, -1))
            .tilted((1, 0))
            .tilted((0, 1))
    }

    fn cycle_loop(&self, n: usize) -> Self {
        (0..n).fold(self.clone(), |platform, _| platform.cycle_once())
    }

    fn cycle(&self, n: usize) -> Self {
        let mut visited = HashMap::new();
        let mut result = self.clone();
        let mut count = 0;

        while count < n {
            if let Some(&prev_count) = visited.get(&result) {
                let cycle_length = count - prev_count;
                let remaining = n - count;

                return result.cycle_loop(remaining % cycle_length);
            } else {
                visited.insert(result.clone(), count);
            }

            result = result.cycle_once();
            count += 1;
        }

        result
    }

    fn total_load(&self) -> usize {
        let len = self.upper_bound().0 + 1;

        self.rocks.iter()
            .filter_map(|(&(row, _), &ch)| if ch == 'O' { Some((len - row) as usize) } else { None })
            .sum()
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let platform: Platform = Platform::parse(lines.into_iter());

    println!("{}", platform.tilted((-1, 0)).total_load());
    println!("{}", platform.cycle(1000000000).total_load());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 10] = [
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ];

    #[test]
    fn _01() {
        let platform: Platform = Platform::parse(LINES.iter().map(|&line| line.to_string()));

        assert_eq!(platform.tilted((-1, 0)).total_load(), 136, "\n{}", platform.tilted((-1, 0)));
    }

    #[test]
    fn _02() {
        let platform: Platform = Platform::parse(LINES.iter().map(|&line| line.to_string()));

        assert_eq!(platform.cycle(1).total_load(), 87, "\n{}", platform.cycle(1));
        assert_eq!(platform.cycle(2).total_load(), 69, "\n{}", platform.cycle(2));
        assert_eq!(platform.cycle(3).total_load(), 69, "\n{}", platform.cycle(3));
        assert_eq!(platform.cycle(1000000000).total_load(), 64, "\n{}", platform.cycle(1000000000));
    }
}
