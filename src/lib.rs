//! Verified relations (equality, ordering) for Verus.
//!
//! This library provides traits that require proofs of mathematical properties
//! like reflexivity, symmetry, transitivity for equality and ordering relations.
pub mod proven_eq;
pub mod proven_partialeq;
pub mod use_proven_eq;
pub mod use_proven_partialeq;

pub mod complex;
pub mod proven_ord;
pub mod proven_partialord;
pub mod use_proven_ord;
pub mod use_proven_partialord;

pub mod use_hash_map;
