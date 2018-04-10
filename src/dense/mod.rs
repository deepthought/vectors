// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// use std::cmp::max;
use std::fmt;
use std::iter::FromIterator;
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

/// A dense vector representation with efficient iteration.
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
        self.len() == 0
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self.components.iter())
    }
}

impl<T> Default for DenseVector<T> {
    #[inline]
    fn default() -> Self {
        Self::from(vec![])
    }
}

impl<T> From<Vec<T>> for DenseVector<T> {
    #[inline]
    fn from(items: Vec<T>) -> Self {
        DenseVector { components: items }
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

    use std::iter::FromIterator;

    use expectest::prelude::*;

    #[test]
    fn dense_vec() {
        let (value, count) = (0.0, 5);
        let values = vec![value; count];
        let subject = dense_vec![value; count];
        expect!(subject.components).to(be_equal_to(values));
    }

    #[test]
    fn from() {
        let values: Vec<_> = vec![0.0; 5];
        let subject = DenseVector::from(values.clone());
        expect!(subject.components).to(be_equal_to(values));
    }
}
