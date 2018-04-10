// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dense::*;

impl<T, U> Mul<U> for DenseVector<T>
    where T: Clone + Default + MulAssign<T>,
          U: Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(mut self, rhs: U) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl<'a, T, U> Mul<U> for &'a DenseVector<T>
    where T: Clone + Default + MulAssign<T>,
          U: Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(self, rhs: U) -> Self::Output {
        self.clone().mul(rhs)
    }
}

impl<T, U> MulAssign<U> for DenseVector<T>
    where T: Clone + Default + MulAssign<T>,
          U: Into<T>
{
    fn mul_assign(&mut self, rhs: U) {
        let into: T = rhs.into();
        for lhs in &mut self.0 {
            lhs.0 *= into.clone();
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
