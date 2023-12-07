use std::{collections::HashMap, io::{self, BufRead}, ops::AddAssign};

const CARDS: &str = "AKQJT98765432";
const CARDS_WITH_JOKER: &str = "AKQT98765432J";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPairs,
    OnePair,
    HighCard,
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

    fn hand_type(&self, joker: Option<char>) -> HandType {
        let histogram = self.cards.iter().fold(
            HashMap::with_capacity(5),
            |mut histogram, &card| {
                histogram.entry(card).or_insert(0).add_assign(1);
                histogram
            }
        );
        let mut pairs = histogram.iter()
            .filter_map(|(&card, &count)| if Some(card) == joker { None } else { Some(count) })
            .collect::<Vec<_>>();
        pairs.sort_by_key(|&count| -count);
        pairs.resize(5, 0);

        if let Some(num_jokers) = joker.and_then(|joker| histogram.get(&joker)) {
            pairs[0] += num_jokers;
        }

        match &pairs[..5] {
            [5, 0, 0, 0, 0] => HandType::Five,
            [4, 1, 0, 0, 0] => HandType::Four,
            [3, 2, 0, 0, 0] => HandType::FullHouse,
            [3, 1, 1, 0, 0] => HandType::Three,
            [2, 2, 1, 0, 0] => HandType::TwoPairs,
            [2, 1, 1, 1, 0] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }

    fn key(&self, cards: &str, joker: Option<char>) -> (HandType, Vec<usize>) {
        (
            self.hand_type(joker),
            self.cards.iter()
                .map(|&ch| cards.find(ch).unwrap())
                .collect()
        )
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let mut hands = lines.iter().filter_map(|line: &String| Hand::parse(line)).collect::<Vec<_>>();

    hands.sort_by_key(|hand| hand.key(CARDS, None));
    println!("{}", hands.iter().rev().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum::<usize>());

    hands.sort_by_key(|hand| hand.key(CARDS_WITH_JOKER, Some('J')));
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
        hands.sort_by_key(|hand: &Hand| hand.key(CARDS, None));

        assert_eq!(Hand::parse("AAAAA 0").unwrap().hand_type(None), HandType::Five);
        assert_eq!(Hand::parse("AA8AA 0").unwrap().hand_type(None), HandType::Four);
        assert_eq!(Hand::parse("23332 0").unwrap().hand_type(None), HandType::FullHouse);
        assert_eq!(Hand::parse("23456 0").unwrap().hand_type(None), HandType::HighCard);
        assert_eq!(
            hands.iter().map(|hand| hand.hand_type(None)).collect::<Vec<_>>(),
            vec! [
                HandType::Three,
                HandType::Three,
                HandType::TwoPairs,
                HandType::TwoPairs,
                HandType::OnePair,
            ]
        );
        assert_eq!(hands.iter().rev().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum::<usize>(), 6440);
    }

    #[test]
    fn _02() {
        let mut hands = LINES.iter().filter_map(|line| Hand::parse(line)).collect::<Vec<_>>();
        hands.sort_by_key(|hand: &Hand| hand.key(CARDS_WITH_JOKER, Some('J')));

        assert_eq!(
            hands.iter().map(|hand| hand.hand_type(Some('J'))).collect::<Vec<_>>(),
            vec! [
                HandType::Four,
                HandType::Four,
                HandType::Four,
                HandType::TwoPairs,
                HandType::OnePair,
            ]
        );
        assert_eq!(hands.iter().rev().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum::<usize>(), 5905);
    }
}
