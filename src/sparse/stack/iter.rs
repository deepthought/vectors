// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

use ordered_iter::OrderedMapIterator;

impl<T, A> FromIterator<(usize, T)> for SparseVector<A>
where
    A: Array<Item = (usize, T)>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let mut components = ArrayVec::new();
        components.extend(iter);
        Self { components }
    }
}

impl<T, A> IntoIterator for SparseVector<A>
where
    T: Copy,
    A: Array<Item = (usize, T)>,
{
    type Item = <Self::IntoIter as IntoIterator>::Item;
    type IntoIter = IntoIter<A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.components.into_iter())
    }
}

impl<'a, T, A> IntoIterator for &'a SparseVector<A>
where
    T: 'a + Copy,
    A: Array<Item = (usize, T)>,
{
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter::new((&self.components[..]).into_iter())
    }
}

pub struct IntoIter<A>
where
    A: Array,
{
    inner: <ArrayVec<A> as IntoIterator>::IntoIter,
}

impl<A> IntoIter<A>
where
    A: Array,
{
    #[inline]
    pub fn new(iter: <ArrayVec<A> as IntoIterator>::IntoIter) -> Self {
        IntoIter { inner: iter }
    }
}

impl<T, A> Iterator for IntoIter<A>
where
    T: Copy,
    A: Array<Item = (usize, T)>,
{
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T, A> ExactSizeIterator for IntoIter<A>
where
    T: Copy,
    A: Array<Item = (usize, T)>,
{
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T, A> OrderedMapIterator for IntoIter<A>
where
    T: Copy,
    A: Array<Item = (usize, T)>,
{
    type Key = usize;
    type Val = T;
}

pub struct Iter<'a, T>
where
    T: 'a
{
    inner: <&'a [(usize, T)] as IntoIterator>::IntoIter,
}

impl<'a, T> Iter<'a, T> {
    #[inline]
    pub fn new(iter: <&'a [(usize, T)] as IntoIterator>::IntoIter) -> Self {
        Iter { inner: iter }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Copy
{
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|i| i.clone())
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T>
where
    T: Copy
{
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, T> OrderedMapIterator for Iter<'a, T>
where
    T: Copy
{
    type Key = usize;
    type Val = T;
}

#[cfg(test)]
mod test {
    use super::*;

    use std::iter::{IntoIterator, FromIterator};

    use expectest::prelude::*;

    #[test]
    fn from_iter() {
        const VALUES: [(usize, f32); 5] = [(0, 0.0), (1, 1.0), (2, 0.5), (4, 0.25), (8, 0.125)];
        let subject = SparseVector::from_iter(VALUES.iter().cloned());
        let expected = ArrayVec::from(VALUES);
        expect!(subject.components).to(be_equal_to(expected));
    }

    #[test]
    fn into_iter() {
        let values = vec![(0, 0.1), (1, 0.2), (2, 0.3), (4, 0.4), (5, 0.5)];
        let subject = SparseVector::from([(0, 0.1), (1, 0.2), (2, 0.3), (4, 0.4), (5, 0.5)]);
        let output: Vec<_> = subject.into_iter().collect();
        expect!(output).to(be_equal_to(values));
    }
}
