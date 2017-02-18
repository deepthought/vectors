// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//!

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

extern crate ordered_iter;

#[macro_use]
mod dense;

#[macro_use]
mod sparse;

pub use dense::DenseVector;
pub use sparse::SparseVector;

pub trait AddScaled<Rhs = Self, S = f64> {
    type Output;
    fn add_scaled(self, rhs: Rhs, scale: S) -> Self::Output;
}

pub trait AddAssignScaled<Rhs = Self, S = f64> {
    fn add_assign_scaled(&mut self, rhs: Rhs, scale: S);
}

pub trait Dot<Rhs = Self> {
    fn dot(&self, rhs: Rhs) -> f64;
}
