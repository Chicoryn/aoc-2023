use std::io::{self, BufRead};

fn pluck_spelled_out_digits(s: &str) -> Vec<u32> {
    let mut result = vec! [];

    for (i, ch) in s.chars().enumerate() {
        if s[i..].starts_with("one") {
            result.push(1);
        } else if s[i..].starts_with("two") {
            result.push(2);
        } else if s[i..].starts_with("three") {
            result.push(3);
        } else if s[i..].starts_with("four") {
            result.push(4);
        } else if s[i..].starts_with("five") {
            result.push(5);
        } else if s[i..].starts_with("six") {
            result.push(6);
        } else if s[i..].starts_with("seven") {
            result.push(7);
        } else if s[i..].starts_with("eight") {
            result.push(8);
        } else if s[i..].starts_with("nine") {
            result.push(9);
        } else if s[i..].starts_with("zero") {
            result.push(0);
        } else if let Some(digit) = ch.to_digit(10) {
            result.push(digit)
        } else {
            // pass
        }
    }

    result
}

fn pluck_digits(s: &str) -> Vec<u32> {
    s.chars()
        .filter_map(|ch| ch.to_digit(10))
        .map(|d| d as u32)
        .collect::<Vec<_>>()
}

fn parse_calibration_value(digits: &[u32]) -> u32 {
    10 * digits[0] + digits[digits.len() - 1]
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();

    println!("{}", lines.iter().map(|line| parse_calibration_value(&pluck_digits(&line))).sum::<u32>());
    println!("{}", lines.iter().map(|line| parse_calibration_value(&pluck_spelled_out_digits(&line))).sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _01() {
        let lines = &[
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];

        assert_eq!(lines.iter().map(|line| parse_calibration_value(&pluck_digits(&line))).collect::<Vec<_>>(), &[12, 38, 15, 77]);
        assert_eq!(lines.iter().map(|line| parse_calibration_value(&pluck_digits(&line))).sum::<u32>(), 142);
    }

    #[test]
    fn _02() {
        let lines = &[
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen"
        ];

        assert_eq!(lines.iter().map(|line| parse_calibration_value(&pluck_spelled_out_digits(&line))).collect::<Vec<_>>(), &[29, 83, 13, 24, 42, 14, 76]);
        assert_eq!(lines.iter().map(|line| parse_calibration_value(&pluck_spelled_out_digits(&line))).sum::<u32>(), 281);
    }

    #[test]
    fn triple_spelled_digits() {
        assert_eq!(pluck_spelled_out_digits("threeightwo"), &[3, 8, 2]);
    }
}
