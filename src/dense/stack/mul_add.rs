// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

impl<T, A, V> MulAdd<T, V> for DenseVector<A>
where
    T: Copy + MulAddAssign<T, T>,
    A: Array<Item = T>,
    V: IntoIterator<Item = (usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator,
{
    type Output = Self;

    #[inline]
    fn mul_add(mut self, a: T, b: V) -> Self::Output {
        self.mul_add_assign(a, b);
        self
    }
}

impl<T, A, V> MulAddAssign<T, V> for DenseVector<A>
where
    T: Copy + MulAddAssign<T, T>,
    A: Array<Item = T>,
    V: IntoIterator<Item = (usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator,
{
    #[inline]
    fn mul_add_assign(&mut self, a: T, b: V) {
        let iter = b.into_iter();
        debug_assert_eq!(self.len(), iter.len());
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
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([4.0, 2.5, 1.0, 0.0, -1.0]);
        let result = other.mul_add(2.0, subject);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_add_ref() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([4.0, 2.5, 1.0, 0.0, -1.0]);
        let result = other.mul_add(2.0, &subject);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_add_assign() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([4.0, 2.5, 1.0, 0.0, -1.0]);
        let mut result = other;
        result.mul_add_assign(2.0, &subject);
        expect!(result).to(be_equal_to(expected));
    }
}
