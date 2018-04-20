// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Sparse heap-allocated vector representation.

use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

use num_traits::{NumAssign, MulAdd, MulAddAssign};

use {Vector, VectorOps, VectorAssignOps};

mod add;
mod sub;
mod mul;
mod div;
mod mul_add;

mod dot;
mod distance;

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
    Self: VectorOps<Self, T>,
    T: Copy + PartialOrd + NumAssign + MulAdd<T, T, Output = T>,
{
    type Scalar = T;
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
}
