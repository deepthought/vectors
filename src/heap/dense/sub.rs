// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

impl<'a, T> Sub<&'a DenseVector<T>> for DenseVector<T>
where
    T: Copy + SubAssign<T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn sub(mut self, rhs: &'a Self) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<'a, T> Sub<&'a DenseVector<T>> for &'a DenseVector<T>
where
    T: Copy + SubAssign<T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.clone().sub(rhs)
    }
}

impl<'a, T> SubAssign<&'a DenseVector<T>> for DenseVector<T>
where
    T: Copy + SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: &'a Self) {
        assert_eq!(self.len(), rhs.len());
        let iter = rhs.iter();
        for (lhs, (_, rhs)) in self.components.iter_mut().zip(iter) {
            *lhs -= rhs;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn sub() {
        let subject = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let result = subject - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_from_ref() {
        let subject = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let result = (&subject) - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_assign() {
        let subject = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let mut result = subject;
        result -= &other;
        expect!(result).to(be_equal_to(expected));
    }
}
