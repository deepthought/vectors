use std::ops::{Add, Mul};

use num_traits::Zero;
use arrayvec::Array;
use ordered_iter::OrderedMapIterator;

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
        dot!(T => (self, rhs))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn dot() {
        let subject: SparseVector<[(usize, f32); 5]> = SparseVector::from([
            (0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)
        ]);
        let other: SparseVector<[(usize, f32); 5]> = SparseVector::from([
            (1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)
        ]);

        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.85));
    }
}