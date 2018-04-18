// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Dense heap-allocated vector representation.

use std::fmt;
use std::iter::FromIterator;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::cmp::Ordering;

use num_traits::{ NumAssign, MulAdd, MulAddAssign};

use {Vector, VectorExt, VectorOps, VectorAssignOps};

mod add;
mod sub;
mod mul;
mod div;
mod mul_add;

mod debug;
mod iter;

pub use self::iter::{Iter, IntoIter};

/// A dense heap-allocated multi-dimensional vector.
#[derive(Clone, PartialEq)]
pub struct DenseVector<T> {
    components: Vec<T>,
}

impl<T> DenseVector<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.components.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self.components.iter())
    }
}

impl<T> From<Vec<T>> for DenseVector<T> {
    #[inline]
    fn from(items: Vec<T>) -> Self {
        Self { components: items }
    }
}

impl<V, T> VectorOps<V, T> for DenseVector<T>
where
    Self: Add<V, Output = Self> + Sub<V, Output = Self> + Mul<T, Output = Self> + Div<T, Output = Self> + MulAdd<T, V, Output = Self>,
    T: Copy + NumAssign + MulAdd<T, T, Output = T>,
{}

impl<V, T> VectorAssignOps<V, T> for DenseVector<T>
where
    Self: AddAssign<V> + SubAssign<V> + MulAssign<T> + DivAssign<T> + MulAddAssign<T, V>,
    T: Copy + NumAssign + MulAddAssign,
{}

impl<T> Vector<T> for DenseVector<T>
where
    Self: PartialEq + VectorOps<Self, T>,
    T: Copy + PartialOrd + NumAssign + MulAdd<T, T, Output = T>,
{
    type Scalar = T;
}

impl<T> VectorExt<T> for DenseVector<T>
where
    Self: PartialEq + Vector<T, Scalar = T>,
    T: Copy + PartialOrd + NumAssign + MulAdd<T, T, Output = T>,
{
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.components.iter()
            .zip(rhs.components.iter())
            .fold(T::zero(),
                  |sum, (lhs, rhs)| sum + ((*lhs) * (*rhs)))
    }

    fn squared_distance(&self, rhs: &Self) -> Self::Scalar {
        self.components.iter()
            .zip(rhs.components.iter())
            .fold(T::zero(),
                  |sum, (lhs, rhs)| {
                      // We might be dealing with an unsigned scalar type.
                      // As such just doing `lhs - rhs` might lead to underfows:
                      let delta = match lhs.partial_cmp(rhs) {
                          Some(Ordering::Less) => (*rhs) - (*lhs),
                          Some(Ordering::Equal) => T::zero(),
                          Some(Ordering::Greater) => (*lhs) - (*rhs),
                          None => T::zero(),
                      };
                      sum + (delta * delta)
                  })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn from() {
        let values: Vec<_> = vec![0.0; 5];
        let subject = DenseVector::from(values.clone());
        expect!(subject.components).to(be_equal_to(values));
    }

    #[test]
    fn dot() {
        let subject = DenseVector::from(vec![0.0, 0.5, 1.0, 2.0, 4.0]);
        let other = DenseVector::from(vec![0.1, 0.2, 0.3, 0.4, 0.0]);
        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.2));
    }

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
