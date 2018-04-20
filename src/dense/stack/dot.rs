use std::ops::{Add, Mul};

use num_traits::Zero;
use arrayvec::Array;

use Dot;
use super::DenseVector;

impl<'a, T, A, V> Dot<V> for &'a DenseVector<A>
where
    Self: IntoIterator<Item=(usize, T)>,
    T: Copy + Add<T, Output = T> + Mul<T, Output = T> + Zero,
    A: Array<Item = T>,
    V: IntoIterator<Item=(usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator,
{
    type Scalar = T;

    fn dot(self, rhs: V) -> Self::Scalar {
        dot!(T => (self, rhs))
    }
}
