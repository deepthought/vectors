// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

impl<'a, T, A> Sub<&'a DenseVector<A>> for DenseVector<A>
where
    T: Copy + SubAssign<T>,
    A: Array<Item = T>,
{
    type Output = DenseVector<A>;

    #[inline]
    fn sub(mut self, rhs: &'a Self) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<'a, T, A> Sub<&'a DenseVector<A>> for &'a DenseVector<A>
where
    T: Copy + SubAssign<T>,
    A: Array<Item = T>,
{
    type Output = DenseVector<A>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.clone().sub(rhs)
    }
}

impl<'a, T, A> SubAssign<&'a DenseVector<A>> for DenseVector<A>
where
    T: Copy + SubAssign<T>,
    A: Array<Item = T>,
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
        let subject = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let result = subject - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_from_ref() {
        let subject = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let result = (&subject) - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_assign() {
        let subject = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let mut result = subject;
        result -= &other;
        expect!(result).to(be_equal_to(expected));
    }
}
