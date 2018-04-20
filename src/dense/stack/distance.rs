use std::ops::{Add, Sub, Mul};

use arrayvec::Array;
use num_traits::{Zero, Signed};

use Distance;
use super::DenseVector;

impl<'a, T, A, V> Distance<V> for &'a DenseVector<A>
where
    Self: IntoIterator<Item = (usize, T)>,
    T: Copy + Signed + Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Zero,
    A: Copy + Array<Item = T>,
    V: IntoIterator<Item = (usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator,
{
    type Scalar = T;

    fn squared_distance(self, rhs: V) -> Self::Scalar {
        squared_distance_dense!(T => (self, rhs))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn squared_distance() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 4.0]);
        let other = DenseVector::from([0.1, 0.2, 0.3, 0.4, 0.0]);
        let squared_distance = subject.squared_distance(&other);
        expect!(squared_distance).to(be_close_to(19.15));
    }

    #[test]
    fn distance() {
        let subject = DenseVector::from([0.0, 0.5, 1.0, 2.0, 4.0]);
        let other = DenseVector::from([0.1, 0.2, 0.3, 0.4, 0.0]);
        let distance = subject.distance(&other);
        expect!(distance).to(be_close_to(4.376));
    }
}
