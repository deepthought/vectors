// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dense::*;

impl<T, U> Div<U> for DenseVector<T>
    where T: Clone + DivAssign<T>,
          U: Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn div(mut self, rhs: U) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl<'a, T, U> Div<U> for &'a DenseVector<T>
    where T: Clone + DivAssign<T>,
          U: Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn div(self, rhs: U) -> Self::Output {
        self.clone().div(rhs)
    }
}

impl<T, U> DivAssign<U> for DenseVector<T>
    where T: Clone + DivAssign<T>,
          U: Into<T>
{
    #[inline]
    fn div_assign(&mut self, rhs: U) {
        let into: T = rhs.into();
        for lhs in &mut self.components {
            *lhs /= into.clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn div() {
        let subject = dense_vec![0.0, 1.0, 2.0, 4.0, 6.0];
        let other = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let result = subject / 2.0;
        expect!(result).to(be_equal_to(other));
    }

    #[test]
    fn div_from_ref() {
        let subject = dense_vec![0.0, 1.0, 2.0, 4.0, 6.0];
        let other = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let result = (&subject) / 2.0;
        expect!(result).to(be_equal_to(other));
    }

    #[test]
    fn div_assign() {
        let subject = dense_vec![0.0, 1.0, 2.0, 4.0, 6.0];
        let other = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let mut result = subject;
        result /= 2.0;
        expect!(result).to(be_equal_to(other));
    }
}
