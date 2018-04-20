use std::ops::{Add, Mul};

use num_traits::Zero;

use Dot;
use super::DenseVector;

impl<'a, T, V> Dot<V> for &'a DenseVector<T>
where
    Self: IntoIterator<Item=(usize, T)>,
    T: Copy + Add<T, Output = T> + Mul<T, Output = T> + Zero,
    V: IntoIterator<Item=(usize, T)>,
    <V as IntoIterator>::IntoIter: ExactSizeIterator,
{
    type Scalar = T;

    fn dot(self, rhs: V) -> Self::Scalar {
        dot!(T => (self, rhs))
    }
}
