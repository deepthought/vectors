// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::{Add, AddAssign};

use arrayvec::Array;

use super::DenseVector;

impl<T, A, V> Add<V> for DenseVector<A>
where
    T: AddAssign<T>,
    A: Array<Item = T>,
    V: IntoIterator<Item = (usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator,
{
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: V) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl<T, A, V> AddAssign<V> for DenseVector<A>
where
    T: AddAssign<T>,
    A: Array<Item = T>,
    V: IntoIterator<Item = (usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator,
{
    #[inline]
    fn add_assign(&mut self, rhs: V) {
        let iter = rhs.into_iter();
        debug_assert_eq!(self.len(), iter.len());
        for (lhs, (_, rhs)) in self.components.iter_mut().zip(iter) {
            *lhs += rhs;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn add() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let result = subject + other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_ref() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let result = subject + &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_assign() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let mut result = subject;
        result += &other;
        expect!(result).to(be_equal_to(expected));
    }
}
