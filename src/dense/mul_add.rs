// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dense::*;

impl<'a, T> MulAdd<T, &'a DenseVector<T>> for DenseVector<T>
where
    T: Copy + MulAddAssign<T, T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul_add(mut self, a: T, b: &'a Self) -> Self::Output {
        self.mul_add_assign(a, b);
        self
    }
}

impl<'a, T> MulAdd<T, &'a DenseVector<T>> for &'a DenseVector<T>
where
    T: Copy + MulAddAssign<T, T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul_add(self, a: T, b: Self) -> Self::Output {
        self.clone().mul_add(a, b)
    }
}

impl<'a, T> MulAddAssign<T, &'a DenseVector<T>> for DenseVector<T>
where
    T: Copy + MulAddAssign<T, T>,
{
    fn mul_add_assign(&mut self, a: T, b: &'a Self) {
        assert_eq!(self.len(), b.len());
        let iter = b.iter();
        for (lhs, (_, rhs)) in self.components.iter_mut().zip(iter) {
            lhs.mul_add_assign(a, rhs);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn mul_add() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![4.0, 2.5, 1.0, 0.0, -1.0];
        let result = other.mul_add(2.0, &subject);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_add_from_ref() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![4.0, 2.5, 1.0, 0.0, -1.0];
        let result = (&other).mul_add(2.0, &subject);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_add_assign() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![4.0, 2.5, 1.0, 0.0, -1.0];
        let mut result = other;
        result.mul_add_assign(2.0, &subject);
        expect!(result).to(be_equal_to(expected));
    }
}
