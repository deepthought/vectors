// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

use ordered_iter::OrderedMapIterator;

impl<T, A> Sub<SparseVector<A>> for SparseVector<A>
where
    T: Copy + Zero + Sub<T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    type Output = SparseVector<A>;

    #[inline]
    fn sub(self, rhs: SparseVector<A>) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<'a, T, A> Sub<&'a SparseVector<A>> for SparseVector<A>
where
    T: Copy + Zero + Sub<T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    type Output = SparseVector<A>;

    #[inline]
    fn sub(mut self, rhs: &'a SparseVector<A>) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<T, A> SubAssign<SparseVector<A>> for SparseVector<A>
where
    T: Copy + Zero + Sub<T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: SparseVector<A>) {
        self.sub_assign(&rhs)
    }
}

impl<'a, T, A> SubAssign<&'a SparseVector<A>> for SparseVector<A>
where
    T: Copy + Zero + Sub<T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    fn sub_assign(&mut self, rhs: &'a SparseVector<A>) {
        self.components = {
            let iter = (&rhs).into_iter();
            let outer_join = self.iter().outer_join(iter);
            outer_join.filter_map(|(index, (lhs, rhs))| {
                    let value = match (lhs, rhs) {
                        (Some(l), Some(r)) => l - r,
                        (Some(l), None) => l,
                        (None, Some(r)) => r,
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
    fn sub() {
        let subject = Type::from_iter(vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)]);
        let other = Type::from_iter(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let expected = Type::from_iter(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let result = subject - other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_ref() {
        let subject = Type::from_iter(vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)]);
        let other = Type::from_iter(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let expected = Type::from_iter(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let result = subject - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_assign() {
        let subject = Type::from_iter(vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)]);
        let other = Type::from_iter(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let expected = Type::from_iter(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);

        let mut result = subject;
        result -= &other;
        expect!(result).to(be_equal_to(expected));
    }
}
