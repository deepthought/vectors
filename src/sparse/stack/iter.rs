// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub use sparse::iter::{IntoIter, Iter};

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
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = IntoIter<ArrayVec<A>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.components)
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
