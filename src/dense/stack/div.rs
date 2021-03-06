// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::{Div, DivAssign};

use arrayvec::Array;

use super::DenseVector;

impl<T, A> Div<T> for DenseVector<A>
where
    T: Copy + DivAssign<T>,
    A: Array<Item = T>,
{
    type Output = DenseVector<A>;

    #[inline]
    fn div(mut self, rhs: T) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl<T, A> DivAssign<T> for DenseVector<A>
where
    T: Copy + DivAssign<T>,
    A: Array<Item = T>,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        for lhs in &mut self.components {
            *lhs /= rhs;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn div() {
        let subject = DenseVector::from([0.0, 1.0, 2.0, 4.0, 6.0]);
        let other = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let result = subject / 2.0;
        expect!(result).to(be_equal_to(other));
    }

    #[test]
    fn div_assign() {
        let subject = DenseVector::from([0.0, 1.0, 2.0, 4.0, 6.0]);
        let other = DenseVector::from([0.0, 0.5, 1.0, 2.0, 3.0]);
        let mut result = subject;
        result /= 2.0;
        expect!(result).to(be_equal_to(other));
    }
}
