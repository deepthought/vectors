// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

impl<T, A> Mul<T> for SparseVector<A>
where
    T: Copy + Zero + Mul<T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    type Output = SparseVector<A>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl<'a, T, A> Mul<T> for &'a SparseVector<A>
where
    T: Copy + Zero + Mul<T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    type Output = SparseVector<A>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.clone().mul(rhs)
    }
}

impl<T, A> MulAssign<T> for SparseVector<A>
where
    T: Copy + Zero + Mul<T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    fn mul_assign(&mut self, rhs: T) {
        for (_, lhs) in &mut self.components {
            *lhs = (*lhs) * rhs;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn mul() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let expected = sparse_vec![(0, 0.4), (1, 1.0), (2, 2.0), (4, 4.0), (5, 8.0)];
        let result = subject * 2.0;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_from_ref() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let expected = sparse_vec![(0, 0.4), (1, 1.0), (2, 2.0), (4, 4.0), (5, 8.0)];
        let result = (&subject) * 2.0;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_assign() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let expected = sparse_vec![(0, 0.4), (1, 1.0), (2, 2.0), (4, 4.0), (5, 8.0)];

        let mut result = subject;
        result *= 2.0;
        expect!(result).to(be_equal_to(expected));
    }
}