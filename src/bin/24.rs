use std::{io::{self, BufRead}, str::FromStr, fmt::{self, Display}};

use geo::{coord, Line, line_intersection::line_intersection, LineIntersection};
use sscanf::scanf;
use z3::{Config, Context, Solver, ast::{Ast, Int}};

#[derive(Debug, Clone, Copy)]
struct Hail {
    point: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl Display for Hail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y, z) = self.point;
        let (dx, dy, dz) = self.velocity;

        write!(f, "({}, {}, {}) @ ({}, {}, {})", x, y, z, dx, dy, dz)
    }
}

impl FromStr for Hail {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z, dx, dy, dz) = scanf!(s, "{f64}, {f64}, {f64} @ {f64}, {f64}, {f64}").map_err(|_| ())?;

        Ok(Hail {
            point: (x, y, z),
            velocity: (dx, dy, dz),
        })
    }
}

impl Hail {
    fn intersects2d(&self, other: &Self) -> Option<(f64, f64, f64)> {
        let line_1 = Line::new(
            coord! { x: self.point.0, y: self.point.1 },
            coord! { x: self.point.0 + 100000000000000.0 * self.velocity.0, y: self.point.1 + 100000000000000.0 * self.velocity.1 },
        );
        let line_2 = Line::new(
            coord! { x: other.point.0, y: other.point.1 },
            coord! { x: other.point.0 + 100000000000000.0 * other.velocity.0, y: other.point.1 + 100000000000000.0 * other.velocity.1 },
        );

        line_intersection(line_1, line_2).and_then(|p| {
            match p {
                LineIntersection::SinglePoint { intersection, is_proper } => {
                    if is_proper {
                        Some((intersection.x, intersection.y, 0.0))
                    } else {
                        None
                    }
                }
                LineIntersection::Collinear { intersection: _ } => None,
            }
        })
    }
}

struct Storm {
    hail: Vec<Hail>,
}

impl Storm {
    fn new(hail: &[Hail]) -> Self {
        Self {
            hail: hail.to_vec(),
        }
    }

    fn num_intersects2d(&self, min: f64, max: f64) -> usize {
        let mut count = 0;

        for i in 0..self.hail.len() {
            for j in i + 1..self.hail.len() {
                if let Some(t) = self.hail[i].intersects2d(&self.hail[j]) {
                    if t.0 >= min && t.0 <= max && t.1 >= min && t.1 <= max {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn find_ray(&self) -> i64 {
        let config = Config::new();
        let context = Context::new(&config);
        let solver = Solver::new(&context);

        let x = Int::new_const(&context, "x");
        let y = Int::new_const(&context, "y");
        let z = Int::new_const(&context, "z");
        let vx = Int::new_const(&context, "vx");
        let vy = Int::new_const(&context, "vy");
        let vz = Int::new_const(&context, "vz");

        for (i, hail) in self.hail.iter().enumerate() {
            let x_n = Int::from_i64(&context, hail.point.0 as i64);
            let y_n = Int::from_i64(&context, hail.point.1 as i64);
            let z_n = Int::from_i64(&context, hail.point.2 as i64);
            let vx_n = Int::from_i64(&context, hail.velocity.0 as i64);
            let vy_n = Int::from_i64(&context, hail.velocity.1 as i64);
            let vz_n = Int::from_i64(&context, hail.velocity.2 as i64);
            let t_n = Int::fresh_const(&context, &format!("t_{}", i));

            solver.assert(&t_n.ge(&Int::from_i64(&context, 0)));
            solver.assert(&(&x + &vx * &t_n)._eq(&(&x_n + &vx_n * &t_n)));
            solver.assert(&(&y + &vy * &t_n)._eq(&(&y_n + &vy_n * &t_n)));
            solver.assert(&(&z + &vz * &t_n)._eq(&(&z_n + &vz_n * &t_n)));
        }

        solver.check();

        let model = solver.get_model().expect("no model found");

        let x = model.get_const_interp(&x).unwrap().as_i64().unwrap();
        let y = model.get_const_interp(&y).unwrap().as_i64().unwrap();
        let z = model.get_const_interp(&z).unwrap().as_i64().unwrap();

        x + y + z
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let stones = lines.filter_map(|line| line.parse::<Hail>().ok()).collect::<Vec<_>>();
    let storm = Storm::new(&stones);

    println!("{}", storm.num_intersects2d(200000000000000.0, 400000000000000.0));
    println!("{}", storm.find_ray());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 5] = [
        "19, 13, 30 @ -2, 1, -2",
        "18, 19, 22 @ -1, -1, -2",
        "20, 25, 34 @ -2, -2, -4",
        "12, 31, 28 @ -1, -2, -1",
        "20, 19, 15 @ 1, -5, -3",
    ];

    #[test]
    fn _01() {
        let stones = LINES.iter().filter_map(|line| line.parse::<Hail>().ok()).collect::<Vec<_>>();
        let storm = Storm::new(&stones);

        assert_eq!(storm.num_intersects2d(7.0, 27.0), 2);
    }

    #[test]
    fn _02() {
        let stones = LINES.iter().filter_map(|line| line.parse::<Hail>().ok()).collect::<Vec<_>>();
        let storm = Storm::new(&stones);

        assert_eq!(storm.find_ray(), 47);
    }
}
