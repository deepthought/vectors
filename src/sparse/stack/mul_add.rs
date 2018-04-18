// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

use ordered_iter::OrderedMapIterator;

impl<T, A> MulAdd<T, SparseVector<A>> for SparseVector<A>
where
    T: Copy + Zero + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    type Output = SparseVector<A>;

    #[inline]
    fn mul_add(self, a: T, b: SparseVector<A>) -> Self::Output {
        self.mul_add(a, &b)
    }
}

impl<'a, T, A> MulAdd<T, &'a SparseVector<A>> for SparseVector<A>
where
    T: Copy + Zero + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    type Output = SparseVector<A>;

    #[inline]
    fn mul_add(mut self, a: T, b: &'a SparseVector<A>) -> Self::Output {
        self.mul_add_assign(a, b);
        self
    }
}

impl<T, A> MulAddAssign<T, SparseVector<A>> for SparseVector<A>
where
    T: Copy + Zero + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    #[inline]
    fn mul_add_assign(&mut self, a: T, b: SparseVector<A>) {
        self.mul_add_assign(a, &b)
    }
}

impl<'a, T, A> MulAddAssign<T, &'a SparseVector<A>> for SparseVector<A>
where
    T: Copy + Zero + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    fn mul_add_assign(&mut self, a: T, b: &'a SparseVector<A>) {
        self.components = {
            let iter = b.iter();
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

    type Type = SparseVector<[(usize, f32); 6]>;

    #[test]
    fn mul_add() {
        let subject = Type::from_iter(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let other = Type::from_iter(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let expected = Type::from_iter(vec![(0, 0.2), (1, 0.7), (2, 1.4), (3, 0.6), (4, 2.0), (5, 4.8)]);
        let result = subject.mul_add(2.0, other);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_add_ref() {
        let subject = Type::from_iter(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let other = Type::from_iter(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let expected = Type::from_iter(vec![(0, 0.2), (1, 0.7), (2, 1.4), (3, 0.6), (4, 2.0), (5, 4.8)]);
        let result = subject.mul_add(2.0, &other);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_add_assign() {
        let subject = Type::from_iter(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let other = Type::from_iter(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let expected = Type::from_iter(vec![(0, 0.2), (1, 0.7), (2, 1.4), (3, 0.6), (4, 2.0), (5, 4.8)]);

        let mut result = subject;
        result.mul_add_assign(2.0, &other);
        expect!(result).to(be_equal_to(expected));
    }
}
