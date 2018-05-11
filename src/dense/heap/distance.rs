// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use num_traits::Signed;

use Distance;
use super::DenseVector;

impl<T> Distance for DenseVector<T>
where
    T: Copy + Signed,
{
    type Scalar = T;

    fn squared_distance(&self, rhs: &Self) -> Self::Scalar {
        let lhs_iter = self.iter();
        let rhs_iter = rhs.iter();
        debug_assert_eq!(lhs_iter.len(), rhs_iter.len());
        lhs_iter.zip(rhs_iter).fold(T::zero(), |sum, ((_, lhs), (_, rhs))| {
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
        let subject = DenseVector::from(vec![0.0, 0.5, 1.0, 2.0, 4.0]);
        let other = DenseVector::from(vec![0.1, 0.2, 0.3, 0.4, 0.0]);
        let squared_distance = subject.squared_distance(&other);
        expect!(squared_distance).to(be_close_to(19.15));
    }

    #[test]
    fn distance() {
        let subject = DenseVector::from(vec![0.0, 0.5, 1.0, 2.0, 4.0]);
        let other = DenseVector::from(vec![0.1, 0.2, 0.3, 0.4, 0.0]);
        let distance = subject.distance(&other);
        expect!(distance).to(be_close_to(4.376));
    }
}
