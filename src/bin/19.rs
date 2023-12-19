use std::{collections::HashMap, io::{self, BufRead}, ops::Range, str::FromStr};

use sscanf::scanf;

#[derive(Clone, Copy, Debug)]
enum Category {
    ExtremelyCoolLooking = 0,
    Musical = 1,
    Aerodynamic = 2,
    Shiny = 3,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::ExtremelyCoolLooking),
            "m" => Ok(Self::Musical),
            "a" => Ok(Self::Aerodynamic),
            "s" => Ok(Self::Shiny),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
enum Op {
    GreaterThan { lhs: Category, rhs: u32 },
    LessThan { lhs: Category, rhs: u32 },
    Always,
}

#[derive(Clone, Debug)]
struct Rule {
    op: Op,
    send_to: String,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((lhs, rhs, send_to)) = scanf!(s, "{String}>{u32}:{String}").map_err(|_| ()) {
            Ok(Self {
                op: Op::GreaterThan { lhs: Category::from_str(&lhs)?, rhs },
                send_to
            })
        } else if let Ok((lhs, rhs, send_to)) = scanf!(s, "{String}<{u32}:{String}").map_err(|_| ()) {
            Ok(Self {
                op: Op::LessThan { lhs: Category::from_str(&lhs)?, rhs },
                send_to
            })
        } else {
            Ok(Self {
                op: Op::Always,
                send_to: s.to_string(),
            })
        }
    }
}

impl Rule {
    fn split(&self, part: &Part) -> (Part, Part) {
        match self.op {
            Op::Always => (part.clone(), Part::empty()),
            Op::GreaterThan { lhs, rhs } => {
                let (lower, upper) = part.split_at(lhs as usize, rhs + 1);

                (upper, lower)
            },
            Op::LessThan { lhs, rhs } => part.split_at(lhs as usize, rhs),
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rules) = scanf!(s, "{String}{{{String}}}").map_err(|_| ())?;
        let rules = rules.split(',').map(|s| s.parse::<Rule>()).collect::<Result<Vec<Rule>, ()>>()?;

        Ok(Self { name, rules })
    }
}

impl Workflow {
    fn parse_all(lines: &mut impl Iterator<Item=String>) -> Vec<Self> {
        lines.take_while(|line| !line.is_empty())
            .filter_map(|line| line.parse::<Self>().ok())
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
struct Part {
    ratings: [Range<u32>; 4],
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, m, a, s) = scanf!(s, "{{x={u32},m={u32},a={u32},s={u32}}}").map_err(|_| ())?;

        Ok(Self { ratings: [x..(x+1), m..(m+1), a..(a+1), s..(s+1)] })
    }
}

impl Part {
    fn all() -> Self {
        Self { ratings: [1..4001, 1..4001, 1..4001, 1..4001] }
    }

    fn empty() -> Self {
        Self { ratings: [0..0, 0..0, 0..0, 0..0] }
    }

    fn parse_all(lines: &mut impl Iterator<Item=String>) -> Vec<Self> {
        lines.filter_map(|line| line.parse::<Self>().ok()).collect::<Vec<_>>()
    }

    fn split_at(&self, category: usize, rating: u32) -> (Self, Self) {
        let mut lower = self.clone();
        let mut upper = self.clone();

        lower.ratings[category] = self.ratings[category].start..rating.min(self.ratings[category].end);
        upper.ratings[category] = rating.max(self.ratings[category].start)..self.ratings[category].end;

        (lower, upper)
    }

    fn is_empty(&self) -> bool {
        self.ratings.iter().any(|rating| rating.is_empty())
    }

    fn total_rating(&self) -> usize {
        self.ratings.iter().map(|rating| rating.start).sum::<u32>() as usize
    }

    fn num_accepted(&self, workflows: &[Workflow]) -> usize {
        let workflows = workflows.iter().map(|workflow| (workflow.name.clone(), workflow)).collect::<HashMap<_, _>>();
        let mut remaining = vec! [("in".to_string(), self.clone())];
        let mut count = 0;

        while let Some((current_workflow, part)) = remaining.pop() {
            let workflow = workflows.get(&current_workflow).expect("no workflow found");
            let leftovers = workflow.rules.iter().fold(
                part,
                |leftovers, rule| {
                    let (new_part, remaining_part) = rule.split(&leftovers);

                    if new_part.is_empty() {
                        // pass
                    } else if rule.send_to == "A" {
                        count += new_part.ratings.iter().map(|rating| rating.len()).product::<usize>();
                    } else if rule.send_to == "R" {
                        // pass
                    } else {
                        remaining.push((rule.send_to.clone(), new_part));
                    }

                    remaining_part
                }
            );

            assert!(leftovers.is_empty());
        }

        count
    }

    fn sort(&self, workflows: &[Workflow]) -> bool {
        self.num_accepted(workflows) > 0
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let mut lines = stdin.lines().filter_map(|line| line.ok());
    let workflows = Workflow::parse_all(&mut lines);
    let parts = Part::parse_all(&mut lines);

    println!("{}", parts.iter().filter(|part| part.sort(&workflows)).map(|part| part.total_rating()).sum::<usize>());
    println!("{}", Part::all().num_accepted(&workflows));
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 17] = [
        "px{a<2006:qkq,m>2090:A,rfg}",
        "pv{a>1716:R,A}",
        "lnx{m>1548:A,A}",
        "rfg{s<537:gd,x>2440:R,A}",
        "qs{s>3448:A,lnx}",
        "qkq{x<1416:A,crn}",
        "crn{x>2662:A,R}",
        "in{s<1351:px,qqz}",
        "qqz{s>2770:qs,m<1801:hdj,R}",
        "gd{a>3333:R,R}",
        "hdj{m>838:A,pv}",
        "",
        "{x=787,m=2655,a=1222,s=2876}",
        "{x=1679,m=44,a=2067,s=496}",
        "{x=2036,m=264,a=79,s=2244}",
        "{x=2461,m=1339,a=466,s=291}",
        "{x=2127,m=1623,a=2188,s=1013}",
    ];

    #[test]
    fn _01() {
        let mut lines = LINES.iter().map(|s| s.to_string());
        let workflows = Workflow::parse_all(&mut lines);
        let parts = Part::parse_all(&mut lines);

        assert_eq!(parts.into_iter().filter(|part| part.sort(&workflows)).map(|part| part.total_rating()).sum::<usize>(), 19114);
    }

    #[test]
    fn _02() {
        let mut lines = LINES.iter().map(|s| s.to_string());
        let workflows = Workflow::parse_all(&mut lines);

        assert_eq!(Part::all().num_accepted(&workflows), 167409079868000);
    }
}
