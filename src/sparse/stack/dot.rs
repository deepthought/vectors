// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::{Add, Mul};

use num_traits::Zero;
use ordered_iter::OrderedMapIterator;
use arrayvec::Array;

use Dot;
use super::SparseVector;

impl<'a, T, A, V, I, J> Dot<V> for &'a SparseVector<A>
where
    Self: IntoIterator<IntoIter = I, Item = (usize, T)>,
    T: Add<T, Output = T> + Mul<T, Output = T> + Zero,
    A: Array<Item = (usize, T)>,
    V: IntoIterator<IntoIter = J, Item = (usize, T)>,
    I: OrderedMapIterator<Key = usize, Val = T>,
    J: OrderedMapIterator<Key = usize, Val = T>,
{
    type Scalar = T;

    fn dot(self, rhs: V) -> Self::Scalar {
        dot_sparse!(T => (self, rhs))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn dot() {
        let subject = SparseVector::from([(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from([(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);

        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.85));
    }
}
