// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::{Div, DivAssign};

use num_traits::Zero;

use super::SparseVector;

impl<T> Div<T> for SparseVector<T>
where
    T: Copy + Zero + Div<T, Output = T>,
{
    type Output = SparseVector<T>;

    #[inline]
    fn div(mut self, rhs: T) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl<T> DivAssign<T> for SparseVector<T>
where
    T: Copy + Zero + Div<T, Output = T>,
{
    fn div_assign(&mut self, rhs: T) {
        self.components = {
            self.iter().filter_map(|(index, lhs)| {
                let value = lhs / rhs;
                if value.is_zero() {
                    None
                } else {
                    Some((index, value))
                }
            })
            .collect()
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn div() {
        let subject = SparseVector::from(vec![(0, 0.4), (1, 1.0), (2, 2.0), (4, 4.0), (5, 8.0)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let result = subject / 2.0;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn div_assign() {
        let subject = SparseVector::from(vec![(0, 0.4), (1, 1.0), (2, 2.0), (4, 4.0), (5, 8.0)]);
        let expected = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);

        let mut result = subject;
        result /= 2.0;
        expect!(result).to(be_equal_to(expected));
    }
}
