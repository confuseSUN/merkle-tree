//! merkle-tree implement Merkle Tree in Rust.
//!
//! A disadvantage in current implementation is need to recalculate full tree when added
//! or removed element into/from tree. To increase performance we need to implement partial
//! tree modification when adding or removing element.

// #![deny(missing_docs,
// missing_debug_implementations,
// missing_copy_implementations,
// trivial_casts,
// trivial_numeric_casts,
// unsafe_code,
// unused_import_braces,
// unused_qualifications)]
//#![feature(test)]

extern crate crypto;
// extern crate test;

mod element;
mod hash_utils;
mod merkle_tree;
mod proof;

mod tests;

pub use self::merkle_tree::MerkleTree;
pub use self::hash_utils::*;
pub use self::proof::*;
pub use self::element::*;
