// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dense::*;

impl<T> Dot for DenseVector<T>
    where T: Clone + Zero + Num
{
    type Output = T;

    fn dot(&self, rhs: &Self) -> Self::Output {
        let iter = rhs.iter();
        self.iter()
            .zip(iter)
            .fold(T::zero(),
                  |sum, (lhs, rhs)| sum + (lhs.clone() * rhs))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn dot() {
        let subject = dense_vec![0.0, 0.5, 1.0, 2.0, 4.0];
        let other = dense_vec![0.1, 0.2, 0.3, 0.4, 0.0];
        let dot = subject.dot(&other);
        expect!(dot).to(be_close_to(1.2));
    }
}
