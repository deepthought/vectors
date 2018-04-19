// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

impl<T> Mul<T> for DenseVector<T>
where
    T: Copy + MulAssign<T>,
{
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: T) -> Self::Output {
        self.mul_assign(rhs);
        self
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
    fn mul() {
        let subject = DenseVector::from(vec![0.0, 0.5, 1.25, 2.0, 3.0]);
        let expected = DenseVector::from(vec![0.0, 1.0, 2.5, 4.0, 6.0]);
        let result = subject.mul(2.0);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_assign() {
        let subject = DenseVector::from(vec![0.0, 0.5, 1.25, 2.0, 3.0]);
        let expected = DenseVector::from(vec![0.0, 1.0, 2.5, 4.0, 6.0]);
        let mut result = subject;
        result.mul_assign(2.0);
        expect!(result).to(be_equal_to(expected));
    }
}
