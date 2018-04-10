// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sparse::*;

use sparse::iter::OrderedMapIterable;
use ordered_iter::OrderedMapIterator;

impl<T> Dot for SparseVector<T>
    where T: Clone + Zero + Num
{
    type Output = T;

    fn dot(&self, rhs: &Self) -> Self::Output {
        let iter = rhs.iter().map(|(k, v)| (k, v.clone())).ordered_map_iterator();
        self.iter()
            .inner_join_map(iter)
            .fold(T::zero(),
                  |sum, (_, (lhs, rhs))| sum + (lhs.clone() * rhs.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn dot() {
        let subject = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0), (5, 4.0)];
        let other = sparse_vec![(1, 0.1), (2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)];
        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.85));
    }
}
