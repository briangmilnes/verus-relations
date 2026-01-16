//! ProvenEq: Extends ProvenPartialEq with reflexivity proof.
//!
//! Rust's Eq is a marker trait promising reflexivity. ProvenEq requires proof.

pub mod proven_eq {
    use vstd::prelude::*;
    use crate::proven_partialeq::proven_partialeq::ProvenPartialEq;

verus! {

    pub trait ProvenEq: ProvenPartialEq {
        proof fn proof_reflexivity()
            ensures forall |x: Self::V| #[trigger] Self::spec_eq(x, x) == Some(true);
    }

    impl ProvenEq for i32 {
        proof fn proof_reflexivity() {}
    }

    pub use crate::proven_partialeq::proven_partialeq::MyInt;

    impl ProvenEq for MyInt {
        proof fn proof_reflexivity() {}
    }

} // verus!
}
