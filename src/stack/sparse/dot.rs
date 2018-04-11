// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

use super::iter::OrderedMapIterable;
use ordered_iter::OrderedMapIterator;

impl<T, A> Dot for SparseVector<A>
where
    T: Copy + Zero + Num,
    A: Array<Item = (usize, T)>,
{
    type Output = T;

    fn dot(&self, rhs: &Self) -> Self::Output {
        let iter = rhs.iter().ordered_map_iterator();
        self.iter()
            .inner_join_map(iter)
            .fold(T::zero(),
                  |sum, (_, (lhs, rhs))| sum + (lhs * rhs))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn dot() {
        let subject = SparseVector::from([(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)]);
        let other = SparseVector::from([(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)]);
        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.85));
    }
}
