use std::{collections::HashSet, io::{self, BufRead}};

use sscanf::scanf;

#[derive(Clone, Debug)]
struct Brick {
    id: u32,
    from: (i32, i32, i32),
    to: (i32, i32, i32),
}

impl Brick {
    fn parse_all(lines: impl Iterator<Item=String>) -> Vec<Self> {
        lines.enumerate().filter_map(|(id, line)| {
            let (x1, y1, z1, x2, y2, z2) = scanf!(line, "{i32},{i32},{i32}~{i32},{i32},{i32}").ok()?;

            assert!(x1 <= x2);
            assert!(y1 <= y2);
            assert!(z1 <= z2);

            Some(Self {
                id: id as u32,
                from: (x1, y1, z1),
                to: (x2, y2, z2),
            })
        }).collect()
    }

    fn intersects(&self, other_brick: &Self) -> bool {
        self.from.0 <= other_brick.to.0 && self.to.0 >= other_brick.from.0
            && self.from.1 <= other_brick.to.1 && self.to.1 >= other_brick.from.1
            && self.from.2 <= other_brick.to.2 && self.to.2 >= other_brick.from.2
    }

    fn drop(&self) -> Option<Self> {
        let mut brick = self.clone();

        if brick.from.2 > 0 {
            brick.from.2 -= 1;
            brick.to.2 -= 1;
            Some(brick)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct Stack {
    bricks: Vec<Brick>,
}

impl Stack {
    fn new(bricks: &[Brick]) -> Self {
        Self {
            bricks: bricks.to_vec(),
        }
    }

    fn collapsable(&self) -> Option<(usize, Brick)> {
        self.bricks.iter().enumerate().find_map(|(i, brick)| {
            let drop_brick = brick.drop()?;

            if self.bricks.iter().enumerate().all(|(j, other_brick)| i == j || !drop_brick.intersects(other_brick)) {
                Some((i, drop_brick))
            } else {
                None
            }
        })
    }

    fn is_collapsable(&self) -> bool {
        self.collapsable().is_some()
    }

    fn safe_disintegrate(&self) -> usize {
        self.bricks.iter().enumerate().filter(|(i, _)| {
            let mut other_stack = self.clone();
            other_stack.bricks.swap_remove(*i);

            !other_stack.is_collapsable()
        }).count()
    }

    fn unsafe_disintegrate(&self) -> usize {
        self.bricks.iter().enumerate().map(|(i, _)| {
            let mut other_stack = self.clone();
            other_stack.bricks.swap_remove(i);
            other_stack.collapse().1.len()
        }).sum()
    }

    fn collapse(mut self) -> (Self, HashSet<u32>) {
        let mut visited = HashSet::new();

        while let Some((index, drop_brick)) = self.collapsable() {
            visited.insert(drop_brick.id);
            self.bricks.push(drop_brick);
            self.bricks.swap_remove(index);
        }

        debug_assert!(!self.is_collapsable());

        (self, visited)
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let bricks = Brick::parse_all(lines.into_iter());
    let stack = Stack::new(&bricks).collapse().0;

    println!("{}", stack.safe_disintegrate());
    println!("{}", stack.unsafe_disintegrate());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 7] = [
        "1,0,1~1,2,1",
        "0,0,2~2,0,2",
        "0,2,3~2,2,3",
        "0,0,4~0,2,4",
        "2,0,5~2,2,5",
        "0,1,6~2,1,6",
        "1,1,8~1,1,9",
    ];

    #[test]
    fn _01() {
        let bricks = Brick::parse_all(LINES.iter().map(|s| s.to_string()));
        let stack = Stack::new(&bricks).collapse().0;

        assert_eq!(stack.bricks.len(), 7);
        assert_eq!(stack.safe_disintegrate(), 5);
    }

    #[test]
    fn _02() {
        let bricks = Brick::parse_all(LINES.iter().map(|s| s.to_string()));
        let stack = Stack::new(&bricks).collapse().0;

        assert_eq!(stack.unsafe_disintegrate(), 7, "{:?}", bricks);
    }
}
