//! Proven Eq extending ProvenPartialEq with reflexivity.
//!
//! GOAL: Create ProvenEq that extends ProvenPartialEq by adding reflexivity proof.
//!   - reflexivity: forall x. eq(x, x) (what Eq adds over PartialEq)
//!   - symmetry, transitivity, consistency: inherited from ProvenPartialEq
//!
//! NOTE: Rust's Eq is a marker trait that PROMISES reflexivity but doesn't verify it.
//! This trait REQUIRES a proof of reflexivity, making it a true equivalence relation.
//!
//! RESULT: Yes - reflexivity proof auto-verified for i32.

pub mod proven_eq {
    use vstd::prelude::*;
    use crate::proven_partialeq::proven_partialeq::ProvenPartialEq;

verus! {

    /// ProvenEq: Extends ProvenPartialEq with reflexivity proof.
    ///
    /// Unlike Rust's Eq (which is just a marker trait promising reflexivity),
    /// ProvenEq requires an actual proof that reflexivity holds.
    ///
    /// Inherited from ProvenPartialEq:
    /// - spec_eq, eq, ne
    /// - Symmetry: forall x, y. eq(x, y) ==> eq(y, x)
    /// - Transitivity: forall x, y, z. eq(x,y) && eq(y,z) ==> eq(x,z)
    /// - Consistency: ne(a,b) <==> !eq(a,b) (by construction)
    ///
    /// Added by ProvenEq:
    /// - Reflexivity: forall x. eq(x, x)
    pub trait ProvenEq: ProvenPartialEq {
        /// Reflexivity: every element equals itself.
        /// This is what Rust's Eq promises but doesn't verify.
        proof fn proof_reflexivity()
            ensures forall |x: Self::V| Self::spec_eq(x, x);
    }

    // Implement ProvenEq for i32 (already has ProvenPartialEq)
    impl ProvenEq for i32 {
        proof fn proof_reflexivity() {
            // Verus proves: forall x. x == x
        }
    }

    // Test using i32 ProvenEq (eq inherited from ProvenPartialEq)
    fn _test_use_i32(a: i32, b: i32) -> (result: bool)
        ensures result == (a@ == b@)
    {
        <i32 as ProvenPartialEq>::eq(&a, &b)
    }

    // Re-export MyInt from proven_partialeq for convenience
    pub use crate::proven_partialeq::proven_partialeq::MyInt;

    // Implement ProvenEq for MyInt (already has ProvenPartialEq)
    impl ProvenEq for MyInt {
        proof fn proof_reflexivity() {
            // Verus proves: forall x. x == x
        }
    }

    // Test using MyInt
    fn _test_use_myint(a: MyInt, b: MyInt) -> (result: bool)
        ensures result == (a@ == b@)
    {
        a.eq(&b)
    }

} // verus!
}
