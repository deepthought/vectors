// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// use std::cmp::max;
use std::fmt;
use std::iter::{IntoIterator, FromIterator};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use num_traits::{Num, NumAssign, Zero, MulAdd, MulAddAssign};

use {Dot, Vector, VectorOps, VectorAssignOps};

#[macro_export]
macro_rules! dense_vec {
    ($e:expr; $n:expr) => (DenseVector::from_iter(vec![$e; $n]));
    ($($e:expr),*) => (DenseVector::from_iter(vec![$($e),*]));
    ($($e:expr),+,) => (dense_vec!($($e),+));
}

mod add;
mod sub;
mod mul;
mod div;
mod mul_add;

mod dot;

mod debug;
mod iter;

pub use dense::iter::{Iter, IntoIter};

#[macro_export]
macro_rules! dense_vec {
    ($e:expr; $n:expr) => (DenseVector::from_iter(vec![$e; $n]));
    ($($e:expr),*) => (DenseVector::from_iter(vec![$($e),*]));
    ($($e:expr),+,) => (dense_vec!($($e),+));
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Item<T>(pub T);

/// A dense vector representation with efficient iteration.
#[derive(Clone, PartialEq)]
pub struct DenseVector<T>(Vec<Item<T>>);

impl<T> DenseVector<T> {
    #[inline]
    pub fn new(items: Vec<Item<T>>) -> Self {
        DenseVector(items)
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

impl<T> Default for DenseVector<T> {
    #[inline]
    fn default() -> Self {
        Self::from(vec![])
    }
}

impl<T> From<Vec<Item<T>>> for DenseVector<T> {
    #[inline]
    fn from(items: Vec<Item<T>>) -> Self {
        DenseVector(items)
    }
}

impl<'a, T> VectorOps<'a, T> for DenseVector<T>
where
    Self: 'a + VectorAssignOps<'a, T> + MulAdd<T, &'a Self, Output = Self>,
    T: Clone + Default + NumAssign + MulAdd<Output = T>,
{}

impl<'a, T> VectorAssignOps<'a, T> for DenseVector<T>
where
    T: 'a + Clone + Default + NumAssign + MulAddAssign,
{}

impl<'a, T> Vector<'a, T> for DenseVector<T>
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
            $vec.into_iter().map(|v| Item(v)).collect()
        };
    }

    #[test]
    fn dense_vec() {
        let (value, count) = (0.0, 5);
        let values = vec![value; count];
        let items: Vec<_> = itemize!(values.clone());
        let subject = dense_vec![value; count];
        expect!(subject.0).to(be_equal_to(items));
    }

    #[test]
    fn from() {
        let items: Vec<_> = itemize!(vec![0.0; 5]);
        let subject = DenseVector::from(items.clone());
        expect!(subject.0).to(be_equal_to(items));
    }
}
