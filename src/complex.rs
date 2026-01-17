//! Complex number type with ProvenPartialEq and ProvenEq implementations.

pub mod complex {
    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    #[cfg(verus_keep_ghost)]
    use crate::proven_partialeq::proven_partialeq::ProvenPartialEq;
    #[cfg(verus_keep_ghost)]
    use crate::proven_eq::proven_eq::ProvenEq;

#[cfg(verus_keep_ghost)]
verus! {

    pub struct Complex {
        pub real: i32,
        pub imaginary: i32,
    }

    impl PartialEq for Complex {
        fn eq(&self, other: &Self) -> bool {
             self.real == other.real
        }
    }

    impl PartialEqSpecImpl for Complex {
        open spec fn obeys_eq_spec() -> bool { true }

        open spec fn eq_spec(&self, other: &Self) -> bool {
            self.real == other.real
        }
    }

    impl ProvenPartialEq for Complex {
        proof fn proof_obeys_eq_spec() {}
        proof fn proof_symmetry() {}
        proof fn proof_transitivity() {}
    }

    // Full equality version as we can't have two implementations for the PartiaEqSpecImpl.
    pub struct ComplexEq {
        pub real: i32,
        pub imaginary: i32,
    }

    impl PartialEq for ComplexEq {
        fn eq(&self, other: &Self) -> bool {
            self.real == other.real && self.imaginary == other.imaginary
        }
    }

    impl Eq for ComplexEq {}

    impl PartialEqSpecImpl for ComplexEq {
        open spec fn obeys_eq_spec() -> bool { true }

        open spec fn eq_spec(&self, other: &Self) -> bool {
            self.real == other.real && self.imaginary == other.imaginary
        }
    }

    impl ProvenPartialEq for ComplexEq {
        proof fn proof_obeys_eq_spec() {}
        proof fn proof_symmetry() {}
        proof fn proof_transitivity() {}
    }

    impl ProvenEq for ComplexEq {
        proof fn proof_reflexivity() {}
    }

} // verus!
}
