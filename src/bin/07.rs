use std::{collections::HashMap, io::{self, BufRead}, ops::AddAssign};

const CARDS: &str = "AKQJT98765432";
const JOKER_CARDS: &str = "AKQT98765432J";

#[derive(Debug, PartialEq, Eq)]
enum Strength {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPairs,
    OnePair,
    HighCard,
}

impl PartialOrd for Strength {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

impl Ord for Strength {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl Strength {
    fn rank(&self) -> usize {
        match self {
            Strength::Five => 0,
            Strength::Four => 1,
            Strength::FullHouse => 2,
            Strength::Three => 3,
            Strength::TwoPairs => 4,
            Strength::OnePair => 5,
            Strength::HighCard => 6,
        }
    }
}

struct Hand {
    cards: Vec<char>,
    bid: usize,
}

impl Hand {
    fn parse(line: &str) -> Option<Hand> {
        let mut parts = line.split_whitespace();
        let cards = parts.next().unwrap().chars().collect::<Vec<_>>();
        let bid = parts.next().unwrap().parse::<usize>().ok()?;

        Some(Hand { cards, bid })
    }

    fn strength(&self, include_joker: bool) -> Strength {
        let histogram = self.cards.iter().fold(
            HashMap::new(),
            |mut histogram, &card| {
                histogram.entry(card).or_insert(0).add_assign(1);
                histogram
            }
        );
        let mut pairs = histogram.into_iter()
            .filter_map(|(card, count)| if count > 0 { Some((card, count)) } else { None })
            .collect::<Vec<_>>();
        pairs.sort_by_key(|&(card, count)| (-count, card));

        if pairs.len() == 1 {
            Strength::Five
        } else if pairs.len() == 2 {
            if pairs[0].1 == 4 {
                debug_assert!(pairs[0].1 == 4 && pairs[1].1 == 1);

                if include_joker && (pairs[0].0 == 'J' || pairs[1].0 == 'J') {
                    Strength::Five
                } else {
                    Strength::Four
                }
            } else {
                debug_assert!(pairs[0].1 == 3 && pairs[1].1 == 2);

                if include_joker && (pairs[0].0 == 'J' || pairs[1].0 == 'J') {
                    Strength::Five
                } else {
                    Strength::FullHouse
                }
            }
        } else if pairs.len() == 3 {
            if pairs[0].1 == 3 {
                debug_assert!(pairs[0].1 == 3 && pairs[1].1 == 1 && pairs[2].1 == 1);

                if include_joker && (pairs[0].0 == 'J' || pairs[1].0 == 'J' || pairs[2].0 == 'J') {
                    Strength::Four
                } else {
                    Strength::Three
                }
            } else {
                debug_assert!(pairs[0].1 == 2 && pairs[1].1 == 2 && pairs[2].1 == 1);

                if include_joker && (pairs[0].0 == 'J' || pairs[1].0 == 'J') {
                    Strength::Four
                } else if include_joker && pairs[2].0 == 'J' {
                    Strength::FullHouse
                } else {
                    Strength::TwoPairs
                }
            }
        } else if pairs.len() == 4 {
            debug_assert!(pairs[0].1 == 2 && pairs[1].1 == 1 && pairs[2].1 == 1 && pairs[3].1 == 1);

            if include_joker && (pairs[0].0 == 'J' || pairs[1].0 == 'J' || pairs[2].0 == 'J' || pairs[3].0 == 'J') {
                Strength::Three
            } else {
                Strength::OnePair
            }
        } else {
            debug_assert!(pairs[0].1 == 1 && pairs[1].1 == 1 && pairs[2].1 == 1 && pairs[3].1 == 1 && pairs[4].1 == 1);

            if include_joker && (pairs[0].0 == 'J' || pairs[1].0 == 'J' || pairs[2].0 == 'J' || pairs[3].0 == 'J' || pairs[4].0 == 'J') {
                Strength::OnePair
            } else {
                Strength::HighCard
            }
        }
    }

    fn key(&self, include_joker: bool) -> (Strength, Vec<usize>) {
        let ordering =
            if include_joker {
                JOKER_CARDS
            } else {
                CARDS
            };

        (
            self.strength(include_joker),
            self.cards.iter()
                .map(|&ch| ordering.find(ch).unwrap())
                .collect()
        )
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let mut hands = lines.iter().filter_map(|line: &String| Hand::parse(line)).collect::<Vec<_>>();

    hands.sort_by_key(|hand| hand.key(false));
    println!("{}", hands.iter().rev().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum::<usize>());

    hands.sort_by_key(|hand| hand.key(true));
    println!("{}", hands.iter().rev().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 5] = [
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483",
    ];

    #[test]
    fn _01() {
        let mut hands = LINES.iter().filter_map(|line| Hand::parse(line)).collect::<Vec<_>>();
        hands.sort_by_key(|hand: &Hand| hand.key(false));

        assert_eq!(Hand::parse("AAAAA 0").unwrap().strength(false), Strength::Five);
        assert_eq!(Hand::parse("AA8AA 0").unwrap().strength(false), Strength::Four);
        assert_eq!(Hand::parse("23332 0").unwrap().strength(false), Strength::FullHouse);
        assert_eq!(Hand::parse("23456 0").unwrap().strength(false), Strength::HighCard);
        assert_eq!(
            hands.iter().map(|hand| hand.strength(false)).collect::<Vec<_>>(),
            vec! [
                Strength::Three,
                Strength::Three,
                Strength::TwoPairs,
                Strength::TwoPairs,
                Strength::OnePair,
            ]
        );
        assert_eq!(hands.iter().rev().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum::<usize>(), 6440);
    }

    #[test]
    fn _02() {
        let mut hands = LINES.iter().filter_map(|line| Hand::parse(line)).collect::<Vec<_>>();
        hands.sort_by_key(|hand: &Hand| hand.key(true));

        assert_eq!(
            hands.iter().map(|hand| hand.strength(true)).collect::<Vec<_>>(),
            vec! [
                Strength::Four,
                Strength::Four,
                Strength::Four,
                Strength::TwoPairs,
                Strength::OnePair,
            ]
        );
        assert_eq!(hands.iter().rev().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum::<usize>(), 5905);
    }
}
