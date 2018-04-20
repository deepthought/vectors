// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use num_traits::Num;

use arrayvec::Array;

use Dot;
use super::DenseVector;

impl<'a, T, A, V, I, J> Dot<V> for &'a DenseVector<A>
where
    Self: IntoIterator<IntoIter = I, Item = (usize, T)>,
    T: Num,
    A: Array<Item = T>,
    V: IntoIterator<IntoIter = J, Item = (usize, T)>,
    I: ExactSizeIterator<Item = (usize, T)>,
    J: ExactSizeIterator<Item = (usize, T)>,
{
    type Scalar = T;

    fn dot(self, rhs: V) -> Self::Scalar {
        dot_dense!(T => (self, rhs))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn dot() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 4.0]);
        let other = DenseVector::from([0.1, 0.2, 0.3, 0.4, 0.0]);
        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.2));
    }
}
