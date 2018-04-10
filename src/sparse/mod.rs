// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::iter::{IntoIterator, FromIterator};
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

use num_traits::{Num, NumAssign, Zero, MulAdd, MulAddAssign};

use {Dot, Vector, VectorOps, VectorAssignOps};

#[macro_export]
macro_rules! sparse_vec {
    ($(($i:expr, $v:expr)),*) => (SparseVector::from_iter(vec![$(($i, $v)),*]));
    ($(($i:expr, $v:expr)),+,) => (sparse_vec!($($e),+));
}

mod add;
mod sub;
mod mul;
mod div;
mod mul_add;

mod dot;

mod debug;
mod iter;

pub use sparse::iter::{Iter, IntoIter};

#[derive(Clone, PartialEq, Debug)]
pub struct Item<T>(pub (usize, T));

/// A sparse vector representation with efficient iteration.
#[derive(Clone, PartialEq)]
pub struct SparseVector<T>(Vec<Item<T>>);

impl<T> SparseVector<T> {
    #[inline]
    pub fn new(items: Vec<Item<T>>) -> Self {
        SparseVector(items)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self.0.iter())
    }
}

impl<T> Default for SparseVector<T> {
    #[inline]
    fn default() -> Self {
        Self::from(vec![])
    }
}

impl<T> From<Vec<Item<T>>> for SparseVector<T> {
    #[inline]
    fn from(items: Vec<Item<T>>) -> Self {
        SparseVector(items)
    }
}

impl<'a, T> VectorOps<'a, T> for SparseVector<T>
where
    Self: 'a + Sized
    + Add<&'a Self, Output = Self>
    + Sub<&'a Self, Output = Self>
    + Mul<T, Output = Self>
    + Div<T, Output = Self>
    + MulAdd<T, &'a Self, Output = Self>,
    T: Clone + Default + NumAssign + MulAdd<Output = T>,
{}

impl<'a, T> VectorAssignOps<'a, T> for SparseVector<T>
where
    Self: 'a + Sized
    + AddAssign<&'a Self>
    + SubAssign<&'a Self>
    + MulAssign<T>
    + DivAssign<T>
    + MulAddAssign<T, &'a Self>,
    T: 'a + Clone + Default + NumAssign + MulAddAssign,
{}

impl<'a, T> Vector<'a, T> for SparseVector<T>
where
    Self: 'a + VectorOps<'a, T> + MulAdd<T, &'a Self, Output = Self> + Dot,
    T: 'a + Clone + Default + NumAssign + MulAdd<Output = T>,
{
    type Scalar = T;
}

#[cfg(test)]
mod test {
    use super::*;

    use std::iter::{IntoIterator, FromIterator};

    use expectest::prelude::*;

    macro_rules! itemize {
        ($vec:expr) => {
            $vec.into_iter().map(|(i, v)| Item((i, v))).collect()
        };
    }

    #[test]
    fn sparse_vec() {
        let values = vec![(0, 5.0)];
        let items: Vec<_> = itemize!(values.clone());
        let subject = sparse_vec![(0, 5.0)];
        expect!(subject.0).to(be_equal_to(items));
    }

    #[test]
    fn from() {
        let items: Vec<_> = itemize!(vec![(0, 5.0)]);
        let subject = SparseVector::from(items.clone());
        expect!(subject.0).to(be_equal_to(items));
    }
}
