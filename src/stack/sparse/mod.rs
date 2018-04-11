// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::iter::{IntoIterator, FromIterator};
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

use num_traits::{Num, NumAssign, Zero, MulAdd, MulAddAssign};
use arrayvec::{Array, ArrayVec};

use {Dot, Vector, VectorOps, VectorAssignOps};

#[macro_export]
macro_rules! sparse_vec {
    ($(($i:expr, $v:expr)),*) => (SparseVector::from([$(($i, $v)),*]));
    ($(($i:expr, $v:expr)),+,) => (SparseVector::from([$(($i, $v)),+]));
}

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
pub struct SparseVector<A>
where
    A: Array,
{
    components: ArrayVec<A>,
}

impl<T, A> SparseVector<A>
where
    T: Copy,
    A: Array<Item = (usize, T)>,
{
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self.components.iter())
    }

    // #[cfg(feature = "drain_filter")]
    // fn shrink_to_fit(&mut self) {
    //     self.components.drain_filter(|(i, v)| v.is_zero()).drain();
    // }
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
    T: Copy,
    A: Array<Item = (usize, T)>,
{
    #[inline]
    fn from(items: A) -> Self {
        Self { components: ArrayVec::from(items) }
    }
}

impl<'a, T, A> VectorOps<'a, T> for SparseVector<A>
where
    Self: 'a + Sized
    + Add<&'a Self, Output = Self>
    + Sub<&'a Self, Output = Self>
    + Mul<T, Output = Self>
    + Div<T, Output = Self>
    + MulAdd<T, &'a Self, Output = Self>,
    T: 'a + Copy + NumAssign + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
{}

impl<'a, T, A> VectorAssignOps<'a, T> for SparseVector<A>
where
    Self: 'a + Sized
    + AddAssign<&'a Self>
    + SubAssign<&'a Self>
    + MulAssign<T>
    + DivAssign<T>
    + MulAddAssign<T, &'a Self>,
    T: 'a + Copy + NumAssign + MulAddAssign,
    A: Array<Item = (usize, T)>,
{}

impl<'a, T, A> Vector<'a, T> for SparseVector<A>
where
    Self: 'a + VectorOps<'a, T> + MulAdd<T, &'a Self, Output = Self> + Dot,
    T: 'a + Copy + NumAssign + MulAdd<T, T, Output = T>,
    A: Array<Item = (usize, T)>,
{
    type Scalar = T;
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn sparse_vec() {
        const VALUES: [(usize, f32); 5] = [(0, 0.0), (1, 1.0), (2, 0.5), (4, 0.25), (8, 0.125)];
        let subject = sparse_vec![(0, 0.0), (1, 1.0), (2, 0.5), (4, 0.25), (8, 0.125)];
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
}
