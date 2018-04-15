// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

impl<T, A> FromIterator<T> for DenseVector<A>
where
    T: Copy,
    A: Array<Item = T>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut components = ArrayVec::new();
        components.extend(iter);
        Self { components }
    }
}

impl<T, A> IntoIterator for DenseVector<A>
where
    T: Copy,
    A: Array<Item = T>,
{
    type Item = <Self::IntoIter as IntoIterator>::Item;
    type IntoIter = IntoIter<A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.components.into_iter())
    }
}

impl<'a, T, A> IntoIterator for &'a DenseVector<A>
where
    T: 'a + Copy,
    A: Array<Item = T>,
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
    index: usize,
    inner: <ArrayVec<A> as IntoIterator>::IntoIter,
}

impl<A> IntoIter<A>
where
    A: Array,
{
    pub fn new(iter: <ArrayVec<A> as IntoIterator>::IntoIter) -> Self {
        IntoIter { index: 0, inner: iter }
    }
}

impl<T, A> Iterator for IntoIter<A>
where
    T: Copy,
    A: Array<Item = T>,
{
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if let Some(value) = self.inner.next() {
            self.index += 1;
            Some((index, value))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T, A> ExactSizeIterator for IntoIter<A>
where
    T: Copy,
    A: Array<Item = T>,
{
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct Iter<'a, T>
where
    T: 'a
{
    index: usize,
    inner: <&'a [T] as IntoIterator>::IntoIter,
}

impl<'a, T> Iter<'a, T>
where
    T: 'a
{
    pub fn new(iter: <&'a [T] as IntoIterator>::IntoIter) -> Self {
        Iter { index: 0, inner: iter }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Copy,
{
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if let Some(value) = self.inner.next() {
            self.index += 1;
            Some((index, *value))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T>
where
    T: Copy,
{
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn from_iter() {
        let values = [0.1, 0.2, 0.3, 0.4, 0.5];
        let iter = values.iter().cloned();
        let subject = DenseVector::from_iter(iter);
        let expected = ArrayVec::from(values);
        expect!(subject.components).to(be_equal_to(expected));
    }

    #[test]
    fn into_iter() {
        let values = [0.1, 0.2, 0.3, 0.4, 0.5];
        let subject = DenseVector::from(values.clone());
        let expected = vec![(0, 0.1), (1, 0.2), (2, 0.3), (3, 0.4), (4, 0.5)];
        let output: Vec<_> = subject.into_iter().collect();
        expect!(output).to(be_equal_to(expected));
    }
}
