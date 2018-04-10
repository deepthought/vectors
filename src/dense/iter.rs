// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dense::*;

impl<T> FromIterator<T> for DenseVector<T>
    where T: Copy
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let items: Vec<_> = iter.into_iter().collect();
        DenseVector::from(items)
    }
}

impl<T> IntoIterator for DenseVector<T> {
    type Item = <Self::IntoIter as IntoIterator>::Item;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.components.into_iter())
    }
}

pub struct IntoIter<T> {
    index: usize,
    inner: <Vec<T> as IntoIterator>::IntoIter,
}

impl<T> IntoIter<T> {
    pub fn new(iter: <Vec<T> as IntoIterator>::IntoIter) -> Self {
        IntoIter { index: 0, inner: iter }
    }
}

impl<T> Iterator for IntoIter<T> {
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

impl<T> ExactSizeIterator for IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct Iter<'a, T> where T: 'a {
    index: usize,
    inner: <&'a [T] as IntoIterator>::IntoIter,
}

impl<'a, T> Iter<'a, T> where T: 'a {
    pub fn new(iter: <&'a [T] as IntoIterator>::IntoIter) -> Self {
        Iter { index: 0, inner: iter }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
    where T: Copy
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
    where T: Copy
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
        let values = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let subject = DenseVector::from_iter(values.clone());
        let expected = values;
        expect!(subject.components).to(be_equal_to(expected));
    }

    #[test]
    fn into_iter() {
        let values = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let subject = DenseVector::from_iter(values.clone());
        let expected = vec![(0, 0.1), (1, 0.2), (2, 0.3), (3, 0.4), (4, 0.5)];
        let output: Vec<_> = subject.into_iter().collect();
        expect!(output).to(be_equal_to(expected));
    }
}
