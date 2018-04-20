use std::ops::{Add, Mul};

use num_traits::Zero;
use ordered_iter::OrderedMapIterator;

use Dot;
use super::SparseVector;

impl<'a, T, V, I, J> Dot<V> for &'a SparseVector<T>
where
    Self: IntoIterator<IntoIter = I, Item = (usize, T)>,
    T: Add<T, Output = T> + Mul<T, Output = T> + Zero,
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
        let subject = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);
        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.85));
    }
}
