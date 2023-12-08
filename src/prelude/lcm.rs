use crate::prelude::gcd;

pub fn lcm(numbers: &[u64]) -> u64 {
    numbers.iter().fold(1, |a, b| {
        let max = a.max(*b);
        let min = a.min(*b);

        min * max / gcd(a, *b)
    })
}
