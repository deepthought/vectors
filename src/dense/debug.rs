// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dense::*;

impl<T> fmt::Debug for DenseVector<T>
where
    T: Copy + fmt::Debug
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

    use expectest::prelude::*;

    #[test]
    fn debug() {
        let vector = dense_vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let subject = format!("{:?}", vector);
        let expected = "[0.0, 0.25, 0.5, 0.75, 1.0]";
        expect!(subject).to(be_equal_to(expected));
    }
}
