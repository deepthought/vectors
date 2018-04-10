// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dense::*;

impl<'b, T, U> Add<&'b DenseVector<U>> for DenseVector<T>
    where T: Clone + Default + AddAssign<T>,
          U: Clone + Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn add(mut self, rhs: &'b DenseVector<U>) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl<'a, 'b, T, U> Add<&'b DenseVector<U>> for &'a DenseVector<T>
    where T: Clone + Default + AddAssign<T>,
          U: Clone + Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn add(self, rhs: &'b DenseVector<U>) -> Self::Output {
        self.clone().add(rhs)
    }
}

impl<'b, T, U> AddAssign<&'b DenseVector<U>> for DenseVector<T>
    where T: Clone + Default + AddAssign<T>,
          U: Clone + Into<T>
{
    fn add_assign(&mut self, rhs: &'b DenseVector<U>) {
        assert_eq!(self.len(), rhs.len());
        let iter = rhs.iter();
        for (lhs, (_, rhs)) in self.components.iter_mut().zip(iter) {
            *lhs += rhs.into();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn add() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let result = subject + &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_from_ref() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let result = (&subject) + &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_assign() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let mut result = subject;
        result += &other;
        expect!(result).to(be_equal_to(expected));
    }
}
