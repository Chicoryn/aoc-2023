use std::{collections::BTreeSet, ops::Range};

#[derive(Clone)]
struct OrderedRange<T: Copy> {
    range: Range<T>,
}

impl<T: Copy> From<Range<T>> for OrderedRange<T> {
    fn from(range: Range<T>) -> Self {
        Self { range }
    }
}

impl<T: Copy + PartialEq> PartialEq for OrderedRange<T> {
    fn eq(&self, other: &Self) -> bool {
        self.range.start == other.range.start
    }
}

impl<T: Copy + Eq> Eq for OrderedRange<T> {}

impl<T: Copy + PartialOrd> PartialOrd for OrderedRange<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range.start.partial_cmp(&other.range.start)
    }
}

impl<T: Copy + Ord> Ord for OrderedRange<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

impl<T: Copy> OrderedRange<T> {
    fn new(range: Range<T>) -> Self {
        Self { range }
    }

    fn start(&self) -> T {
        self.range.start
    }

    fn end(&self) -> T {
        self.range.end
    }
}

pub struct IntoIter<T: Copy + Ord> {
    ranges: BTreeSet<OrderedRange<T>>,
}

impl<T: Copy + Ord> Iterator for IntoIter<T> {
    type Item = Range<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ranges.pop_first().map(|r| r.range)
    }
}

impl<T: Copy + Ord> IntoIter<T> {
    fn new(ranges: BTreeSet<OrderedRange<T>>) -> Self {
        Self { ranges }
    }
}

pub struct RangeSet<T: Ord + Copy> {
    ranges: BTreeSet<OrderedRange<T>>,
}

impl<T: Ord + Copy> FromIterator<Range<T>> for RangeSet<T> {
    fn from_iter<I: IntoIterator<Item = Range<T>>>(iter: I) -> Self {
        Self {
            ranges: iter.into_iter().map(OrderedRange::from).collect()
        }
    }
}

impl<T: Ord + Copy> IntoIterator for RangeSet<T> {
    type Item = Range<T>;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.ranges)
    }
}

impl<T: Ord + Copy, const N: usize> From<[Range<T>; N]> for RangeSet<T> {
    fn from(ranges: [Range<T>; N]) -> Self {
        Self::from_iter(ranges.into_iter())
    }
}

impl<T: Ord + Copy> RangeSet<T> {
    pub fn new() -> Self {
        Self {
            ranges: BTreeSet::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Range<T>> {
        self.ranges.iter().map(|r| &r.range)
    }

    pub fn push(&mut self, range: Range<T>) {
        let mut range = OrderedRange::new(range.clone());

        while let Some(overlapping_range) = self.ranges.range(..=range.clone()).next_back().filter(|r| r.end() > range.start()).cloned() {
            range = OrderedRange::new(overlapping_range.start()..range.end().max(overlapping_range.end()));
            self.ranges.remove(&overlapping_range);
        }

        while let Some(overlapping_range) = self.ranges.range(range.clone()..).next().filter(|r| r.start() < range.end()).cloned() {
            range = OrderedRange::new(range.start()..overlapping_range.end().max(range.end()));
            self.ranges.remove(&overlapping_range);
        }

        self.ranges.insert(range);
    }

    pub fn pop(&mut self) -> Option<Range<T>> {
        self.ranges.pop_last().map(|r| r.range)
    }

    pub fn extend(&mut self, ranges: impl IntoIterator<Item = Range<T>>) {
        for range in ranges.into_iter() {
            self.push(range);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping() {
        let mut ranges = RangeSet::new();
        ranges.push(5..15);
        ranges.push(0..10);
        ranges.push(20..30);
        ranges.push(25..35);
        ranges.push(150..170);
        ranges.push(120..130);
        ranges.push(100..200);

        assert_eq!(ranges.into_iter().collect::<Vec<_>>(), vec! [0..15, 20..35, 100..200]);
    }

    #[test]
    fn non_overlapping() {
        let mut ranges = RangeSet::new();
        ranges.push(0..10);
        ranges.push(20..30);

        assert_eq!(ranges.into_iter().collect::<Vec<_>>(), vec! [0..10, 20..30]);
    }
}
