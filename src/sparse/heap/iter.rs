// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::iter::FromIterator;

use super::SparseVector;

pub use sparse::iter::{IntoIter, Iter};

impl<T> FromIterator<(usize, T)> for SparseVector<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let items: Vec<_> = iter.into_iter().collect();
        SparseVector::from(items)
    }
}

impl<T> IntoIterator for SparseVector<T> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = IntoIter<Vec<(usize, T)>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.components)
    }
}

impl<'a, T> IntoIterator for &'a SparseVector<T>
where
    T: 'a + Copy,
{
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(&self.components[..])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::iter::{IntoIterator, FromIterator};

    use expectest::prelude::*;

    #[test]
    fn from_iter() {
        let values = vec![(0, 0.1), (1, 0.2), (2, 0.3), (4, 0.4), (5, 0.5)];
        let subject = SparseVector::from_iter(values.clone());
        expect!(subject.components).to(be_equal_to(values));
    }

    #[test]
    fn into_iter() {
        let values = vec![(0, 0.1), (1, 0.2), (2, 0.3), (4, 0.4), (5, 0.5)];
        let subject = SparseVector::from_iter(values.clone());
        let output: Vec<_> = subject.into_iter().collect();
        expect!(output).to(be_equal_to(values));
    }
}
