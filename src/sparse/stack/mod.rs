// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Sparse stack-allocated vector representation.

use std::fmt;
use std::iter::{IntoIterator, FromIterator};
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::cmp::Ordering;

#[cfg(feature = "use-specialization")]
use num_traits::Signed;
use num_traits::{NumAssign, Zero, MulAdd, MulAddAssign};
use ordered_iter::OrderedMapIterator;
use arrayvec::{Array, ArrayVec};

use {Vector, VectorExt, VectorOps, VectorAssignOps};

mod add;
mod sub;
mod mul;
mod div;
mod mul_add;

mod debug;
mod iter;

pub use self::iter::{Iter, IntoIter};

/// A sparse stack-allocated multi-dimensional vector.
pub struct SparseVector<A>
where
    A: Array,
{
    components: ArrayVec<A>,
}

impl<T, A> SparseVector<A>
where
    A: Array<Item = (usize, T)>,
{
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

impl<T, A> Clone for SparseVector<A>
where
    T: Clone,
    A: Array<Item = (usize, T)>,
{
    fn clone(&self) -> Self {
        let components = self.components.clone();
        Self { components }
    }
}

impl<T, A> PartialEq for SparseVector<A>
where
    T: PartialEq,
    A: Array<Item = (usize, T)>,
{
    fn eq(&self, other: &Self) -> bool {
        self.components.eq(&other.components)
    }
}

impl<T, A> From<A> for SparseVector<A>
where
    A: Array<Item = (usize, T)>,
{
    #[inline]
    fn from(items: A) -> Self {
        Self { components: ArrayVec::from(items) }
    }
}

impl<V, T, A> VectorOps<V, T> for SparseVector<A>
where
    Self: Add<V, Output = Self> + Sub<V, Output = Self> + Mul<T, Output = Self> + Div<T, Output = Self> + MulAdd<T, V, Output = Self>,
    T: Copy + NumAssign + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
{}

impl<V, T, A> VectorAssignOps<V, T> for SparseVector<A>
where
    Self: AddAssign<V> + SubAssign<V> + MulAssign<T> + DivAssign<T> + MulAddAssign<T, V>,
    T: Copy + NumAssign + MulAddAssign,
    A: Array<Item = (usize, T)>,
{}

impl<T, A> Vector<T> for SparseVector<A>
where
    Self: PartialEq + VectorOps<Self, T>,
    T: Copy + PartialOrd + NumAssign + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    type Scalar = T;
}

#[cfg(feature = "use-specialization")]
default impl<T, A> VectorExt<T> for SparseVector<A>
where
    Self: PartialEq + Vector<T, Scalar = T>,
    T: Copy + PartialOrd + NumAssign + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        dot!(T => (self, rhs))
    }

    fn squared_distance(&self, rhs: &Self) -> Self::Scalar {
        squared_distance_generic!(T => (self, rhs))
    }
}

#[cfg(not(feature = "use-specialization"))]
impl<T, A> VectorExt<T> for SparseVector<A>
where
    Self: PartialEq + Vector<T, Scalar = T>,
    T: Copy + PartialOrd + NumAssign + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        dot!(T => (self, rhs))
    }

    fn squared_distance(&self, rhs: &Self) -> Self::Scalar {
        squared_distance_generic!(T => (self, rhs))
    }
}

#[cfg(feature = "use-specialization")]
impl<T, A> VectorExt<T> for SparseVector<A>
where
    Self: PartialEq + Vector<T, Scalar = T>,
    T: Copy + PartialOrd + Signed + NumAssign + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
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
    fn sparse_vec() {
        const VALUES: [(usize, f32); 5] = [(0, 0.0), (1, 1.0), (2, 0.5), (4, 0.25), (8, 0.125)];
        let subject = SparseVector::from([(0, 0.0), (1, 1.0), (2, 0.5), (4, 0.25), (8, 0.125)]);
        let expected = ArrayVec::from(VALUES);
        expect!(subject.components).to(be_equal_to(expected));
    }

    #[test]
    fn from() {
        const VALUES: [(usize, f32); 5] = [(0, 0.0), (1, 1.0), (2, 0.5), (4, 0.25), (8, 0.125)];
        let subject = SparseVector::from(VALUES.clone());
        let expected = ArrayVec::from(VALUES);
        expect!(subject.components).to(be_equal_to(expected));
    }

    #[test]
    fn dot() {
        let subject = SparseVector::from([(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from([(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);
        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.85));
    }

    #[test]
    fn squared_distance() {
        let subject = SparseVector::from([(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from([(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);
        let squared_distance = subject.squared_distance(&other);
        expect!(squared_distance).to(be_close_to(13.76));
    }

    #[test]
    fn distance() {
        let subject = SparseVector::from([(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from([(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);
        let distance = subject.distance(&other);
        expect!(distance).to(be_close_to(3.71));
    }
}
