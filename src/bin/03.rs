use std::io::{self, BufRead};

type Grid<T> = Vec<Vec<T>>;

fn mark_adjacent_grid(grid: &Grid<char>) -> Grid<bool> {
    let mut adjacent_grid = vec![vec![false; grid[0].len() + 1]; grid.len() + 1];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                '0'..='9' => { /* pass */ },
                '.' => { /* pass */ },
                _ => {
                    if i > 0 && j > 0 {
                        adjacent_grid[i-1][j-1] = true;
                    }
                    if j > 0 {
                        adjacent_grid[i+0][j-1] = true;
                    }
                    adjacent_grid[i+1][j-1] = true;
                    if i > 0 {
                        adjacent_grid[i-1][j+0] = true;
                    }
                    adjacent_grid[i+1][j+0] = true;
                    if i > 0 {
                        adjacent_grid[i-1][j+1] = true;
                    }
                    adjacent_grid[i+0][j+1] = true;
                    adjacent_grid[i+1][j+1] = true;
                }
            }
        }
    }

    adjacent_grid
}

fn parse_marked_numbers(grid: &Grid<char>, adjacent_grid: &Grid<bool>) -> Vec<usize> {
    let mut numbers = vec! [];
    let mut number = 0;
    let mut marked = false;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                '0'..='9' => {
                    number = number * 10 + grid[i][j].to_digit(10).unwrap() as usize;
                    marked = marked || adjacent_grid[i][j];
                },
                _ => {
                    if marked {
                        numbers.push(number);
                    }

                    number = 0;
                    marked = false;
                },
            }
        }

        if marked {
            numbers.push(number);
        }

        number = 0;
        marked = false;
    }

    numbers
}

fn parse_adjacent_numbers(grid: &Grid<char>) -> Grid<Vec<usize>> {
    let mut grid_numbers = vec! [vec! [vec! []; grid[0].len() + 2]; grid.len() + 2];
    let mut positions = vec! [];
    let mut number = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if let Some(digit) = grid[i][j].to_digit(10) {
                number = number * 10 + digit as usize;
                positions.push((i, j));
            } else {
                let mut adjacent_positions = positions.drain(..).flat_map(|(i, j)| {
                    vec! [
                        (i+0, j+0), (i+0, j+1), (i+0, j+2),
                        (i+1, j+0), (i+1, j+1), (i+1, j+2),
                        (i+2, j+0), (i+2, j+1), (i+2, j+2),
                    ]
                }).collect::<Vec<_>>();

                adjacent_positions.sort_unstable();
                adjacent_positions.dedup();

                for (i, j) in adjacent_positions {
                    grid_numbers[i][j].push(number);
                }

                number = 0;
            }
        }

        if number > 0 {
            let mut adjacent_positions = positions.drain(..).flat_map(|(i, j)| {
                vec! [
                    (i+0, j+0), (i+0, j+1), (i+0, j+2),
                    (i+1, j+0), (i+1, j+1), (i+1, j+2),
                    (i+2, j+0), (i+2, j+1), (i+2, j+2),
                ]
            }).collect::<Vec<_>>();

            adjacent_positions.sort_unstable();
            adjacent_positions.dedup();

            for (i, j) in adjacent_positions {
                grid_numbers[i][j].push(number);
            }

            number = 0;
        }
    }

    grid_numbers
}

fn parse_gears(grid: &Grid<char>, adjacent_numbers: &Grid<Vec<usize>>) -> Vec<usize> {
    let mut gears = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '*' && adjacent_numbers[i+1][j+1].len() == 2 {
                gears.push(adjacent_numbers[i+1][j+1].iter().product::<usize>());
            }
        }
    }

    gears
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let grid: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let adjacent_grid = mark_adjacent_grid(&grid);
    let numbers: Vec<usize> = parse_marked_numbers(&grid, &adjacent_grid);
    let adjacent_numbers = parse_adjacent_numbers(&grid);
    let gears = parse_gears(&grid, &adjacent_numbers);

    println!("{}", numbers.iter().sum::<usize>()); // 553825
    println!("{}", gears.into_iter().sum::<usize>());
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
        let adjacent_grid = mark_adjacent_grid(&grid);
        let numbers = parse_marked_numbers(&grid, &adjacent_grid);

        assert_eq!(numbers.into_iter().sum::<usize>(), 4361);
    }

    #[test]
    fn _02() {
        let grid: Vec<Vec<char>> = LINES.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let adjacent_numbers = parse_adjacent_numbers(&grid);
        let gears = parse_gears(&grid, &adjacent_numbers);

        assert_eq!(gears.into_iter().sum::<usize>(), 467835);
    }
}