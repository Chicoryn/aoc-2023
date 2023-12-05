use std::{collections::HashMap, ops::Range, io::{self, BufRead}};

use aoc_2023::prelude::*;
use sscanf::sscanf;

#[derive(sscanf::FromScanf)]
#[sscanf(format = "{dst_start} {src_start} {length}")]
struct RangeConverter {
    dst_start: usize,
    src_start: usize,
    length: usize,
}

impl RangeConverter {
    fn parse(line: &str) -> Option<Self> {
        sscanf!(line, "{RangeConverter}").ok()
    }

    fn translate(&self, range: Range<usize>) -> Option<(Range<usize>, Vec<Range<usize>>)> {
        if range.start >= self.src_start + self.length {
            None
        } else if range.end <= self.src_start {
            None
        } else {
            let src_start = range.start.max(self.src_start);
            let src_end = range.end.min(self.src_start + self.length);
            let dst_start = self.dst_start + src_start - self.src_start;
            let dst_end = self.dst_start + src_end - self.src_start;

            Some((
                dst_start..dst_end,
                [range.start..src_start, src_end..range.end].into_iter()
                    .filter(|r| !r.is_empty())
                    .collect()
            ))
        }
    }
}

struct Map {
    dst: String,
    converters: Vec<RangeConverter>,
}

impl Map {
    fn translate(&self, range: Range<usize>) -> RangeSet<usize> {
        let mut remaining = RangeSet::new();
        let mut output = RangeSet::new();

        remaining.push(range);
        while let Some(range) = remaining.pop() {
            if let Some((translated_range, remaining_ranges)) = self.converters.iter().find_map(|converter| converter.translate(range.clone())) {
                remaining.extend(remaining_ranges);
                output.push(translated_range);
            } else {
                output.push(range.clone());
            }
        }

        output
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<String, Map>
}

impl Almanac {
    fn parse(lines: impl Iterator<Item=String>) -> Self {
        let mut maps = HashMap::new();
        let mut seeds = vec! [];
        let mut src = String::new();
        let mut dst = String::new();

        for line in lines {
            if let Ok(seeds_) = sscanf!(line, "seeds: {String}") {
                seeds = seeds_.split_whitespace().filter_map(|s| s.parse().ok()).collect()
            } else if let Ok((src_, dst_)) = sscanf!(line, "{String}-to-{String} map:") {
                src = src_;
                dst = dst_;
            } else if let Some(range) = RangeConverter::parse(&line) {
                maps.entry(src.clone()).or_insert_with(|| Map {
                    dst: dst.clone(),
                    converters: Vec::new(),
                }).converters.push(range);
            }
        }

        Self { seeds, maps }
    }

    fn seeds(&self) -> &[usize] {
        &self.seeds
    }

    fn seed_ranges(&self) -> RangeSet<usize> {
        let mut seed_ranges = RangeSet::new();

        for i in (0..self.seeds.len()).step_by(2) {
            let start = self.seeds[i];
            let length = self.seeds[i + 1];

            seed_ranges.push(start..(start + length));
        }

        seed_ranges
    }

    fn translate(&self, seed: Range<usize>) -> RangeSet<usize> {
        let mut ranges = RangeSet::new();
        let mut name = "seed".to_string();

        ranges.push(seed);
        while let Some(map) = self.maps.get(&name) {
            ranges = ranges.iter().flat_map(|range| map.translate(range.clone())).collect();
            name = map.dst.clone();
        }

        ranges
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let almanac = Almanac::parse(stdin.lines().filter_map(Result::ok));

    println!("{}", almanac.seeds().iter().flat_map(|&seed| almanac.translate(seed..(seed + 1))).map(|range| range.start).min().unwrap());
    println!("{}", almanac.seed_ranges().iter()
        .map(|seed_range| almanac.translate(seed_range.clone()))
        .flat_map(|ranges| ranges.into_iter().map(|range| range.start))
        .min().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&'static str; 33] = [
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];

    #[test]
    fn _01() {
        let almanac = Almanac::parse(LINES.iter().map(|s| s.to_string()));

        assert_eq!(
            almanac.seeds().iter()
                .flat_map(|&seed| almanac.translate(seed..(seed + 1)))
                .map(|r| r.start)
                .collect::<Vec<_>>(),
            vec! [82, 43, 86, 35]
        );
        assert_eq!(
            almanac.seeds().iter()
                .flat_map(|&seed| almanac.translate(seed..(seed + 1)))
                .map(|range| range.start)
                .min().unwrap(),
            35
        );
    }

    #[test]
    fn _02() {
        let almanac = Almanac::parse(LINES.iter().map(|s| s.to_string()));

        assert_eq!(
            almanac.seed_ranges().iter()
                .map(|seed_range| almanac.translate(seed_range.clone()))
                .flat_map(|ranges| ranges.into_iter().map(|range| range.start))
                .min().unwrap(),
            46
        );
    }
}
