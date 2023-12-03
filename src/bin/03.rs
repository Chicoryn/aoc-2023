use std::io::{self, BufRead};

type Grid<T> = Vec<Vec<T>>;

fn try_parse_number_at(grid: &Grid<char>, i: usize, j: usize) -> Option<(usize, usize, usize)> {
    if i >= grid.len() || j >= grid[i].len() {
        return None;
    } else if !grid[i][j].is_digit(10) {
        return None;
    }

    let lower = (0..=j).rev().take_while(|&j| grid[i][j].is_digit(10)).last().unwrap();
    let upper = (j..grid[i].len()).take_while(|&j| grid[i][j].is_digit(10)).last().unwrap();

    grid[i][lower..=upper].iter().collect::<String>().parse::<usize>().ok()
        .map(|number| (i, lower, number))
}

fn parse_numbers_adjacent_to(grid: &Grid<char>, i: usize, j: usize) -> Vec<(usize, usize, usize)> {
    [
        (i.wrapping_sub(1), j.wrapping_sub(1)),
        (i.wrapping_add(0), j.wrapping_sub(1)),
        (i.wrapping_add(1), j.wrapping_sub(1)),
        (i.wrapping_sub(1), j.wrapping_add(0)),
        (i.wrapping_add(1), j.wrapping_add(0)),
        (i.wrapping_sub(1), j.wrapping_add(1)),
        (i.wrapping_add(0), j.wrapping_add(1)),
        (i.wrapping_add(1), j.wrapping_add(1)),
    ].into_iter()
        .filter_map(|(i, j)| try_parse_number_at(grid, i, j))
        .collect::<Vec<_>>()
}

fn parse_part_numbers(grid: &Grid<char>) -> impl Iterator<Item=usize> + '_ {
    let mut part_numbers = (0..grid.len())
        .flat_map(|i| (0..grid[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] != '.' && !grid[i][j].is_digit(10))
        .flat_map(|(i, j)| parse_numbers_adjacent_to(grid, i, j))
        .collect::<Vec<_>>();

    part_numbers.sort_unstable();
    part_numbers.dedup();
    part_numbers.into_iter().map(|(_i, _j, number)| number)
}

fn parse_gears(grid: &Grid<char>) -> impl Iterator<Item=Vec<usize>> + '_ {
    (0..grid.len())
        .flat_map(|i| (0..grid[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == '*')
        .filter_map(|(i, j)| {
            let mut numbers = parse_numbers_adjacent_to(grid, i, j);
            numbers.sort_unstable();
            numbers.dedup();

            if numbers.len() == 2 {
                Some(numbers.into_iter().map(|(_i, _j, number)| number).collect::<Vec<_>>())
            } else {
                None
            }
        })
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let grid: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("{}", parse_part_numbers(&grid).sum::<usize>());
    println!("{}", parse_gears(&grid).map(|numbers| numbers.iter().product::<usize>()).sum::<usize>());
}

#[cfg(test)]
mod test {
    use super::*;

    const LINES: [&'static str; 10] = [
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    #[test]
    fn _01() {
        let grid = LINES.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

        assert_eq!(parse_part_numbers(&grid).sum::<usize>(), 4361);
    }

    #[test]
    fn _02() {
        let grid: Vec<Vec<char>> = LINES.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

        assert_eq!(parse_gears(&grid).map(|numbers| numbers.iter().product::<usize>()).sum::<usize>(), 467835);
    }
}
