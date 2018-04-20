// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Dense stack-allocated vector representation.

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use num_traits::{NumAssign, MulAdd, MulAddAssign};
use arrayvec::{Array, ArrayVec};

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

/// A dense stack-allocated multi-dimensional vector.
pub struct DenseVector<A>
where
    A: Array,
{
    components: ArrayVec<A>,
}

impl<T, A> DenseVector<A>
where
    A: Array<Item = T>,
{
    /// The number of components in `self`
    #[inline]
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// `true` if `self.len() == 0`, otherwise `false`
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// A borrowing iterator over `self`
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(&self.components[..])
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
    A: Array<Item = T>,
{
    #[inline]
    fn from(items: A) -> Self {
        Self { components: ArrayVec::from(items) }
    }
}

impl<T, A> From<ArrayVec<A>> for DenseVector<A>
where
    A: Array<Item = T>,
{
    #[inline]
    fn from(items: ArrayVec<A>) -> Self {
        Self { components: items }
    }
}

impl<V, T, A> VectorOps<V, T> for DenseVector<A>
where
    Self: Add<V, Output = Self> + Sub<V, Output = Self> + Mul<T, Output = Self> + Div<T, Output = Self> + MulAdd<T, V, Output = Self>,
    T: Copy + NumAssign + MulAdd<T, T, Output = T>,
    A: Copy + Array<Item = T>,
{}

impl<V, T, A> VectorAssignOps<V, T> for DenseVector<A>
where
    Self: AddAssign<V> + SubAssign<V> + MulAssign<T> + DivAssign<T> + MulAddAssign<T, V>,
    T: Copy + NumAssign + MulAddAssign,
    A: Copy + Array<Item = T>,
{}

impl<T, A> Vector<T> for DenseVector<A>
where
    Self: VectorOps<Self, T>,
    T: Copy + NumAssign + MulAdd<T, T, Output = T>,
    A: Copy + Array<Item = T>,
{
    type Scalar = T;
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn from() {
        const VALUES: [f32; 5] = [0.0, 1.0, 0.5, 0.25, 0.125];
        let subject = DenseVector::from(VALUES.clone());
        let expected = ArrayVec::from(VALUES);
        expect!(subject.components).to(be_equal_to(expected));
    }
}
