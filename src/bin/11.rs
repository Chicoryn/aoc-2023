use std::io::{self, BufRead};

#[derive(Clone, PartialEq, Eq)]
struct Galaxy {
    row: usize,
    col: usize,
}

impl Galaxy {
    fn distance_to(&self, other: &Self) -> usize {
        let row_diff = (self.row as isize - other.row as isize).abs();
        let col_diff = (self.col as isize - other.col as isize).abs();

        (row_diff + col_diff) as usize
    }
}

struct Image {
    galaxies: Vec<Galaxy>
}

impl Image {
    fn parse(lines: impl Iterator<Item=String>) -> Self {
        Self {
            galaxies: lines.enumerate()
                .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, ch)| (row, col, ch)).collect::<Vec<_>>())
                .filter(|(_, _, ch)| *ch == '#')
                .map(|(row, col, _)| Galaxy { row, col })
                .collect(),
        }
    }

    fn is_row_empty(&self, row: usize) -> bool {
        self.galaxies.iter().all(|galaxy| galaxy.row != row)
    }

    fn is_col_empty(&self, col: usize) -> bool {
        self.galaxies.iter().all(|galaxy| galaxy.col != col)
    }

    fn padded(&self, multiplier: usize) -> Self {
        Self {
            galaxies: self.galaxies.iter().map(|galaxy| {
                let num_row_voids = (0..galaxy.row).filter(|&row| self.is_row_empty(row)).count();
                let num_col_voids = (0..galaxy.col).filter(|&col| self.is_col_empty(col)).count();

                Galaxy {
                    row: galaxy.row + (multiplier - 1) * num_row_voids,
                    col: galaxy.col + (multiplier - 1) * num_col_voids,
                }
            }).collect()
        }
    }

    fn pairs(&self) -> impl Iterator<Item=(&Galaxy, &Galaxy)> {
        self.galaxies.iter().enumerate().flat_map(move |(i, galaxy)| {
            self.galaxies.iter().skip(i + 1).map(move |other| (galaxy, other))
        })
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap).collect::<Vec<_>>();
    let image = Image::parse(lines.into_iter());

    println!("{}", image.padded(2).pairs().map(|(a, b)| a.distance_to(b)).sum::<usize>());
    println!("{}", image.padded(1000000).pairs().map(|(a, b)| a.distance_to(b)).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 10] = [
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];

    #[test]
    fn _01() {
        let image = Image::parse(LINES.iter().map(|line| line.to_string())).padded(2);

        assert_eq!(image.pairs().count(), 36);
        assert_eq!(image.pairs().map(|(a, b)| a.distance_to(b)).sum::<usize>(), 374);
    }

    #[test]
    fn _02_10() {
        let image = Image::parse(LINES.iter().map(|line| line.to_string())).padded(10);

        assert_eq!(image.pairs().map(|(a, b)| a.distance_to(b)).sum::<usize>(), 1030);
    }

    #[test]
    fn _02_100() {
        let image = Image::parse(LINES.iter().map(|line| line.to_string())).padded(100);

        assert_eq!(image.pairs().map(|(a, b)| a.distance_to(b)).sum::<usize>(), 8410);
    }
}
