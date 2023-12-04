use std::io::{self, BufRead};

use sscanf::scanf;

struct Scratchcard {
    id: usize,
    numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}

impl Scratchcard {
    fn parse(line: &str) -> Option<Self> {
        let line = line.split_whitespace().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ");
        let (id, winning_numbers, numbers) = scanf!(line, "Card {usize}: {String} | {String}").ok()?;

        Some(Self {
            id,
            numbers: numbers.split_whitespace().filter_map(|s| s.parse().ok()).collect(),
            winning_numbers: winning_numbers.split_whitespace().filter_map(|s| s.parse().ok()).collect(),
        })
    }

    fn id(&self) -> usize {
        self.id
    }

    fn matching_numbers(&self) -> usize {
        self.numbers.iter().filter(|n| self.winning_numbers.contains(n)).count()
    }

    fn score(&self) -> usize {
        let n = self.matching_numbers();

        if n > 0 {
            2_i32.pow(n as u32 - 1) as usize
        } else {
            0
        }
    }
}

fn total_scratchcards(scatchcards: &[Scratchcard]) -> usize {
    let max_id = scatchcards.iter().map(Scratchcard::id).max().unwrap();
    let mut copies = vec! [0; max_id + 1];

    for scratchcard in scatchcards {
        let matching_numbers = scratchcard.matching_numbers();
        let id = scratchcard.id();

        copies[id] += 1;
        for i in (id + 1)..=(id + matching_numbers) {
            copies[i] += copies[id];
        }
    }

    copies.iter().sum()
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let scratchcards = lines.iter().filter_map(|line| Scratchcard::parse(line)).collect::<Vec<_>>();

    println!("{}", scratchcards.iter().map(|s| s.score()).sum::<usize>());
    println!("{}", total_scratchcards(&scratchcards));
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&'static str; 6] = [
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    #[test]
    fn _01() {
        let scratchcards = LINES.iter().filter_map(|line| Scratchcard::parse(line)).collect::<Vec<_>>();

        assert_eq!(scratchcards.iter().map(|s| s.score()).sum::<usize>(), 13);
    }

    #[test]
    fn _02() {
        let scratchcards = LINES.iter().filter_map(|line| Scratchcard::parse(line)).collect::<Vec<_>>();

        assert_eq!(total_scratchcards(&scratchcards), 30);
    }
}
