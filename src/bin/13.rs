use std::{collections::BTreeMap, io::{self, BufRead}};

struct Pattern {
    x: BTreeMap<(usize, usize), char>,
}

impl Pattern {
    fn parse<'a>(lines: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
        let x = lines.take_while(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, ch)| ((row, col), ch)))
            .collect::<BTreeMap<_, _>>();

        if x.is_empty() {
            None
        } else {
            Some(Self { x })
        }
    }

    fn parse_all<'a>(lines: &mut impl Iterator<Item=&'a str>) -> Vec<Self> {
        let mut patterns = vec! [];

        while let Some(pattern) = Self::parse(lines) {
            patterns.push(pattern);
        }

        patterns
    }

    fn summarize(&self, max_smudges: usize) -> usize {
        let num_columns = self.find_mirror(self.cols(), |i| self.col(i).collect(), max_smudges).unwrap_or(0);
        let num_rows = self.find_mirror(self.rows(), |j| self.row(j).collect(), max_smudges).unwrap_or(0);

        100 * num_rows + num_columns
    }

    fn find_mirror(&self, len: usize, elements: impl Fn(usize) -> Vec<char> + Clone, max_smudges: usize) -> Option<usize> {
        (1..len)
            .find(move |&row| self.num_mirrored_differences(row, len, elements.clone()) == max_smudges)
    }

    fn num_mirrored_differences<'a>(&'a self, row: usize, len: usize, elements: impl Fn(usize) -> Vec<char>) -> usize {
        (0..row).rev()
            .zip(row..len)
            .map(|(i, j)| {
                elements(i).into_iter().zip(elements(j)).filter(|&(a, b)| a != b).count()
            })
            .sum()
    }

    fn cols(&self) -> usize {
        self.x.keys().map(|&(_, col)| col + 1).max().unwrap_or(0)
    }

    fn rows(&self) -> usize {
        self.x.keys().map(|&(row, _)| row + 1).max().unwrap_or(0)
    }

    fn col(&self, col: usize) -> impl Iterator<Item=char> + '_ {
        self.x.iter().filter_map(move |(&(_, c), &ch)| if c == col { Some(ch) } else { None })
    }

    fn row(&self, row: usize) -> impl Iterator<Item=char> + '_ {
        self.x.iter().filter_map(move |(&(r, _), &ch)| if r == row { Some(ch) } else { None })
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let patterns = Pattern::parse_all(&mut lines.iter().map(String::as_str));

    println!("{}", patterns.iter().map(|pattern| pattern.summarize(0)).sum::<usize>());
    println!("{}", patterns.iter().map(|pattern| pattern.summarize(1)).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 15] = [
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ];

    const RIGHT_EDGE: [&str; 17] = [
        "...##.....#..##",
        "..#..#.....#...",
        "##.##.##.#..#..",
        "..#..#...##..##",
        "..........##...",
        "..#..#..##..#..",
        "#......###.#...",
        "#..##..#..#.###",
        "........#.#....",
        "##....##.#.##..",
        "##.##.###.#..##",
        "##....###.##.##",
        "#.####.#.#..#..",
        ".######...##...",
        "...##..##..#.##",
        ".##..##.#.###..",
        ".#....#.##.##..",
    ];

    #[test]
    fn _01() {
        let patterns = Pattern::parse_all(&mut LINES.iter().copied());

        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns.iter().map(|pattern| pattern.summarize(0)).sum::<usize>(), 405);
    }

    #[test]
    fn _01_edge() {
        let pattern = Pattern::parse(&mut RIGHT_EDGE.iter().copied()).unwrap();

        assert_eq!(pattern.summarize(0), 14);
    }

    #[test]
    fn _02() {
        let patterns = Pattern::parse_all(&mut LINES.iter().copied());

        assert_eq!(patterns.iter().map(|pattern| pattern.summarize(1)).sum::<usize>(), 400);
    }
}
