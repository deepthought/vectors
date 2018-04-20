use num_traits::Signed;

use Distance;
use super::DenseVector;

impl<'a, T, V, I, J> Distance<V> for &'a DenseVector<T>
where
    Self: IntoIterator<IntoIter = I, Item = (usize, T)>,
    T: Copy + Signed,
    V: IntoIterator<IntoIter = J, Item = (usize, T)>,
    I: ExactSizeIterator<Item = (usize, T)>,
    J: ExactSizeIterator<Item = (usize, T)>,
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
