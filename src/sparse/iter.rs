// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sparse::*;

use ordered_iter::OrderedMapIterator;

impl<T> FromIterator<(usize, T)> for SparseVector<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let items: Vec<_> = iter.into_iter().map(Item).collect();
        SparseVector::from(items)
    }
}

impl<T> IntoIterator for SparseVector<T> {
    type Item = (usize, T);
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

pub struct OrderedMapIteratorWrapper<I>(I);

impl<I> Iterator for OrderedMapIteratorWrapper<I>
    where I: Iterator
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<I, K, V> OrderedMapIterator for OrderedMapIteratorWrapper<I>
    where I: Iterator<Item = (K, V)>
{
    type Key = K;
    type Val = V;
}

pub trait OrderedMapIterable: Sized {
    #[inline]
    fn ordered_map_iterator(self) -> OrderedMapIteratorWrapper<Self> {
        OrderedMapIteratorWrapper(self)
    }
}

impl<I, K, V> OrderedMapIterable for I where I: Iterator<Item = (K, V)> {}

pub struct IntoIter<T>(<Vec<Item<T>> as IntoIterator>::IntoIter);

impl<T> IntoIter<T> {
    #[inline]
    pub fn new(iter: <Vec<Item<T>> as IntoIterator>::IntoIter) -> Self {
        IntoIter(iter)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| i.0)
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> OrderedMapIterator for IntoIter<T> {
    type Key = usize;
    type Val = T;
}

pub struct Iter<'a, T>(<&'a [Item<T>] as IntoIterator>::IntoIter) where T: 'a;

impl<'a, T> Iter<'a, T> {
    #[inline]
    pub fn new(iter: <&'a [Item<T>] as IntoIterator>::IntoIter) -> Self {
        Iter(iter)
    }
}

impl<'a, T> Iterator for Iter<'a, T>
    where T: Clone
{
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| i.0.clone())
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T>
    where T: Clone
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> OrderedMapIterator for Iter<'a, T>
    where T: Clone
{
    type Key = usize;
    type Val = T;
}

#[cfg(test)]
mod test {
    use super::*;

    use std::iter::{IntoIterator, FromIterator};

    use expectest::prelude::*;

    macro_rules! itemize {
        ($vec:expr) => {
            $vec.into_iter().map(|(i, v)| Item((i, v))).collect()
        };
    }

    #[test]
    fn from_iter() {
        let values = vec![(0, 0.1), (1, 0.2), (2, 0.3), (4, 0.4), (5, 0.5)];
        let items: Vec<_> = itemize!(values.clone());
        let subject = SparseVector::from_iter(values.clone());
        expect!(subject.0).to(be_equal_to(items));
    }

    #[test]
    fn into_iter() {
        let values = vec![(0, 0.1), (1, 0.2), (2, 0.3), (4, 0.4), (5, 0.5)];
        let subject = SparseVector::from_iter(values.clone());
        let output: Vec<_> = subject.into_iter().collect();
        expect!(output).to(be_equal_to(values));
    }
}
