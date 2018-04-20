// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::iter::FromIterator;

use arrayvec::{Array, ArrayVec};

use super::DenseVector;

pub use dense::iter::{IntoIter, Iter};

impl<T, A> FromIterator<T> for DenseVector<A>
where
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
    A: Array<Item = T>,
{
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = IntoIter<ArrayVec<A>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.components)
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
        Iter::new(&self.components[..])
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
