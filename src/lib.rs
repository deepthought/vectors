// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//!

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(test), feature(lang_items))]

#[cfg(not(feature = "std"))]
extern crate core as std;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate std;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

extern crate num_traits;
extern crate ordered_iter;
extern crate arrayvec;

pub mod stack;

#[cfg(feature = "std")]
pub mod heap;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use num_traits::{MulAdd, MulAddAssign};

pub trait Dot {
    type Output;

    fn dot(&self, rhs: &Self) -> Self::Output;
}

pub trait VectorOps<'a, Scalar>: 'a + Sized
    + Add<&'a Self, Output = Self>
    + Sub<&'a Self, Output = Self>
    + Mul<Scalar, Output = Self>
    + Div<Scalar, Output = Self>
    + MulAdd<Scalar, &'a Self, Output = Self>
{}

pub trait VectorAssignOps<'a, Scalar>: 'a + Sized
    + AddAssign<&'a Self>
    + SubAssign<&'a Self>
    + MulAssign<Scalar>
    + DivAssign<Scalar>
    + MulAddAssign<Scalar, &'a Self>
{}

pub trait Vector<'a, Scalar>: PartialEq + Dot + VectorOps<'a, Scalar> {
    type Scalar;
}

pub trait VectorAssign<'a, Scalar>: Vector<'a, Scalar> + VectorAssignOps<'a, Scalar> {}

impl<'a, T, S> VectorAssign<'a, S> for T
where
    T: 'a + Vector<'a, S> + VectorAssignOps<'a, S>
{}
