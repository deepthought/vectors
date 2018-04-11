// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::iter::{IntoIterator, FromIterator};
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

use num_traits::{Num, NumAssign, Zero, MulAdd, MulAddAssign};

use {Dot, Vector, VectorOps, VectorAssignOps};

mod add;
mod sub;
mod mul;
mod div;
mod mul_add;

mod dot;

mod debug;
mod iter;

pub use self::iter::{Iter, IntoIter};

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

impl<T> From<Vec<(usize, T)>> for SparseVector<T> {
    #[inline]
    fn from(items: Vec<(usize, T)>) -> Self {
        Self { components: items }
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
    T: 'a + Copy + NumAssign + MulAdd<T, T, Output = T>,
{}

impl<'a, T> VectorAssignOps<'a, T> for SparseVector<T>
where
    Self: 'a + Sized
    + AddAssign<&'a Self>
    + SubAssign<&'a Self>
    + MulAssign<T>
    + DivAssign<T>
    + MulAddAssign<T, &'a Self>,
    T: 'a + Copy + NumAssign + MulAddAssign,
{}

impl<'a, T> Vector<'a, T> for SparseVector<T>
where
    Self: 'a + VectorOps<'a, T> + MulAdd<T, &'a Self, Output = Self> + Dot,
    T: 'a + Copy + NumAssign + MulAdd<T, T, Output = T>,
{
    type Scalar = T;
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn sparse_vec() {
        let values = vec![(0, 5.0)];
        let subject = SparseVector::from(vec![(0, 5.0)]);
        expect!(subject.components).to(be_equal_to(values));
    }

    #[test]
    fn from() {
        let values: Vec<_> = vec![(0, 5.0)];
        let subject = SparseVector::from(values.clone());
        expect!(subject.components).to(be_equal_to(values));
    }
}
