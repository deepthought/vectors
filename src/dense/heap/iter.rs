// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::iter::FromIterator;

use super::DenseVector;

pub use dense::iter::{IntoIter, Iter};

impl<T> FromIterator<T> for DenseVector<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let items: Vec<_> = iter.into_iter().collect();
        DenseVector::from(items)
    }
}

impl<T> IntoIterator for DenseVector<T> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = IntoIter<Vec<T>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.components)
    }
}

impl<'a, T> IntoIterator for &'a DenseVector<T>
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
