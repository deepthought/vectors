// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

use ordered_iter::OrderedMapIterator;

impl<T> Add<SparseVector<T>> for SparseVector<T>
where
    T: Copy + Zero + Add<T, Output = T>,
{
    type Output = SparseVector<T>;

    #[inline]
    fn add(self, rhs: SparseVector<T>) -> Self::Output {
        self.add(&rhs)
    }
}

impl<'a, T> Add<&'a SparseVector<T>> for SparseVector<T>
where
    T: Copy + Zero + Add<T, Output = T>,
{
    type Output = SparseVector<T>;

    #[inline]
    fn add(mut self, rhs: &'a SparseVector<T>) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl<T> AddAssign<SparseVector<T>> for SparseVector<T>
where
    T: Copy + Zero + Add<T, Output = T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: SparseVector<T>) {
        self.add_assign(&rhs)
    }
}

impl<'a, T> AddAssign<&'a SparseVector<T>> for SparseVector<T>
where
    T: Copy + Zero + Add<T, Output = T>,
{
    fn add_assign(&mut self, rhs: &'a SparseVector<T>) {
        self.components = {
            let iter = rhs.iter(); //.ordered_map_iterator();
            let outer_join = self.iter().outer_join(iter);
            outer_join.filter_map(|(index, (lhs, rhs))| {
                    let value = match (lhs, rhs) {
                        (Some(l), Some(r)) => l + r,
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
    fn add() {
        let subject = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)]);
        let result = subject + other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_ref() {
        let subject = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)]);
        let result = subject + &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_assign() {
        let subject = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)]);

        let mut result = subject;
        result += &other;
        expect!(result).to(be_equal_to(expected));
    }
}
