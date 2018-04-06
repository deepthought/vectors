// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::iter::{IntoIterator, FromIterator};
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

use ordered_iter::OrderedMapIterator;

use {Dot, AddScaled, AddAssignScaled};

#[macro_export]
macro_rules! sparse_vec {
    ($(($i:expr, $v:expr)),*) => (SparseVector::from_iter(vec![$(($i, $v)),*]));
    ($(($i:expr, $v:expr)),+,) => (sparse_vec!($($e),+));
}

#[derive(Clone, PartialEq, Debug)]
pub struct Item<T>(pub (usize, T));

/// A sparse vector representation with efficient iteration.
#[derive(Clone, PartialEq)]
pub struct SparseVector<T>(Vec<Item<T>>);

impl<T> SparseVector<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter(self.0.iter())
    }
}

impl<T> Default for SparseVector<T> {
    #[inline]
    fn default() -> Self {
        Self::from(vec![])
    }
}

impl<T> From<Vec<Item<T>>> for SparseVector<T> {
    #[inline]
    fn from(items: Vec<Item<T>>) -> Self {
        SparseVector(items)
    }
}

impl<T> FromIterator<(usize, T)> for SparseVector<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let items: Vec<_> = iter.into_iter().map(Item).collect();
        SparseVector::from(items)
    }
}

impl<T> IntoIterator for SparseVector<T> {
    type Item = (usize, T);
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

impl<'b, T, U> Dot<&'b SparseVector<U>> for SparseVector<T>
    where T: Clone + Into<f64>,
          U: Clone + Into<f64>
{
    fn dot(&self, rhs: &'b SparseVector<U>) -> f64 {
        let iter = rhs.iter().map(|(k, v)| (k, v.clone())).ordered_map_iterator();
        self.iter()
            .inner_join_map(iter)
            .fold(0.0f64,
                  |sum, (_, (lhs, rhs))| sum + (lhs.clone().into() * rhs.clone().into()))
            .into()
    }
}

impl<'b, T, U> AddScaled<&'b SparseVector<U>, f64> for SparseVector<T>
    where T: Clone + Default + PartialEq + Add<T, Output = T> + From<f64>,
          U: Clone + Into<f64>
{
    type Output = SparseVector<T>;

    fn add_scaled(mut self, rhs: &'b SparseVector<U>, scale: f64) -> Self::Output {
        self.add_assign_scaled(rhs, scale);
        self
    }
}

impl<'a, 'b, T, U> AddScaled<&'b SparseVector<U>, f64> for &'a SparseVector<T>
    where T: Clone + Default + PartialEq + Add<T, Output = T> + From<f64>,
          U: Clone + Into<f64>
{
    type Output = SparseVector<T>;

    fn add_scaled(self, rhs: &'b SparseVector<U>, scale: f64) -> Self::Output {
        self.clone().add_scaled(rhs, scale)
    }
}

impl<'b, T, U> AddAssignScaled<&'b SparseVector<U>, f64> for SparseVector<T>
    where T: Clone + Default + PartialEq + Add<T, Output = T> + From<f64>,
          U: Clone + Into<f64>
{
    fn add_assign_scaled(&mut self, rhs: &'b SparseVector<U>, scale: f64) {
        let zero = T::default();
        self.0 = {
            let iter = rhs.iter().ordered_map_iterator();
            let outer_join = self.iter().outer_join(iter);
            outer_join.filter_map(|(index, (lhs, rhs))| {
                    let value = match (lhs, rhs) {
                        (Some(l), Some(r)) => l + (r.into() * scale).into(),
                        (Some(l), None) => l,
                        (None, Some(r)) => (r.into() * scale).into(),
                        _ => unreachable!(),
                    };
                    if value == zero {
                        None
                    } else {
                        Some(Item((index, value)))
                    }
                })
                .collect()
        }
    }
}

impl<'b, T, U> Add<&'b SparseVector<U>> for SparseVector<T>
    where T: Clone + Default + PartialEq + Add<T, Output = T>,
          U: Clone + Into<T>
{
    type Output = SparseVector<T>;

    #[inline]
    fn add(mut self, rhs: &'b SparseVector<U>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a, 'b, T, U> Add<&'b SparseVector<U>> for &'a SparseVector<T>
    where T: Clone + Default + PartialEq + Add<T, Output = T>,
          U: Clone + Into<T>
{
    type Output = SparseVector<T>;

    #[inline]
    fn add(self, rhs: &'b SparseVector<U>) -> Self::Output {
        self.clone() + rhs
    }
}

impl<'b, T, U> AddAssign<&'b SparseVector<U>> for SparseVector<T>
    where T: Clone + Default + PartialEq + Add<T, Output = T>,
          U: Clone + Into<T>
{
    fn add_assign(&mut self, rhs: &'b SparseVector<U>) {
        let zero = T::default();
        self.0 = {
            let iter = rhs.iter().ordered_map_iterator();
            let outer_join = self.iter().outer_join(iter);
            outer_join.filter_map(|(index, (lhs, rhs))| {
                    let value = match (lhs, rhs) {
                        (Some(l), Some(r)) => l + r.into(),
                        (Some(l), None) => l,
                        (None, Some(r)) => r.into(),
                        _ => unreachable!(),
                    };
                    if value == zero {
                        None
                    } else {
                        Some(Item((index, value)))
                    }
                })
                .collect()
        }
    }
}

impl<'b, T, U> Sub<&'b SparseVector<U>> for SparseVector<T>
    where T: Clone + Default + PartialEq + Sub<T, Output = T>,
          U: Clone + Into<T>
{
    type Output = SparseVector<T>;

    #[inline]
    fn sub(mut self, rhs: &'b SparseVector<U>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<'a, 'b, T, U> Sub<&'b SparseVector<U>> for &'a SparseVector<T>
    where T: Clone + Default + PartialEq + Sub<T, Output = T>,
          U: Clone + Into<T>
{
    type Output = SparseVector<T>;

    #[inline]
    fn sub(self, rhs: &'b SparseVector<U>) -> Self::Output {
        self.clone() - rhs
    }
}

impl<'b, T, U> SubAssign<&'b SparseVector<U>> for SparseVector<T>
    where T: Clone + Default + PartialEq + Sub<T, Output = T>,
          U: Clone + Into<T>
{
    fn sub_assign(&mut self, rhs: &'b SparseVector<U>) {
        let zero = T::default();
        self.0 = {
            let iter = rhs.iter().ordered_map_iterator();
            let outer_join = self.iter().outer_join(iter);
            outer_join.filter_map(|(index, (lhs, rhs))| {
                    let value = match (lhs, rhs) {
                        (Some(l), Some(r)) => l - r.into(),
                        (Some(l), None) => l,
                        (None, Some(r)) => r.into(),
                        _ => unreachable!(),
                    };
                    if value == zero {
                        None
                    } else {
                        Some(Item((index, value)))
                    }
                })
                .collect()
        };
    }
}

impl<T, U> Mul<U> for SparseVector<T>
    where T: Clone + Default + PartialEq + Mul<T, Output = T>,
          U: Into<T>
{
    type Output = SparseVector<T>;

    fn mul(self, rhs: U) -> Self::Output {
        let zero = T::default();
        let rhs_as_t = rhs.into();
        SparseVector::from_iter(self.into_iter().filter_map(|(index, value)| {
            let new_value = value * rhs_as_t.clone();
            if new_value == zero {
                None
            } else {
                Some((index, new_value))
            }
        }))
    }
}

impl<'a, T, U> Mul<U> for &'a SparseVector<T>
    where T: Clone + Default + PartialEq + Mul<T, Output = T>,
          U: Into<T>
{
    type Output = SparseVector<T>;

    #[inline]
    fn mul(self, rhs: U) -> Self::Output {
        self.clone() * rhs
    }
}

impl<T, U> MulAssign<U> for SparseVector<T>
    where T: Clone + Default + PartialEq + MulAssign<T>,
          U: Into<T>
{
    fn mul_assign(&mut self, rhs: U) {
        let into: T = rhs.into();
        for lhs in &mut self.0 {
            (lhs.0).1 *= into.clone();
        }
    }
}

impl<T, U> Div<U> for SparseVector<T>
    where T: Clone + Default + PartialEq + Div<T, Output = T>,
          U: Into<T>
{
    type Output = SparseVector<T>;

    fn div(self, rhs: U) -> Self::Output {
        let zero = T::default();
        let rhs_as_t = rhs.into();
        SparseVector::from_iter(self.into_iter().filter_map(|(index, value)| {
            let new_value = value / rhs_as_t.clone();
            if new_value == zero {
                None
            } else {
                Some((index, new_value))
            }
        }))
    }
}

impl<'a, T, U> Div<U> for &'a SparseVector<T>
    where T: Clone + Default + PartialEq + Div<T, Output = T>,
          U: Into<T>
{
    type Output = SparseVector<T>;

    #[inline]
    fn div(self, rhs: U) -> Self::Output {
        self.clone() / rhs
    }
}

impl<T, U> DivAssign<U> for SparseVector<T>
    where T: Clone + Default + PartialEq + DivAssign<T>,
          U: Into<T>
{
    fn div_assign(&mut self, rhs: U) {
        let into: T = rhs.into();
        for lhs in &mut self.0 {
            (lhs.0).1 /= into.clone();
        }
    }
}

impl<T> fmt::Debug for SparseVector<T>
    where T: Clone + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "[");
        for (fmt_idx, (index, value)) in self.iter().enumerate() {
            try! {
                if fmt_idx > 0 { write!(f, ", ({}, {:?})", index, value.clone()) }
                else { write!(f, "({}, {:?})", index, value.clone()) }
            };
        }
        let _ = write!(f, "]");
        Ok(())
    }
}

struct OrderedMapIteratorWrapper<I>(I);

impl<I> Iterator for OrderedMapIteratorWrapper<I>
    where I: Iterator
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<I, K, V> OrderedMapIterator for OrderedMapIteratorWrapper<I>
    where I: Iterator<Item = (K, V)>
{
    type Key = K;
    type Val = V;
}

trait OrderedMapIterable: Sized {
    #[inline]
    fn ordered_map_iterator(self) -> OrderedMapIteratorWrapper<Self> {
        OrderedMapIteratorWrapper(self)
    }
}

impl<I, K, V> OrderedMapIterable for I where I: Iterator<Item = (K, V)> {}

pub struct IntoIter<T>(<Vec<Item<T>> as IntoIterator>::IntoIter);

impl<T> Iterator for IntoIter<T> {
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| i.0)
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> OrderedMapIterator for IntoIter<T> {
    type Key = usize;
    type Val = T;
}

pub struct Iter<'a, T>(<&'a [Item<T>] as IntoIterator>::IntoIter) where T: 'a;

impl<'a, T> Iterator for Iter<'a, T>
    where T: Clone
{
    type Item = (usize, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| i.0.clone())
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T>
    where T: Clone
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> OrderedMapIterator for Iter<'a, T>
    where T: Clone
{
    type Key = usize;
    type Val = T;
}

#[cfg(test)]
mod test {
    use expectest::prelude::*;

    use super::*;

    use std::iter::{IntoIterator, FromIterator};

    use {Dot, AddScaled};

    macro_rules! itemize {
        ($vec:expr) => {
            $vec.into_iter().map(|(i, v)| Item((i, v))).collect()
        };
    }

    #[test]
    fn sparse_vec() {
        let values = vec![(0, 5.0)];
        let items: Vec<_> = itemize!(values.clone());
        let subject = sparse_vec![(0, 5.0)];
        expect!(subject.0).to(be_equal_to(items));
    }

    #[test]
    fn from() {
        let items: Vec<_> = itemize!(vec![(0, 5.0)]);
        let subject = SparseVector::from(items.clone());
        expect!(subject.0).to(be_equal_to(items));
    }

    #[test]
    fn from_iter() {
        let values = vec![(0, 0.1), (1, 0.2), (2, 0.3), (4, 0.4), (5, 0.5)];
        let items: Vec<_> = itemize!(values.clone());
        let subject = SparseVector::from_iter(values.clone());
        expect!(subject.0).to(be_equal_to(items));
    }

    #[test]
    fn into_iter() {
        let values = vec![(0, 0.1), (1, 0.2), (2, 0.3), (4, 0.4), (5, 0.5)];
        let subject = SparseVector::from_iter(values.clone());
        let output: Vec<_> = subject.into_iter().collect();
        expect!(output).to(be_equal_to(values));
    }

    #[test]
    fn dot() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)];
        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.85));
    }

    #[test]
    fn add_scaled() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)];
        let expected = sparse_vec![(0, 0.2), (1, 0.7), (2, 1.4), (3, 0.6), (4, 2.0), (5, 4.8)];
        let result = subject.add_scaled(&other, 2.0);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_scaled_from_ref() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)];
        let expected = sparse_vec![(0, 0.2), (1, 0.7), (2, 1.4), (3, 0.6), (4, 2.0), (5, 4.8)];
        let result = (&subject).add_scaled(&other, 2.0);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)];
        let expected = sparse_vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)];
        let result = subject + &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_from_ref() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)];
        let expected = sparse_vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)];
        let result = (&subject) + &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub() {
        let subject = sparse_vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)];
        let expected = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let result = subject - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_from_ref() {
        let subject = sparse_vec![(0, 0.2), (1, 0.6), (2, 1.2), (3, 0.3), (4, 2.0), (5, 4.4)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4)];
        let expected = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let result = (&subject) - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let expected = sparse_vec![(0, 0.4), (1, 1.0), (2, 2.0), (4, 4.0), (5, 8.0)];
        let result = subject * 2.0;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul_from_ref() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let expected = sparse_vec![(0, 0.4), (1, 1.0), (2, 2.0), (4, 4.0), (5, 8.0)];
        let result = (&subject) * 2.0;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn div() {
        let subject = sparse_vec![(0, 0.4), (1, 1.0), (2, 2.0), (4, 4.0), (5, 8.0)];
        let expected = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let result = subject / 2.0;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn div_from_ref() {
        let subject = sparse_vec![(0, 0.4), (1, 1.0), (2, 2.0), (4, 4.0), (5, 8.0)];
        let expected = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let result = (&subject) / 2.0;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn debug() {
        let v = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0)];
        expect!(format!("{:?}", v)).to(be_equal_to("[(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0)]"));
    }
}
