// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

impl<T, A> Add<DenseVector<A>> for DenseVector<A>
where
    T: Copy + AddAssign<T>,
    A: Array<Item = T>,
{
    type Output = DenseVector<A>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl<'a, T, A> Add<&'a DenseVector<A>> for DenseVector<A>
where
    T: Copy + AddAssign<T>,
    A: Array<Item = T>,
{
    type Output = DenseVector<A>;

    #[inline]
    fn add(mut self, rhs: &'a Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl<T, A> AddAssign<DenseVector<A>> for DenseVector<A>
where
    T: Copy + AddAssign<T>,
    A: Array<Item = T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs)
    }
}

impl<'a, T, A> AddAssign<&'a DenseVector<A>> for DenseVector<A>
where
    T: Copy + AddAssign<T>,
    A: Array<Item = T>,
{
    fn add_assign(&mut self, rhs: &'a Self) {
        assert_eq!(self.len(), rhs.len());
        let iter = (&rhs).into_iter();
        for (lhs, (_, rhs)) in self.components.iter_mut().zip(iter) {
            *lhs += rhs;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn add() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let result = subject + other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_ref() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let result = subject + &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_assign() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let other = DenseVector::from([2.0, 1.0, 0.0, -1.0, -2.0]);
        let expected = DenseVector::from([2.0, 1.5, 1.0, 1.0, 1.0]);
        let mut result = subject;
        result += &other;
        expect!(result).to(be_equal_to(expected));
    }
}
