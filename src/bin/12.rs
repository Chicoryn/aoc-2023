use std::{io::{self, BufRead}, collections::HashMap};

struct Springs {
    conditions: Vec<char>,
    sequences: Vec<usize>,
}

impl Springs {
    fn parse(line: &str) -> Option<Self> {
        let mut parts = line.split_whitespace();
        let conditions = parts.next()?.chars().collect::<Vec<_>>();
        let sequences = parts.next()?.split(',').map(|part| part.parse().unwrap()).collect::<Vec<_>>();

        Some(Self { conditions, sequences })
    }

    fn unfold(&self) -> Self {
        Self {
            conditions: self.conditions.iter().chain(['?'].iter()).cycle().take(self.conditions.len() * 5 + 4).copied().collect(),
            sequences: self.sequences.iter().cycle().take(self.sequences.len() * 5).copied().collect(),
        }
    }

    fn assignments_aux(
        &self,
        visited: &mut HashMap<(usize, usize, usize), usize>,
        offset: usize,
        current_sequence: usize,
        remaining_dots: usize
    ) -> usize
    {
        if let Some(result) = visited.get(&(offset, current_sequence, remaining_dots)) {
            return *result;
        } else if current_sequence == self.sequences.len() {
            if self.conditions[offset..].iter().any(|&ch| ch == '#') {
                return 0;
            } else {
                return 1;
            }
        } else if remaining_dots + current_sequence < self.sequences.len() - 1 {
            return 0;
        } else if self.conditions.len() < offset + self.sequences[current_sequence] || self.conditions.len() < offset + remaining_dots {
            return 0;
        }

        let len = self.sequences[current_sequence];
        let end = offset + len;

        if self.conditions[offset] == '#' {
            // next `len` characters must be '#' or '?'
            if self.conditions[offset..end].iter().any(|&ch| ch == '.') {
                return 0;
            }

            // `len + 1` must be '.' or '?'
            if end < self.conditions.len() && self.conditions[end] == '#' {
                return 0;
            }

            let result = self.assignments_aux(visited, (end + 1).min(self.conditions.len()), current_sequence + 1, remaining_dots.saturating_sub(1));

            visited.insert((offset, current_sequence, remaining_dots), result);
            result
        } else if self.conditions[offset] == '.' {
            if remaining_dots == 0 {
                return 0;
            }

            let result = self.assignments_aux(visited, offset + 1, current_sequence, remaining_dots - 1);

            visited.insert((offset, current_sequence, remaining_dots), result);
            result
        } else {
            let dot_assignments =
                if remaining_dots == 0 {
                    0
                } else {
                    self.assignments_aux(visited, offset + 1, current_sequence, remaining_dots - 1)
                };

            // next `len` characters must be '#' or '?'
            if self.conditions[offset..end].iter().any(|&ch| ch == '.') {
                return dot_assignments;
            }

            // `len + 1` must be '.' or '?'
            if end < self.conditions.len() && self.conditions[end] == '#' {
                return dot_assignments;
            }

            let result = dot_assignments + self.assignments_aux(visited, (end + 1).min(self.conditions.len()), current_sequence + 1, remaining_dots.saturating_sub(1));

            visited.insert((offset, current_sequence, remaining_dots), result);
            result
        }
    }

    fn assignments(&self) -> usize {
        let remaining_dots = self.conditions.len() - self.sequences.iter().sum::<usize>();
        let mut visited = HashMap::new();

        self.assignments_aux(&mut visited, 0, 0, remaining_dots)
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().map(Result::unwrap).collect::<Vec<_>>();
    let springs = lines.into_iter().filter_map(|springs| Springs::parse(&springs)).collect::<Vec<_>>();

    println!("{}", springs.iter().map(|springs| springs.assignments()).sum::<usize>());
    println!("{}", springs.iter().map(|springs| springs.unfold().assignments()).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 6] = [
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];

    #[test]
    fn _01() {
        let springs = LINES.iter().filter_map(|line| Springs::parse(line)).collect::<Vec<_>>();

        assert_eq!(springs.iter().map(|springs| springs.assignments()).collect::<Vec<_>>(), [1, 4, 1, 1, 4, 10]);
        assert_eq!(springs.iter().map(|springs| springs.assignments()).sum::<usize>(), 21);
    }

    #[test]
    fn _02_unfold() {
        let spring = Springs::parse(".# 1").unwrap().unfold();

        assert_eq!(spring.conditions, vec! ['.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#']);
        assert_eq!(spring.sequences, vec! [1, 1, 1, 1, 1]);
    }

    #[test]
    fn _02_slow() {
        let spring = Springs::parse("?#????????????##???? 1,1,3,1,6,2").unwrap().unfold();

        assert_eq!(spring.assignments(), 1296);
    }

    #[test]
    fn _02_slow2() {
        let spring = Springs::parse("???.????????#? 1,1,1,3").unwrap().unfold();

        assert_eq!(spring.assignments(), 20232942838);
    }

    #[test]
    fn _02() {
        let springs = LINES.iter().filter_map(|line| Springs::parse(line)).collect::<Vec<_>>();

        assert_eq!(springs.iter().map(|springs| springs.unfold().assignments()).collect::<Vec<_>>(), [1, 16384, 1, 16, 2500, 506250]);
        assert_eq!(springs.iter().map(|springs| springs.unfold().assignments()).sum::<usize>(), 525152);
    }
}
