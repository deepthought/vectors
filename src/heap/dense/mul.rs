// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

impl<T> Mul<T> for DenseVector<T>
where
    T: Copy + MulAssign<T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(mut self, rhs: T) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl<'a, T> Mul<T> for &'a DenseVector<T>
where
    T: Copy + MulAssign<T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.clone().mul(rhs)
    }
}

impl<T> MulAssign<T> for DenseVector<T>
where
    T: Copy + MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        for lhs in &mut self.components {
            *lhs *= rhs;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn mul_add() {
        let subject = DenseVector::from(vec![0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from(vec![2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from(vec![4.0, 2.5, 1.0, 0.0, -1.0]);
        let result = other.mul_add(2.0, &subject);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_add_from_ref() {
        let subject = DenseVector::from(vec![0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from(vec![2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from(vec![4.0, 2.5, 1.0, 0.0, -1.0]);
        let result = (&other).mul_add(2.0, &subject);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_add_assign() {
        let subject = DenseVector::from(vec![0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from(vec![2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from(vec![4.0, 2.5, 1.0, 0.0, -1.0]);
        let mut result = other;
        result.mul_add_assign(2.0, &subject);
        expect!(result).to(be_equal_to(expected));
    }
}
