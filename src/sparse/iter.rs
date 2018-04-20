// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ordered_iter::OrderedMapIterator;

pub struct IntoIter<I>
where
    I: IntoIterator,
{
    inner: <I as IntoIterator>::IntoIter,
}

impl<I> IntoIter<I>
where
    I: IntoIterator,
{
    pub fn new(iter: I) -> Self {
        IntoIter { inner: iter.into_iter() }
    }
}

impl<T, I> Iterator for IntoIter<I>
where
    I: IntoIterator<Item = (usize, T)>
{
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T, I> ExactSizeIterator for IntoIter<I>
where
    I: IntoIterator<Item = (usize, T)>,
    <I as IntoIterator>::IntoIter: ExactSizeIterator
{
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T, I> OrderedMapIterator for IntoIter<I>
where
    I: IntoIterator<Item = (usize, T)>,
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
    pub fn new(iter: &'a [(usize, T)]) -> Self {
        Iter { inner: iter.into_iter() }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Copy
{
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|pair| *pair)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
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

    use expectest::prelude::*;

    #[test]
    fn into_iter() {

        let values = vec![(0, 0.1), (1, 0.2), (2, 0.3), (4, 0.4), (5, 0.5)];
        let iter = IntoIter::new(values.clone());
        let subject: Vec<_> = iter.collect();
        let expected = values;
        expect!(subject).to(be_equal_to(expected));
    }

    #[test]
    fn iter() {
        let values = vec![(0, 0.1), (1, 0.2), (2, 0.3), (4, 0.4), (5, 0.5)];
        let subject: Vec<_> = {
            let iter = Iter::new(&values[..]);
            iter.collect()
        };
        let expected = values;
        expect!(subject).to(be_equal_to(expected));
    }
}
