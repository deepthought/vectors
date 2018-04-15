// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

use ordered_iter::OrderedMapIterator;

impl<T> MulAdd<T, SparseVector<T>> for SparseVector<T>
where
    T: Copy + Zero + MulAdd<T, T, Output = T>,
{
    type Output = SparseVector<T>;

    #[inline]
    fn mul_add(self, a: T, b: SparseVector<T>) -> Self::Output {
        self.mul_add(a, &b)
    }
}

impl<'a, T> MulAdd<T, &'a SparseVector<T>> for SparseVector<T>
where
    T: Copy + Zero + MulAdd<T, T, Output = T>,
{
    type Output = SparseVector<T>;

    #[inline]
    fn mul_add(mut self, a: T, b: &'a SparseVector<T>) -> Self::Output {
        self.mul_add_assign(a, b);
        self
    }
}

impl<T> MulAddAssign<T, SparseVector<T>> for SparseVector<T>
where
    T: Copy + Zero + MulAdd<T, T, Output = T>,
{
    #[inline]
    fn mul_add_assign(&mut self, a: T, b: SparseVector<T>) {
        self.mul_add_assign(a, &b)
    }
}

impl<'a, T> MulAddAssign<T, &'a SparseVector<T>> for SparseVector<T>
where
    T: Copy + Zero + MulAdd<T, T, Output = T>,
{
    fn mul_add_assign(&mut self, a: T, b: &'a SparseVector<T>) {
        self.components = {
            let iter = b.iter(); //.ordered_map_iterator();
            let outer_join = self.iter().outer_join(iter);
            outer_join.filter_map(|(index, (lhs, rhs))| {
                    let value = match (lhs, rhs) {
                        (Some(l), Some(r)) => l.mul_add(a, r),
                        (Some(l), None) => l.mul_add(a, T::zero()),
                        (None, Some(r)) => T::zero().mul_add(a, r),
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
    fn mul_add() {
        let subject = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let other = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.7), (2, 1.4), (3, 0.6), (4, 2.0), (5, 4.8)]);
        let result = subject.mul_add(2.0, other);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_add_ref() {
        let subject = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let other = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.7), (2, 1.4), (3, 0.6), (4, 2.0), (5, 4.8)]);
        let result = subject.mul_add(2.0, &other);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_add_assign() {
        let subject = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let other = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.7), (2, 1.4), (3, 0.6), (4, 2.0), (5, 4.8)]);

        let mut result = subject;
        result.mul_add_assign(2.0, &other);
        expect!(result).to(be_equal_to(expected));
    }
}
