// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sparse::*;

impl<T> fmt::Debug for SparseVector<T>
where
    T: Copy + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "[");
        for (fmt_idx, (index, value)) in self.iter().enumerate() {
            try! {
                if fmt_idx > 0 { write!(f, ", ({}, {:?})", index, value) }
                else { write!(f, "({}, {:?})", index, value) }
            };
        }
        let _ = write!(f, "]");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use expectest::prelude::*;

    #[test]
    fn debug() {
        let vector = sparse_vec![(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0)];
        let subject = format!("{:?}", vector);
        let expected = "[(0, 0.2), (1, 0.5), (2, 1.0), (4, 2.0)]";
        expect!(subject).to(be_equal_to(expected));
    }
}
