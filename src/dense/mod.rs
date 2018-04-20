//! Dense vector representations.

#[macro_use]
mod shared;

mod iter;

pub mod stack;
#[cfg(feature = "std")]
pub mod heap;
