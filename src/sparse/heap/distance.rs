use std::ops::{Add, Mul};

use num_traits::{Zero, Signed};
use ordered_iter::OrderedMapIterator;

use Distance;
use super::SparseVector;

impl<'a, T, V, I, J> Distance<V> for &'a SparseVector<T>
where
    Self: IntoIterator<IntoIter = I, Item = (usize, T)>,
    T: Copy + Signed + Add<T, Output = T> + Mul<T, Output = T> + Zero,
    V: IntoIterator<IntoIter = J, Item = (usize, T)>,
    I: OrderedMapIterator<Key = usize, Val = T>,
    J: OrderedMapIterator<Key = usize, Val = T>,
{
    type Scalar = T;

    fn squared_distance(self, rhs: V) -> Self::Scalar {
        squared_distance_sparse!(T => (self, rhs))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn squared_distance() {
        let subject = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);
        let squared_distance = subject.squared_distance(&other);
        expect!(squared_distance).to(be_close_to(13.76));
    }

    #[test]
    fn distance() {
        let subject = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);
        let distance = subject.distance(&other);
        expect!(distance).to(be_close_to(3.71));
    }
}
