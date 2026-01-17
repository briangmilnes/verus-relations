//! ProvenPartialEq: Certifies that a PartialEq impl is well-behaved.
//!
//! Uses vstd's eq_spec, requires proofs of symmetry and transitivity.

pub mod proven_partialeq {
    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpec;

#[cfg(verus_keep_ghost)]
verus! {

    /// ProvenPartialEq certifies that a PartialEq impl is well-behaved:
    /// symmetric and transitive by requiring proofs.
    pub trait ProvenPartialEq: PartialEqSpec + Sized {
        // Require obeys_eq_spec() == true so vstd's eq_spec is meaningful
        proof fn proof_obeys_eq_spec()
            ensures Self::obeys_eq_spec();

        proof fn proof_symmetry()
            ensures forall |x: &Self, y: &Self| 
                #[trigger] x.eq_spec(y) == y.eq_spec(x);
        
        proof fn proof_transitivity()
            ensures forall |x: &Self, y: &Self, z: &Self|
                #![trigger x.eq_spec(y), y.eq_spec(z)]
                (x.eq_spec(y) && y.eq_spec(z)) ==> x.eq_spec(z);
    }

    pub broadcast proof fn lemma_obeys_eq_spec<T: ProvenPartialEq>()
        ensures #[trigger] T::obeys_eq_spec()
    {
        T::proof_obeys_eq_spec();
    }

    pub broadcast proof fn lemma_symmetry<T: ProvenPartialEq>(x: &T, y: &T)
        ensures #[trigger] x.eq_spec(y) == y.eq_spec(x)
    {
        T::proof_symmetry();
    }

    pub broadcast proof fn lemma_transitivity<T: ProvenPartialEq>(x: &T, y: &T, z: &T)
        requires #[trigger] x.eq_spec(y), #[trigger] y.eq_spec(z)
        ensures x.eq_spec(z)
    {
        T::proof_transitivity();
    }

    pub broadcast group group_proven_partialeq {
        lemma_obeys_eq_spec,
        lemma_symmetry,
        lemma_transitivity,
    }

    // i32 already has eq_spec in vstd
    impl ProvenPartialEq for i32 {
        proof fn proof_obeys_eq_spec() {}
        proof fn proof_symmetry() {}
        proof fn proof_transitivity() {}
    }

} // verus!
}
