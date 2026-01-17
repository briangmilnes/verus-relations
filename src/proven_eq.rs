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

// Signed integers
impl ProvenEq for i8 {
    proof fn proof_reflexivity() {
    }
}

impl ProvenEq for i16 {
    proof fn proof_reflexivity() {
    }
}

impl ProvenEq for i32 {
    proof fn proof_reflexivity() {
    }
}

impl ProvenEq for i64 {
    proof fn proof_reflexivity() {
    }
}

impl ProvenEq for i128 {
    proof fn proof_reflexivity() {
    }
}

impl ProvenEq for isize {
    proof fn proof_reflexivity() {
    }
}

// Unsigned integers
impl ProvenEq for u8 {
    proof fn proof_reflexivity() {
    }
}

impl ProvenEq for u16 {
    proof fn proof_reflexivity() {
    }
}

impl ProvenEq for u32 {
    proof fn proof_reflexivity() {
    }
}

impl ProvenEq for u64 {
    proof fn proof_reflexivity() {
    }
}

impl ProvenEq for u128 {
    proof fn proof_reflexivity() {
    }
}

impl ProvenEq for usize {
    proof fn proof_reflexivity() {
    }
}

// Note: bool and char are omitted because vstd doesn't provide PartialEqSpec for them
} // verus!
}
