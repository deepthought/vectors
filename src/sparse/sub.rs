// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sparse::*;

use sparse::iter::OrderedMapIterable;
use ordered_iter::OrderedMapIterator;

impl<'a, T> Sub<&'a SparseVector<T>> for SparseVector<T>
where
    T: Copy + Zero + Sub<T, Output = T>,
{
    type Output = SparseVector<T>;

    #[inline]
    fn sub(mut self, rhs: &'a SparseVector<T>) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<'a, T> Sub<&'a SparseVector<T>> for &'a SparseVector<T>
where
    T: Copy + Zero + Sub<T, Output = T>,
{
    type Output = SparseVector<T>;

    #[inline]
    fn sub(self, rhs: &'a SparseVector<T>) -> Self::Output {
        self.clone().sub(rhs)
    }
}

impl<'a, T> SubAssign<&'a SparseVector<T>> for SparseVector<T>
where
    T: Copy + Zero + Sub<T, Output = T>,
{
    fn sub_assign(&mut self, rhs: &'a SparseVector<T>) {
        self.components = {
            let iter = rhs.iter().ordered_map_iterator();
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

    #[test]
    fn sub() {
        let subject = sparse_vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)];
        let expected = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let result = subject - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_from_ref() {
        let subject = sparse_vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)];
        let expected = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let result = (&subject) - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_assign() {
        let subject = sparse_vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)];
        let expected = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];

        let mut result = subject;
        result -= &other;
        expect!(result).to(be_equal_to(expected));
    }
}
