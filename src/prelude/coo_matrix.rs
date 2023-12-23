use std::{collections::HashMap, ops::Range};

pub struct CooMatrix2D<T> {
    data: HashMap<(i32, i32), T>,
    lower: (i32, i32),
    upper: (i32, i32),
}

impl<T> FromIterator<((i32, i32), T)> for CooMatrix2D<T> {
    fn from_iter<I: IntoIterator<Item=((i32, i32), T)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        iter.fold(
            Self::with_capacity(size_hint),
            |mut acc, ((row, col), value)| {
                acc.insert((row, col), value);
                acc
            },
        )
    }
}

impl<T> CooMatrix2D<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            lower: (i32::MAX, i32::MAX),
            upper: (i32::MIN, i32::MIN),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity(capacity),
            lower: (i32::MAX, i32::MAX),
            upper: (i32::MIN, i32::MIN),
        }
    }

    pub fn rows(&self) -> Range<i32> {
        self.lower.0..self.upper.0
    }

    pub fn cols(&self) -> Range<i32> {
        self.lower.1..self.upper.1
    }

    pub fn min(&self) -> (i32, i32) {
        self.lower
    }

    pub fn max(&self) -> (i32, i32) {
        self.upper
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item=((i32, i32), &T)> {
        self.data.iter().map(|(k, v)| (*k, v))
    }

    pub fn get(&self, key: (i32, i32)) -> Option<&T> {
        self.data.get(&key)
    }

    pub fn get_mut(&mut self, key: (i32, i32)) -> Option<&mut T> {
        self.data.get_mut(&key)
    }

    pub fn insert(&mut self, key: (i32, i32), value: T) {
        self.lower.0 = self.lower.0.min(key.0);
        self.lower.1 = self.lower.1.min(key.1);
        self.upper.0 = self.upper.0.max(key.0 + 1);
        self.upper.1 = self.upper.1.max(key.1 + 1);
        self.data.insert(key, value);
    }
}
