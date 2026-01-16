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

    // Broadcast lemma for automatic reflexivity
    pub broadcast proof fn lemma_reflexivity<T: ProvenEq>(x: T::V)
        ensures #[trigger] T::spec_eq(x, x) == Some(true)
    {
        T::proof_reflexivity();
    }

    pub broadcast group group_proven_eq {
        lemma_reflexivity,
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
