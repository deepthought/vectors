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

/// A sparse vector representation with efficient iteration.
#[derive(Clone, PartialEq)]
pub struct SparseVector<T> {
    components: Vec<(usize, T)>,
}

impl<T> SparseVector<T> {
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self.components.iter())
    }

    // #[cfg(feature = "drain_filter")]
    // fn shrink_to_fit(&mut self) {
    //     self.components.drain_filter(|(i, v)| v.is_zero()).drain();
    // }
}

impl<T> Default for SparseVector<T> {
    #[inline]
    fn default() -> Self {
        Self::from(vec![])
    }
}

impl<T> From<Vec<(usize, T)>> for SparseVector<T> {
    #[inline]
    fn from(items: Vec<(usize, T)>) -> Self {
        SparseVector { components: items }
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

    use std::iter::FromIterator;

    use expectest::prelude::*;

    #[test]
    fn sparse_vec() {
        let values = vec![(0, 5.0)];
        let subject = sparse_vec![(0, 5.0)];
        expect!(subject.components).to(be_equal_to(values));
    }

    #[test]
    fn from() {
        let values: Vec<_> = vec![(0, 5.0)];
        let subject = SparseVector::from(values.clone());
        expect!(subject.components).to(be_equal_to(values));
    }
}
