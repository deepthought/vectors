# vectors

[![Build Status](http://img.shields.io/travis/deepthought/vectors.svg?style=flat-square)](https://travis-ci.org/deepthought/vectors)
[![Downloads](https://img.shields.io/crates/d/vectors.svg?style=flat-square)](https://crates.io/crates/vectors/)
[![Version](https://img.shields.io/crates/v/vectors.svg?style=flat-square)](https://crates.io/crates/vectors/)
[![License](https://img.shields.io/crates/l/vectors.svg?style=flat-square)](https://crates.io/crates/vectors/)

## Synopsis

Sparse & dense vectors for use in high dimensional vector spaces.

## Motivation

Many machine-learning algorithms make use of vectors in high-dimensional vector spaces.

The **vectors** provides efficient implementations for the following representations:

|       | Dense | Sparse |
|-------|-------|--------|
| **Heap**  | ✅     | ✅      |
| **Stack** | ✅     | ✅      |

## Getting Started

Add the most recent [version](https://crates.io/crates/vectors) of `vectors`
to your dependencies in your project's `Cargo.toml`.

Then add …

```rust
#[macro_use(dense_vec, sparse_vec)]
extern crate vectors;
```

… to your crate's root file (e.g. `lib.rs`, `main.rs`).

Once that's done you're ready to play!

# Example

```rust
extern crate vectors;

use vectors::Vector;
use vectors::heap::{SparseVector, DenseVector};

fn main() {
  let sparse_1 = SparseVector::from(vec![(0, 0.1), (2, 0.2), (4, 0.3), (6, 0.4)]);
  let sparse_2 = SparseVector::from(vec![(0, 0.2), (3, 0.4), (5, 0.2), (6, 0.6)]);
  let dot = sparse_1.dot(&sparse_2);
  println!("{:?}", dot);

  let dense_1 = DenseVector::from(vec![0.0, 1.0, 2.0, 4.0, 6.0]);
  let dense_2 = DenseVector::from(vec![0.2, 3.0, 0.0, 1.5, 6.0]);
  let dot = dense_1.dot(&dense_2);
  println!("{:?}", dot);
}
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html),
and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/deepthought/vectors/tags).

## Authors

* **Vincent Esche** – *Initial work* – [Regexident](https://github.com/Regexident)

See also the list of [contributors](https://github.com/deepthought/vectors/contributors) who participated in this project.

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0) – see the [LICENSE.md](LICENSE.md) file for details.
