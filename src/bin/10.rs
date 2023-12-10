use std::{io::{self, BufRead}, collections::VecDeque};

use micro_ndarray::Array;

#[derive(Clone)]
struct PipeGrid {
    array: Array<char, 2>,
    is_padding: Array<bool, 2>,
}

impl PipeGrid {
    fn parse(lines: &[String]) -> Option<Self> {
        let is_padding = Array::new_with([lines[0].len() + 2, lines.len() + 2], false);
        let array = Array::from_flat(
            lines.iter().flat_map(|line| line.chars()).collect(),
            [lines[0].len(), lines.len()],
        )?;

        Some(Self { array, is_padding })
    }

    fn padded(&self) -> Self {
        let mut array = Array::new_with(
            [self.array.size()[0] * 2, self.array.size()[1] * 2],
            ' '
        );
        let mut is_padding = Array::new_with(array.size(), false);

        for row in 0..self.array.size()[1] {
            for col in 0..self.array.size()[0] {
                is_padding[[col * 2 + 0, row * 2 + 0]] = self.is_padding[[col, row]];
                is_padding[[col * 2 + 0, row * 2 + 1]] = true;
                is_padding[[col * 2 + 1, row * 2 + 0]] = true;
                is_padding[[col * 2 + 1, row * 2 + 1]] = true;

                array[[col * 2 + 0, row * 2 + 0]] = self.array[[col, row]];
                array[[col * 2 + 1, row * 2 + 1]] = '.';
                [
                    array[[col * 2 + 0, row * 2 + 1]],
                    array[[col * 2 + 1, row * 2 + 0]],
                ] =
                    match self.array[[col, row]] {
                        'S' => ['S', 'S'],
                        '.' => ['.', '.'],
                        '|' => ['|', '.'],
                        '-' => ['.', '-'],
                        'L' => ['.', '-'],
                        'J' => ['.', '.'],
                        '7' => ['|', '.'],
                        'F' => ['|', '-'],
                        _ => unreachable!(),
                    };
            }
        }

        Self { array, is_padding }
    }

    fn starting_point(&self) -> [usize; 2] {
        self.array.iter()
            .find_map(|(point, &ch)| {
                if ch == 'S' {
                    Some(point)
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn neighbours(&self, point: [usize; 2]) -> impl Iterator<Item=[usize; 2]> + '_ {
        let north = Some([point[0], point[1].wrapping_sub(1)]).filter(|&at| self.array.get(at).filter(|ch| ['|', '7', 'F', 'S'].contains(ch)).is_some());
        let south = Some([point[0], point[1].wrapping_add(1)]).filter(|&at| self.array.get(at).filter(|ch| ['|', 'L', 'J', 'S'].contains(ch)).is_some());
        let west = Some([point[0].wrapping_sub(1), point[1]]).filter(|&at| self.array.get(at).filter(|ch| ['-', 'L', 'F', 'S'].contains(ch)).is_some());
        let east = Some([point[0].wrapping_add(1), point[1]]).filter(|&at| self.array.get(at).filter(|ch| ['-', 'J', '7', 'S'].contains(ch)).is_some());

        match self.array[point] {
            '|' => vec! [north, south],
            '-' => vec! [east, west],
            'L' => vec! [north, east],
            'J' => vec! [north, west],
            '7' => vec! [south, west],
            'F' => vec! [south, east],
            'S' => vec! [north, south, east, west],
            _ => unreachable!(),
        }.into_iter().filter_map(|x| x)
    }

    fn traverse_pipe(&self) -> Array<usize, 2> {
        let starting_point = self.starting_point();
        let mut remaining: Vec<[usize; 2]> = vec! [starting_point];
        let mut distance_to = Array::new_with(self.array.size(), usize::MAX);
        distance_to[starting_point] = 0;

        while let Some(point) = remaining.pop() {
            let distance = distance_to[point] + 1;

            remaining.extend(self.neighbours(point).filter(|&neighbour| {
                if distance < distance_to[neighbour] {
                    distance_to[neighbour] = distance;
                    true
                } else {
                    false
                }
            }));
        }

        distance_to
    }

    fn is_enclosed(&self, starting_point: [usize; 2], is_pipe: &Array<bool, 2>) -> (bool, Array<bool, 2>) {
        let mut remaining = VecDeque::from([starting_point]);
        let mut visited = Array::new_with(self.array.size(), false);
        visited[starting_point] = true;

        while let Some(point) = remaining.pop_front() {
            let neighbours = [
                [point[0], point[1].wrapping_sub(1)],
                [point[0], point[1].wrapping_add(1)],
                [point[0].wrapping_sub(1), point[1]],
                [point[0].wrapping_add(1), point[1]],
            ];

            for neighbour in neighbours.into_iter() {
                if self.array.get(neighbour) == None {
                    return (false, visited);
                } else if !visited[neighbour] && !is_pipe[neighbour] {
                    remaining.push_back(neighbour);
                }

                visited[neighbour] = true;
            }
        }

        (true, visited)
    }

    fn enclosed_area(&self) -> usize {
        let is_pipe = Array::from_flat(
            self.traverse_pipe().iter().map(|(_, &distance)| distance != usize::MAX).collect(),
            self.array.size(),
        ).unwrap();

        let mut enclosed = Array::new_with(self.array.size(), None);
        let mut count = 0;

        for (point, _) in self.array.iter() {
            if is_pipe[point] || self.is_padding[point] {
                // pass
            } else if let Some(is_enclosed) = enclosed[point] {
                if is_enclosed {
                    count += 1;
                }
            } else  {
                let (is_enclosed, enclosed_) = self.is_enclosed(point, &is_pipe);

                if is_enclosed {
                    for (point, &is_enclosed_) in enclosed_.iter() {
                        debug_assert!(enclosed[point] == None);

                        enclosed[point] = Some(is_enclosed_);
                    }

                    count += 1;
                }
            }
        }

        count
    }

    fn max_distance(&self) -> usize {
        let distance_to = self.traverse_pipe();

        distance_to.iter()
            .filter_map(|(_, &distance)| if distance != usize::MAX { Some(distance) } else { None })
            .max().unwrap()
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(Result::ok).collect::<Vec<_>>();
    let pipe_grid: PipeGrid = PipeGrid::parse(&lines).unwrap();

    println!("{}", pipe_grid.max_distance());
    println!("{}", pipe_grid.padded().enclosed_area());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 5] = [
        "..F7.",
        ".FJ|.",
        "SJ.L7",
        "|F--J",
        "LJ...",
    ];

    const SQUEEZE_LINES: [&str; 9] = [
        "..........",
        ".S------7.",
        ".|F----7|.",
        ".||....||.",
        ".||....||.",
        ".|L-7F-J|.",
        ".|..||..|.",
        ".L--JL--J.",
        "..........",
    ];

    const LARGE_LINES: [&str; 10] = [
        ".F----7F7F7F7F-7....",
        ".|F--7||||||||FJ....",
        ".||.FJ||||||||L7....",
        "FJL7L7LJLJ||LJ.L-7..",
        "L--J.L7...LJS7F-7L7.",
        "....F-J..F7FJ|L7L7L7",
        "....L7.F7||L7|.L7L7|",
        ".....|FJLJ|FJ|F7|.LJ",
        "....FJL-7.||.||||...",
        "....L---J.LJ.LJLJ...",
    ];

    const EXTRA_TILES_LINES: [&str; 10] = [
        "FF7FSF7F7F7F7F7F---7",
        "L|LJ||||||||||||F--J",
        "FL-7LJLJ||||||LJL-77",
        "F--JF--7||LJLJ7F7FJ-",
        "L---JF-JLJ.||-FJLJJ7",
        "|F|F-JF---7F7-L7L|7|",
        "|FFJF7L7F-JF7|JL---7",
        "7-L-JL7||F7|L7F-7F7|",
        "L.L7LFJ|||||FJL7||LJ",
        "L7JLJL-JLJLJL--JLJ.L",
    ];

    #[test]
    fn _01() {
        let pipe_grid: PipeGrid = PipeGrid::parse(&LINES.iter().map(|s| s.to_string()).collect::<Vec<_>>()).unwrap();

        assert_eq!(pipe_grid.max_distance(), 8);
    }

    #[test]
    fn _02() {
        let pipe_grid: PipeGrid = PipeGrid::parse(&LARGE_LINES.iter().map(|s| s.to_string()).collect::<Vec<_>>()).unwrap();

        assert_eq!(pipe_grid.padded().enclosed_area(), 8);
    }

    #[test]
    fn _02_squeeze() {
        let pipe_grid: PipeGrid = PipeGrid::parse(&SQUEEZE_LINES.iter().map(|s| s.to_string()).collect::<Vec<_>>()).unwrap();

        assert_eq!(pipe_grid.padded().enclosed_area(), 4);
    }

    #[test]
    fn _02_extra_tiles() {
        let pipe_grid: PipeGrid = PipeGrid::parse(&EXTRA_TILES_LINES.iter().map(|s| s.to_string()).collect::<Vec<_>>()).unwrap();

        assert_eq!(pipe_grid.padded().enclosed_area(), 10);
    }
}
