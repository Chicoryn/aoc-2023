use std::io::{self, BufRead};

struct ReportHistory {
    numbers: Vec<i64>,
}

impl ReportHistory {
    fn parse(line: &str) -> ReportHistory {
        let numbers = line.split_whitespace()
            .filter_map(|word| word.parse::<i64>().ok())
            .collect::<Vec<_>>();

        Self { numbers }
    }

    fn rev(&self) -> Self {
        ReportHistory { numbers: self.numbers.iter().rev().copied().collect() }
    }

    fn differences(&self) -> Vec<Vec<i64>> {
        let mut differences = vec! [self.numbers.clone()];

        while differences.last().unwrap().iter().any(|&x| x != 0) {
            let last = differences.last().unwrap();
            let next = last.iter().zip(last.iter().skip(1)).map(|(&x, &y)| y - x).collect::<Vec<_>>();

            differences.push(next);
        }

        differences
    }

    fn next_number(&self) -> i64 {
        self.differences().iter()
            .rev()
            .filter_map(|diff| diff.last())
            .sum()
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(Result::ok).collect::<Vec<_>>();

    println!("{}", lines.iter().map(|line| ReportHistory::parse(line).next_number()).sum::<i64>());
    println!("{}", lines.iter().map(|line| ReportHistory::parse(line).rev().next_number()).sum::<i64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 3] = [
        "0 3 6 9 12 15",
        "1 3 6 10 15 21",
        "10 13 16 21 30 45",
    ];

    #[test]
    fn _01() {
        assert_eq!(
            LINES.iter().map(|line| ReportHistory::parse(line).next_number()).collect::<Vec<_>>(),
            vec! [18, 28, 68]
        );
    }

    #[test]
    fn _02() {
        assert_eq!(
            LINES.iter().map(|line| ReportHistory::parse(line).rev().next_number()).collect::<Vec<_>>(),
            vec! [-3, 0, 5]
        );
    }
}
