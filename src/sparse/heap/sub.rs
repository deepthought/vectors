// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::{Sub, SubAssign};

use num_traits::Zero;
use ordered_iter::OrderedMapIterator;

use super::SparseVector;

impl<T, V> Sub<V> for SparseVector<T>
where
    T: Copy + Zero + Sub<T, Output = T>,
    V: IntoIterator<Item = (usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator + OrderedMapIterator<Key = usize, Val = T>,
{
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: V) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<T, V> SubAssign<V> for SparseVector<T>
where
    T: Copy + Zero + Sub<T, Output = T>,
    V: IntoIterator<Item = (usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator + OrderedMapIterator<Key = usize, Val = T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: V) {
        self.components = {
            let iter = rhs.into_iter();
            let outer_join = self.iter().outer_join(iter);
            outer_join.filter_map(|(index, (lhs, rhs))| {
                    let value = match (lhs, rhs) {
                        (Some(l), Some(r)) => l.sub(r),
                        (Some(l), None) => l.sub(T::zero()),
                        (None, Some(r)) => T::zero().sub(r),
                        _ => unreachable!(),
                    };
                    if value.is_zero() {
                        None
                    } else {
                        Some((index, value))
                    }
                })
                .collect()
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn sub() {
        let subject = SparseVector::from(vec![(0, 0.2), (1, 0.6), (2, 1.2), (4, 2.0), (5, 0.4)]);
        let other = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (3, -0.3), (4, 2.0)]);
        let result = subject - other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_ref() {
        let subject = SparseVector::from(vec![(0, 0.2), (1, 0.6), (2, 1.2), (4, 2.0), (5, 0.4)]);
        let other = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (3, -0.3), (4, 2.0)]);
        let result = subject - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_assign() {
        let subject = SparseVector::from(vec![(0, 0.2), (1, 0.6), (2, 1.2), (4, 2.0), (5, 0.4)]);
        let other = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (3, -0.3), (4, 2.0)]);

        let mut result = subject;
        result -= &other;
        expect!(result).to(be_equal_to(expected));
    }
}
