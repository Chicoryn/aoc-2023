use std::io::{self, BufRead};

struct Race {
    total_duration: usize,
    best_distance: usize,
}

impl Race {
    fn parse_all(lines: &[String], ignore_whitespace: bool) -> Vec<Self> {
        let mut distances = vec! [];
        let mut times = vec! [];

        for line in lines {
            let stripped_line =
                if ignore_whitespace {
                    line.chars().filter(|c| c.is_digit(10)).collect()
                } else {
                    line.clone()
                };

            let numbers = stripped_line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<_>>();

            if line.starts_with("Distance:") {
                distances = numbers;
            } else if line.starts_with("Time:") {
                times = numbers;
            }
        }

        distances.into_iter().zip(times.into_iter())
            .map(|(best_distance, total_duration)| Self { best_distance, total_duration })
            .collect()
    }

    fn simulate(&self, speed: usize) -> usize {
        let remaining_duration = self.total_duration - speed;

        speed * remaining_duration
    }

    fn count(&self) -> usize {
        (1..self.total_duration)
            .map(|speed| self.simulate(speed))
            .filter(|&distance| distance > self.best_distance)
            .count()
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();

    println!("{}", Race::parse_all(&lines, false).iter().map(Race::count).product::<usize>());
    println!("{}", Race::parse_all(&lines, true).iter().map(Race::count).product::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 2] = [
        "Time:      7  15   30",
        "Distance:  9  40  200",
    ];

    #[test]
    fn _01() {
        let races = Race::parse_all(
            &LINES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            false
        );

        assert_eq!(races.iter().map(Race::count).product::<usize>(), 288);
    }

    #[test]
    fn _02() {
        let races = Race::parse_all(
            &LINES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            true
        );

        assert_eq!(races.iter().map(Race::count).product::<usize>(), 71503);
    }
}
