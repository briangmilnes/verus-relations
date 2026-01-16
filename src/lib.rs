//! Verified relations (equality, ordering) for Verus.
//!
//! This library provides traits that require proofs of mathematical properties
//! like reflexivity, symmetry, and transitivity for equality relations.

pub mod proven_partialeq;
pub mod use_proven_partialeq;
pub mod proven_eq;
pub mod use_proven_eq;

