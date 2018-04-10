// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dense::*;

impl<'b, T, U> MulAdd<T, &'b DenseVector<U>> for DenseVector<T>
    where T: Clone + MulAddAssign<T, U>,
          U: Clone
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul_add(mut self, a: T, b: &'b DenseVector<U>) -> Self::Output {
        self.mul_add_assign(a, b);
        self
    }
}

impl<'a, 'b, T, U> MulAdd<T, &'b DenseVector<U>> for &'a DenseVector<T>
    where T: Clone + MulAddAssign<T, U>,
          U: Clone
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul_add(self, a: T, b: &'b DenseVector<U>) -> Self::Output {
        self.clone().mul_add(a, b)
    }
}

impl<'b, T, U> MulAddAssign<T, &'b DenseVector<U>> for DenseVector<T>
    where T: Clone + MulAddAssign<T, U>,
          U: Clone
{
    fn mul_add_assign(&mut self, a: T, b: &'b DenseVector<U>) {
        assert_eq!(self.len(), b.len());
        let iter = b.iter();
        for (lhs, (_, rhs)) in self.components.iter_mut().zip(iter) {
            lhs.mul_add_assign(a.clone(), rhs.clone());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn mul_add() {
        let subject: DenseVector<f64> = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other: DenseVector<f64> = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected: DenseVector<f64> = dense_vec![4.0, 2.5, 1.0, 0.0, -1.0];
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
