use rayon::prelude::*;

extern crate num_traits;
use num_traits::{One, Zero};

use std::collections::LinkedList;
use std::hash::Hash;
use std::ops::{AddAssign, SubAssign};

use crate::Counter;

impl<T, N> FromParallelIterator<T> for Counter<T, N>
where
    T: Hash + Eq + Send,
    N: PartialOrd + AddAssign + SubAssign + Zero + One,
{
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: IntoParallelIterator<Item = T>,
    {
        let list: LinkedList<_> = par_iter.into_par_iter().collect();
        let mut counter = Counter::new();
        counter.update(list.into_iter());
        counter
    }
}
