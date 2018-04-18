extern crate vectors;

use vectors::VectorExt;
use vectors::dense::stack::DenseVector;
use vectors::sparse::stack::SparseVector;

fn main() {
  let sparse_1 = SparseVector::from([(0, 0.1), (2, 0.2), (4, 0.3), (6, 0.4)]);
  let sparse_2 = SparseVector::from([(0, 0.2), (3, 0.4), (5, 0.2), (6, 0.6)]);
  let dot = sparse_1.dot(&sparse_2);
  println!("{:?}", dot);

  let dense_1 = DenseVector::from([0.0, 1.0, 2.0, 4.0, 6.0]);
  let dense_2 = DenseVector::from([0.2, 3.0, 0.0, 1.5, 6.0]);
  let dot = dense_1.dot(&dense_2);
  println!("{:?}", dot);
}
