use std::{collections::HashMap, ops::Range, io::{self, BufRead}};

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

    fn translate(&self, index: usize) -> Option<usize> {
        if index < self.src_start || index >= self.src_start + self.length {
            None
        } else {
            Some(self.dst_start + index - self.src_start)
        }
    }

    fn translate_range(&self, range: Range<usize>) -> Option<(Range<usize>, Vec<Range<usize>>)> {
        if range.start >= self.src_start + self.length {
            None
        } else if range.end <= self.src_start {
            None
        } else {
            let start = range.start.max(self.src_start);
            let end = range.end.min(self.src_start + self.length);
            let dst_start = self.dst_start + start - self.src_start;
            let dst_end = self.dst_start + end - self.src_start;

            Some((
                dst_start..dst_end,
                [range.start..start, end..range.end].into_iter().filter(|r| !r.is_empty()).collect()
            ))
        }
    }
}

struct Map {
    dst: String,
    ranges: Vec<RangeConverter>,
}

impl Map {
    fn translate(&self, index: usize) -> usize {
        self.ranges.iter()
            .find_map(|r| r.translate(index))
            .unwrap_or(index)
    }

    fn translate_range(&self, range: Range<usize>) -> Vec<Range<usize>> {
        let mut remaining = vec! [range];
        let mut output = vec! [];

        while let Some(range) = remaining.pop() {
            if let Some((translated_range, remaining_ranges)) = self.ranges.iter().find_map(|converter| converter.translate_range(range.clone())) {
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
    fn parse(lines: &[String]) -> Self {
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
            } else if let Some(range) = RangeConverter::parse(line) {
                maps.entry(src.clone()).or_insert_with(|| Map {
                    dst: dst.clone(),
                    ranges: Vec::new(),
                }).ranges.push(range);
            }
        }

        Self { seeds, maps }
    }

    fn seed_ranges(&self) -> Vec<Range<usize>> {
        let mut seed_ranges = vec! [];

        for i in (0..self.seeds.len()).step_by(2) {
            let start = self.seeds[i];
            let length = self.seeds[i + 1];

            seed_ranges.push(start..(start + length));
        }

        seed_ranges
    }

    fn translate(&self, seed: usize) -> usize {
        let mut index = seed;
        let mut name = "seed".to_string();

        while let Some(map) = self.maps.get(&name) {
            index = map.translate(index);
            name = map.dst.clone();
        }

        index
    }

    fn translate_range(&self, seed: Range<usize>) -> Vec<Range<usize>> {
        let mut ranges = vec! [seed];
        let mut name = "seed".to_string();

        while let Some(map) = self.maps.get(&name) {
            ranges = ranges.iter().flat_map(|range| map.translate_range(range.clone())).collect();
            name = map.dst.clone();
        }

        ranges
    }
}


fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let almanac = Almanac::parse(&lines);

    println!("{}", almanac.seeds.iter().map(|&seed| almanac.translate(seed)).min().unwrap());
    println!("{}", almanac.seed_ranges().iter()
        .map(|seed_range| almanac.translate_range(seed_range.clone()))
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
        let almanac = Almanac::parse(&LINES.iter().map(|s| s.to_string()).collect::<Vec<_>>());

        assert_eq!(
            almanac.seeds.iter().map(|&seed| almanac.translate(seed)).collect::<Vec<_>>(),
            vec! [82, 43, 86, 35]
        );
        assert_eq!(almanac.seeds.iter().map(|&seed| almanac.translate(seed)).min().unwrap(), 35);
    }

    #[test]
    fn _02() {
        let almanac = Almanac::parse(&LINES.iter().map(|s| s.to_string()).collect::<Vec<_>>());

        assert_eq!(
            almanac.seed_ranges().iter()
                .map(|seed_range| almanac.translate_range(seed_range.clone()))
                .flat_map(|ranges| ranges.into_iter().map(|range| range.start))
                .min().unwrap(),
            46
        );
    }
}
