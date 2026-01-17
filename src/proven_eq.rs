//! ProvenEq: Extends ProvenPartialEq with reflexivity proof.
//!
//! Rust's Eq is a marker trait promising reflexivity. ProvenEq requires proof.
pub mod proven_eq {
    #[cfg(verus_keep_ghost)]
    use crate::proven_partialeq::proven_partialeq::ProvenPartialEq;
    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpec;

    #[cfg(verus_keep_ghost)]
    verus! {

pub trait ProvenEq: Eq + ProvenPartialEq {
    proof fn proof_reflexivity()
        ensures
            forall|x: &Self| #[trigger] x.eq_spec(x),
    ;
}

// Broadcast lemma for automatic reflexivity
pub broadcast proof fn lemma_reflexivity<T: ProvenEq>(x: &T)
    ensures
        #[trigger] x.eq_spec(x),
{
    T::proof_reflexivity();
}

pub broadcast group group_proven_eq {
    lemma_reflexivity,
}

impl ProvenEq for i32 {
    proof fn proof_reflexivity() {
    }
}

} // verus!
}
