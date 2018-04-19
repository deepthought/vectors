// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Sparse heap-allocated vector representation.

use std::fmt;
use std::iter::{IntoIterator, FromIterator};
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::cmp::Ordering;

#[cfg(feature = "use-specialization")]
use num_traits::Signed;
use num_traits::{NumAssign, Zero, MulAdd, MulAddAssign};

use ordered_iter::OrderedMapIterator;

use {Vector, VectorExt, VectorOps, VectorAssignOps};

mod add;
mod sub;
mod mul;
mod div;
mod mul_add;

mod debug;
mod iter;

pub use self::iter::{Iter, IntoIter};

/// A sparse heap-allocated multi-dimensional vector.
#[derive(Clone, PartialEq)]
pub struct SparseVector<T> {
    components: Vec<(usize, T)>,
}

impl<T> SparseVector<T> {
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
        Iter::new(&self.components[..])
    }
}

impl<T> From<Vec<(usize, T)>> for SparseVector<T> {
    #[inline]
    fn from(items: Vec<(usize, T)>) -> Self {
        Self { components: items }
    }
}

impl<V, T> VectorOps<V, T> for SparseVector<T>
where
    Self: Add<V, Output = Self> + Sub<V, Output = Self> + Mul<T, Output = Self> + Div<T, Output = Self> + MulAdd<T, V, Output = Self>,
    T: Copy + NumAssign + MulAdd<T, T, Output = T>,
{}

impl<V, T> VectorAssignOps<V, T> for SparseVector<T>
where
    Self: AddAssign<V> + SubAssign<V> + MulAssign<T> + DivAssign<T> + MulAddAssign<T, V>,
    T: Copy + NumAssign + MulAddAssign,
{}

impl<T> Vector<T> for SparseVector<T>
where
    Self: PartialEq + VectorOps<Self, T>,
    T: Copy + PartialOrd + NumAssign + MulAdd<T, T, Output = T>,
{
    type Scalar = T;
}

#[cfg(feature = "use-specialization")]
default impl<T> VectorExt<T> for SparseVector<T>
where
    Self: PartialEq + Vector<T, Scalar = T>,
    T: Copy + PartialOrd + NumAssign + MulAdd<T, T, Output = T>,
{
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        dot!(T => (self, rhs))
    }

    fn squared_distance(&self, rhs: &Self) -> Self::Scalar {
        squared_distance_generic!(T => (self, rhs))
    }
}

#[cfg(not(feature = "use-specialization"))]
impl<T> VectorExt<T> for SparseVector<T>
where
    Self: PartialEq + Vector<T, Scalar = T>,
    T: Copy + PartialOrd + NumAssign + MulAdd<T, T, Output = T>,
{
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        dot!(T => (self, rhs))
    }

    fn squared_distance(&self, rhs: &Self) -> Self::Scalar {
        squared_distance_generic!(T => (self, rhs))
    }
}

#[cfg(feature = "use-specialization")]
impl<T> VectorExt<T> for SparseVector<T>
where
    Self: PartialEq + Vector<T, Scalar = T>,
    T: Copy + PartialOrd + Signed + NumAssign + MulAdd<T, T, Output = T>,
{
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        dot!(T => (self, rhs))
    }

    fn squared_distance(&self, rhs: &Self) -> Self::Scalar {
        squared_distance_signed!(T => (self, rhs))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn from() {
        let values: Vec<_> = vec![(0, 5.0)];
        let subject = SparseVector::from(values.clone());
        expect!(subject.components).to(be_equal_to(values));
    }

    #[test]
    fn dot() {
        let subject = SparseVector::from(vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from(vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);
        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.85));
    }

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
