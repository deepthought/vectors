// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::iter::FromIterator;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use num_traits::{Num, NumAssign, Zero, MulAdd, MulAddAssign};
use arrayvec::{Array, ArrayVec};

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

/// A dense vector representation with efficient iteration.
pub struct DenseVector<A>
where
    A: Array,
{
    components: ArrayVec<A>,
}

impl<T, A> DenseVector<A>
where
    T: Copy,
    A: Array<Item = T>,
{
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

impl<T, A> Clone for DenseVector<A>
where
    T: Clone,
    A: Array<Item = T>,
{
    fn clone(&self) -> Self {
        let components = self.components.clone();
        Self { components }
    }
}

impl<T, A> PartialEq for DenseVector<A>
where
    T: PartialEq,
    A: Array<Item = T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.components.eq(&other.components)
    }
}

impl<T, A> From<A> for DenseVector<A>
where
    T: Copy,
    A: Array<Item = T>,
{
    #[inline]
    fn from(items: A) -> Self {
        Self { components: ArrayVec::from(items) }
    }
}

impl<T, A> From<ArrayVec<A>> for DenseVector<A>
where
    T: Copy,
    A: Array<Item = T>,
{
    #[inline]
    fn from(items: ArrayVec<A>) -> Self {
        Self { components: items }
    }
}

impl<'a, T, A> VectorOps<'a, T> for DenseVector<A>
where
    Self: 'a + VectorAssignOps<'a, T> + MulAdd<T, &'a Self, Output = Self>,
    T: 'a + Copy + NumAssign + MulAdd<T, T, Output = T>,
    A: 'a + Copy + PartialEq + Array<Item = T>,
{}

impl<'a, T, A> VectorAssignOps<'a, T> for DenseVector<A>
where
    T: 'a + Copy + NumAssign + MulAddAssign,
    A: 'a + Copy + PartialEq + Array<Item = T>,
{}

impl<'a, T, A> Vector<'a, T> for DenseVector<A>
where
    Self: 'a + VectorOps<'a, T> + MulAdd<T, &'a Self, Output = Self> + Dot,
    T: 'a + Copy + NumAssign + MulAdd<T, T, Output = T>,
    A: 'a + Copy + PartialEq + Array<Item = T>,
{
    type Scalar = T;
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn dense_vec() {
        const VALUES: [f32; 5] = [0.0, 1.0, 0.5, 0.25, 0.125];
        let subject = DenseVector::from([0.0, 1.0, 0.5, 0.25, 0.125]);
        let expected = ArrayVec::from(VALUES);
        expect!(subject.components).to(be_equal_to(expected));
    }

    #[test]
    fn from() {
        const VALUES: [f32; 5] = [0.0, 1.0, 0.5, 0.25, 0.125];
        let subject = DenseVector::from(VALUES.clone());
        let expected = ArrayVec::from(VALUES);
        expect!(subject.components).to(be_equal_to(expected));
    }
}
