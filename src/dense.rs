// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cmp::max;
use std::fmt;
use std::iter::{IntoIterator, FromIterator};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use {Dot, AddScaled, AddAssignScaled};

#[macro_export]
macro_rules! dense_vec {
    ($e:expr; $n:expr) => (DenseVector::from_iter(vec![$e; $n]));
    ($($e:expr),*) => (DenseVector::from_iter(vec![$($e),*]));
    ($($e:expr),+,) => (dense_vec!($($e),+));
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Item<T>(pub T);

/// A dense vector representation with efficient iteration.
#[derive(Clone, PartialEq)]
pub struct DenseVector<T>(Vec<Item<T>>);

impl<T> DenseVector<T> {
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

    fn resize_if_necessary(&mut self, min_len: usize)
        where T: Clone + Default
    {
        let size = max(self.len(), min_len);
        self.0.resize(size, Item(T::default()));
    }
}

impl<T> Default for DenseVector<T> {
    #[inline]
    fn default() -> Self {
        Self::from(vec![])
    }
}

impl<T> From<Vec<Item<T>>> for DenseVector<T> {
    #[inline]
    fn from(items: Vec<Item<T>>) -> Self {
        DenseVector(items)
    }
}

impl<T> FromIterator<T> for DenseVector<T>
    where T: Clone
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let items: Vec<_> = iter.into_iter().map(Item).collect();
        DenseVector::from(items)
    }
}

impl<T> IntoIterator for DenseVector<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

impl<'b, T, U> Dot<&'b DenseVector<U>> for DenseVector<T>
    where T: Clone + Into<f64>,
          U: Clone + Into<f64>
{
    fn dot(&self, rhs: &'b DenseVector<U>) -> f64 {
        let iter = rhs.iter();
        self.iter()
            .zip(iter)
            .fold(0.0f64,
                  |sum, (lhs, rhs)| sum + (lhs.clone().into() * rhs.into()))
            .into()
    }
}

impl<'b, T, U> AddScaled<&'b DenseVector<U>, f64> for DenseVector<T>
    where T: Clone + Default + AddAssign<T> + From<f64>,
          U: Clone + Into<f64>
{
    type Output = DenseVector<T>;

    #[inline]
    fn add_scaled(mut self, rhs: &'b DenseVector<U>, scale: f64) -> Self::Output {
        self.add_assign_scaled(rhs, scale);
        self
    }
}

impl<'a, 'b, T, U> AddScaled<&'b DenseVector<U>, f64> for &'a DenseVector<T>
    where T: Clone + Default + AddAssign<T> + From<f64>,
          U: Clone + Into<f64>
{
    type Output = DenseVector<T>;

    #[inline]
    fn add_scaled(self, rhs: &'b DenseVector<U>, scale: f64) -> Self::Output {
        self.clone().add_scaled(rhs, scale)
    }
}

impl<'b, T, U> AddAssignScaled<&'b DenseVector<U>> for DenseVector<T>
    where T: Clone + Default + AddAssign<T> + From<f64>,
          U: Clone + Into<f64>
{
    fn add_assign_scaled(&mut self, rhs: &'b DenseVector<U>, scale: f64) {
        let iter = rhs.iter();
        self.resize_if_necessary(rhs.len());
        for (l, r) in self.0.iter_mut().zip(iter) {
            let scaled: T = (r.into() * scale).into();
            l.0 += scaled;
        }
    }
}

impl<'b, T, U> Add<&'b DenseVector<U>> for DenseVector<T>
    where T: Clone + Default + AddAssign<T>,
          U: Clone + Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn add(mut self, rhs: &'b DenseVector<U>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a, 'b, T, U> Add<&'b DenseVector<U>> for &'a DenseVector<T>
    where T: Clone + Default + AddAssign<T>,
          U: Clone + Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn add(self, rhs: &'b DenseVector<U>) -> Self::Output {
        self.clone() + rhs
    }
}

impl<'b, T, U> AddAssign<&'b DenseVector<U>> for DenseVector<T>
    where T: Clone + Default + AddAssign<T>,
          U: Clone + Into<T>
{
    fn add_assign(&mut self, rhs: &'b DenseVector<U>) {
        let iter = rhs.iter();
        self.resize_if_necessary(rhs.len());
        for (lhs, rhs) in self.0.iter_mut().zip(iter) {
            lhs.0 += rhs.into();
        }
    }
}

impl<'b, T, U> Sub<&'b DenseVector<U>> for DenseVector<T>
    where T: Clone + Default + SubAssign<T>,
          U: Clone + Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn sub(mut self, rhs: &'b DenseVector<U>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<'a, 'b, T, U> Sub<&'b DenseVector<U>> for &'a DenseVector<T>
    where T: Clone + Default + SubAssign<T>,
          U: Clone + Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn sub(self, rhs: &'b DenseVector<U>) -> Self::Output {
        self.clone() - rhs
    }
}

impl<'b, T, U> SubAssign<&'b DenseVector<U>> for DenseVector<T>
    where T: Clone + Default + SubAssign<T>,
          U: Clone + Into<T>
{
    fn sub_assign(&mut self, rhs: &'b DenseVector<U>) {
        let iter = rhs.iter();
        self.resize_if_necessary(rhs.len());
        for (lhs, rhs) in self.0.iter_mut().zip(iter) {
            lhs.0 -= rhs.into();
        }
    }
}

impl<T, U> Mul<U> for DenseVector<T>
    where T: Clone + Default + MulAssign<T>,
          U: Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(mut self, rhs: U) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<'a, T, U> Mul<U> for &'a DenseVector<T>
    where T: Clone + Default + MulAssign<T>,
          U: Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(self, rhs: U) -> Self::Output {
        self.clone() * rhs
    }
}

impl<T, U> MulAssign<U> for DenseVector<T>
    where T: Clone + Default + MulAssign<T>,
          U: Into<T>
{
    fn mul_assign(&mut self, rhs: U) {
        let into: T = rhs.into();
        for lhs in &mut self.0 {
            lhs.0 *= into.clone();
        }
    }
}

impl<T, U> Div<U> for DenseVector<T>
    where T: Clone + Default + DivAssign<T>,
          U: Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn div(mut self, rhs: U) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<'a, T, U> Div<U> for &'a DenseVector<T>
    where T: Clone + Default + DivAssign<T>,
          U: Into<T>
{
    type Output = DenseVector<T>;

    #[inline]
    fn div(self, rhs: U) -> Self::Output {
        self.clone() / rhs
    }
}

impl<T, U> DivAssign<U> for DenseVector<T>
    where T: Clone + Default + DivAssign<T>,
          U: Into<T>
{
    #[inline]
    fn div_assign(&mut self, rhs: U) {
        let into: T = rhs.into();
        for lhs in &mut self.0 {
            lhs.0 /= into.clone();
        }
    }
}

impl<T> fmt::Debug for DenseVector<T>
    where T: Clone + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "[");
        for (index, item) in self.iter().enumerate() {
            try! {
                if index > 0 { write!(f, ", {:?}", item) }
                else { write!(f, "{:?}", item) }
            }
        }
        let _ = write!(f, "]");
        Ok(())
    }
}

pub struct IntoIter<T>(<Vec<Item<T>> as IntoIterator>::IntoIter);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| i.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

pub struct Iter<'a, T>(<&'a [Item<T>] as IntoIterator>::IntoIter) where T: 'a;

impl<'a, T> Iterator for Iter<'a, T>
    where T: Clone
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| i.0.clone())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
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

#[cfg(test)]
mod test {
    use expectest::prelude::*;

    use super::*;

    use std::iter::{IntoIterator, FromIterator};

    use {Dot, AddScaled, AddAssignScaled};

    macro_rules! itemize {
        ($vec:expr) => {
            $vec.into_iter().map(|v| Item(v)).collect()
        };
    }

    #[test]
    fn dense_vec() {
        let (value, count) = (0.0, 5);
        let values = vec![value; count];
        let items: Vec<_> = itemize!(values.clone());
        let subject = dense_vec![value; count];
        expect!(subject.0).to(be_equal_to(items));
    }

    #[test]
    fn from() {
        let items: Vec<_> = itemize!(vec![0.0; 5]);
        let subject = DenseVector::from(items.clone());
        expect!(subject.0).to(be_equal_to(items));
    }

    #[test]
    fn from_iter() {
        let values = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let items: Vec<_> = itemize!(values.clone());
        let subject = DenseVector::from_iter(values.clone());
        expect!(subject.0).to(be_equal_to(items));
    }

    #[test]
    fn into_iter() {
        let values = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let subject = DenseVector::from_iter(values.clone());
        let output: Vec<_> = subject.into_iter().collect();
        expect!(output).to(be_equal_to(values));
    }

    #[test]
    fn dot() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 4.0];
        let other = dense_vec![0.1, 0.2, 0.3, 0.4, 0.0];
        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.2));
    }

    #[test]
    fn add_scaled() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![4.0, 2.5, 1.0, 0.0, -1.0];
        let result = subject.add_scaled(&other, 2.0);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_scaled_from_ref() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![4.0, 2.5, 1.0, 0.0, -1.0];
        let result = (&subject).add_scaled(&other, 2.0);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_assign_scaled() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![4.0, 2.5, 1.0, 0.0, -1.0];
        let mut result = subject;
        result.add_assign_scaled(&other, 2.0);
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let result = subject + &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_from_ref() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let result = (&subject) + &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn add_assign() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let mut result = subject;
        result += &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub() {
        let subject = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let result = subject - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_from_ref() {
        let subject = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let result = (&subject) - &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn sub_assign() {
        let subject = dense_vec![2.0, 1.5, 1.0, 1.0, 1.0];
        let other = dense_vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let expected = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let mut result = subject;
        result -= &other;
        expect!(result).to(be_equal_to(expected));
    }

    #[test]
    fn mul() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![0.0, 1.0, 2.0, 4.0, 6.0];
        let result = subject * 2.0;
        expect!(result).to(be_equal_to(other));
    }

    #[test]
    fn mul_from_ref() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![0.0, 1.0, 2.0, 4.0, 6.0];
        let result = (&subject) * 2.0;
        expect!(result).to(be_equal_to(other));
    }

    #[test]
    fn mul_assign() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let other = dense_vec![0.0, 1.0, 2.0, 4.0, 6.0];
        let mut result = subject;
        result *= 2.0;
        expect!(result).to(be_equal_to(other));
    }

    #[test]
    fn div() {
        let subject = dense_vec![0.0, 1.0, 2.0, 4.0, 6.0];
        let other = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let result = subject / 2.0;
        expect!(result).to(be_equal_to(other));
    }

    #[test]
    fn div_from_ref() {
        let subject = dense_vec![0.0, 1.0, 2.0, 4.0, 6.0];
        let other = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let result = (&subject) / 2.0;
        expect!(result).to(be_equal_to(other));
    }

    #[test]
    fn div_assign() {
        let subject = dense_vec![0.0, 1.0, 2.0, 4.0, 6.0];
        let other = dense_vec![0.0, 0.5, 1.0, 2.0, 3.0];
        let mut result = subject;
        result /= 2.0;
        expect!(result).to(be_equal_to(other));
    }

    #[test]
    fn debug() {
        let v = DenseVector::from_iter(vec![0.0, 0.25, 0.5, 0.75, 1.0].into_iter());
        expect!(format!("{:?}", v)).to(be_equal_to("[0.0, 0.25, 0.5, 0.75, 1.0]"));
    }
}
