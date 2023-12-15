use std::io::{self, BufRead};

use sscanf::scanf;

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, ch| ((acc + ch as usize) * 17) % 256)
}

#[derive(Clone)]
struct Step {
    label: String,
    operation: char,
    focal_length: u64,
    box_number: usize,
    hash_code: usize,
}

impl Step {
    fn parse(s: &str) -> Option<Self> {
        let hash_code = hash(s);

        if let Ok((label, focal_length)) = scanf!(s, "{}={}", String, u64) {
            let box_number = hash(&label);

            Some(Self {
                label,
                operation: '=',
                focal_length,
                box_number,
                hash_code,
            })
        } else if let Ok(label) = scanf!(s, "{}-", String) {
            let box_number = hash(&label);

            Some(Self {
                label,
                operation: '-',
                focal_length: 0,
                box_number,
                hash_code,
            })
        } else {
            None
        }
    }
}

struct InitSeq {
    steps: Vec<Step>,
}

impl InitSeq {
    fn parse(line: &str) -> Self {
        Self {
            steps: line.split(',').filter_map(|s| Step::parse(s)).collect(),
        }
    }

    fn verification_number(&self) -> usize {
        self.steps.iter().map(|s| s.hash_code).sum()
    }

    fn focusing_power(&self) -> usize {
        let mut buckets = vec! [Vec::<Step>::new(); 256];

        for step in &self.steps {
            let bucket = &mut buckets[step.box_number];

            if step.operation == '=' {
                if let Some(index) = bucket.iter().position(|s| s.label == step.label) {
                    bucket[index] = step.clone();
                } else {
                    bucket.push(step.clone());
                }
            } else if step.operation == '-' {
                bucket.retain(|s| s.label != step.label);
            } else {
                unreachable!("unrecognized operation: {}", step.operation);
            }
        }

        buckets.iter()
            .enumerate()
            .flat_map(|(i, bucket)| {
                bucket.iter().enumerate().map(move |(j, step)| (i + 1) * (j + 1) * (step.focal_length as usize))
            })
            .sum()
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();

    println!("{}", InitSeq::parse(&lines[0]).verification_number());
    println!("{}", InitSeq::parse(&lines[0]).focusing_power());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn _01() {
        assert_eq!(InitSeq::parse(LINE).verification_number(), 1320);
    }

    #[test]
    fn _02() {
        assert_eq!(InitSeq::parse(LINE).focusing_power(), 145);
    }
}
