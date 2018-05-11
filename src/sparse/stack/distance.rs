// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use num_traits::Signed;
use ordered_iter::OrderedMapIterator;
use arrayvec::Array;

use Distance;
use super::SparseVector;

impl<T, A> Distance for SparseVector<A>
where
    T: Copy + Signed,
    A: Array<Item = (usize, T)>,
{
    type Scalar = T;

    fn squared_distance(&self, rhs: &Self) -> Self::Scalar {
        let lhs_iter = self.iter();
        let rhs_iter = rhs.iter();
        lhs_iter.inner_join_map(rhs_iter).fold(T::zero(), |sum, (_, (lhs, rhs))| {
            let delta = lhs - rhs;
            sum + (delta * delta)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn squared_distance() {
        let subject = SparseVector::from([(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from([(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);
        let squared_distance = subject.squared_distance(&other);
        expect!(squared_distance).to(be_close_to(13.76));
    }

    #[test]
    fn distance() {
        let subject = SparseVector::from([(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from([(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);
        let distance = subject.distance(&other);
        expect!(distance).to(be_close_to(3.71));
    }
}
