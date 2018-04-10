// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dense::*;

impl<T> fmt::Debug for DenseVector<T>
    where T: Clone + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "[");
        for (index, item) in self.components.iter().enumerate() {
            try! {
                if index > 0 { write!(f, ", {:?}", item) }
                else { write!(f, "{:?}", item) }
            }
        }
        let _ = write!(f, "]");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::iter::{IntoIterator, FromIterator};

    use expectest::prelude::*;

    #[test]
    fn debug() {
        let v = DenseVector::from_iter(vec![0.0, 0.25, 0.5, 0.75, 1.0].into_iter());
        expect!(format!("{:?}", v)).to(be_equal_to("[0.0, 0.25, 0.5, 0.75, 1.0]"));
    }
}
