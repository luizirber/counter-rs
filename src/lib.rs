//! Counter based on the Python implementation of same:
//! <https://docs.python.org/3.5/library/collections.html#collections.Counter>
//!
//! Counts recurring elements from an iterable.

use std::collections::HashMap;
use std::hash::Hash;

use std::ops::{Add, Sub, BitAnd, BitOr};

#[derive(Clone)]
pub struct Counter<'a, T: 'a> {
    /// HashMap backing this Counter
    ///
    /// Public to expose the HashMap API for direct manipulation.
    pub hashmap: HashMap<&'a T, usize>,
}

impl<'a, T> Counter<'a, T>
    where T: 'a + Hash + Eq
{
    /// Create a new, empty `Counter`
    pub fn new() -> Counter<'a, T> {
        Counter { hashmap: HashMap::new() }
    }

    /// Create a new `Counter` initialized with the given iterable
    pub fn init<I>(iterable: I) -> Counter<'a, T>
        where I: IntoIterator<Item = &'a T>
    {
        let mut counter = Counter::new();
        counter.update(iterable);
        counter
    }

    /// Add the counts of the elements from the given iterable to this counter
    pub fn update<I>(&mut self, iterable: I)
        where I: IntoIterator<Item = &'a T>
    {
        for item in iterable.into_iter() {
            let entry = self.hashmap.entry(item).or_insert(0);
            *entry += 1;
        }
    }

    /// Remove the counts of the elements from the given iterable to this counter
    ///
    /// Non-positive counts are automatically removed
    pub fn subtract<I>(&mut self, iterable: I)
        where I: IntoIterator<Item = &'a T>
    {
        for item in iterable.into_iter() {
            let mut remove = false;
            if let Some(entry) = self.hashmap.get_mut(item) {
                if *entry >= 0 {
                    *entry -= 1;
                }
                remove = *entry == 0;
            }
            if remove {
                self.hashmap.remove(item);
            }
        }
    }
}

impl<'a, T> Counter<'a, T>
    where T: Ord + Hash
{
    /// Create an iterator over `(frequency, elem)` pairs, sorted most to least common.
    ///
    /// FIXME: This is pretty inefficient: it copies everything into a vector, sorts
    /// the vector, and returns an iterator over the vector. It would be much better
    /// to create some kind of MostCommon struct which implements `Iterator` which
    /// does all the necessary work on demand. PRs appreciated here!
    pub fn most_common(&self) -> ::std::vec::IntoIter<(&&T, &usize)> {
        let mut items = self.hashmap.iter().collect::<Vec<_>>();
        items.sort_by(|&(_, a), &(_, b)| b.cmp(a));
        items.into_iter()
    }
}

impl<'a, T> Add for Counter<'a, T> {
    type Output = Counter<'a, T>;

    /// Add two counters together.
    ///
    /// `out = c + d;` -> `out[x] == c[x] + d[x]`
    fn add(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        let mut counter = self.clone();
        for (key, value) in rhs.hashmap.items() {
            let entry = self.hashmap.entry(key).or_insert(0);
            *entry += value;
        }
    }
}

impl<'a, T> Sub for Counter<'a, T> {
    type Output = Counter<'a, T>;

    /// Subtract (keeping only positive values).
    ///
    /// `out = c - d;` -> `out[x] == c[x] - d[x]`
    fn sub(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        unimplemented!()
    }
}

impl<'a, T> BitAnd for Counter<'a, T> {
    type Output = Counter<'a, T>;

    /// Intersection
    ///
    /// `out = c & d;` -> `out[x] == min(c[x], d[x])`
    fn bitand(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        unimplemented!()
    }
}

impl<'a, T> BitOr for Counter<'a, T> {
    type Output = Counter<'a, T>;

    /// Union
    ///
    /// `out = c | d;` -> `out[x] == max(c[x], d[x])`
    fn bitor(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
