// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::{Sub, SubAssign};

use arrayvec::Array;

use super::DenseVector;

impl<T, A, V> Sub<V> for DenseVector<A>
where
    T: SubAssign<T>,
    A: Array<Item = T>,
    V: IntoIterator<Item = (usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator,
{
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: V) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<T, A, V> SubAssign<V> for DenseVector<A>
where
    T: SubAssign<T>,
    A: Array<Item = T>,
    V: IntoIterator<Item = (usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator,
{
    #[inline]
    fn sub_assign(&mut self, rhs: V) {
        let iter = rhs.into_iter();
        debug_assert_eq!(self.len(), iter.len());
        for (lhs, (_, rhs)) in self.components.iter_mut().zip(iter) {
            *lhs -= rhs;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn sub() {
        let subject = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let result = subject - other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_ref() {
        let subject = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let result = subject - other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_assign() {
        let subject = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let mut result = subject;
        result -= &other;
        expect!(result).to(be_equal_to(expected));
    }
}
