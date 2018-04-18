//! Sparse vector representations.

#[macro_use]
mod shared;

pub mod stack;
#[cfg(feature = "std")]
pub mod heap;
