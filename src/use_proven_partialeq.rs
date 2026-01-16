//! Use ProvenPartialEq as a trait bound
//!
//! GOAL: Demonstrate using ProvenPartialEq as a generic trait bound
//! RESULT: Yes - ProvenPartialEq works as generic bound; proof methods callable in proof blocks.

pub mod use_proven_partialeq {
    use vstd::prelude::*;
    use crate::proven_partialeq::proven_partialeq::ProvenPartialEq;

verus! {

    /// Check equality using ProvenPartialEq trait (returns Option<bool>)
    pub fn are_equal<T: ProvenPartialEq>(a: &T, b: &T) -> (result: Option<bool>)
        ensures result == T::spec_eq(a@, b@)
    {
        a.eq(b)
    }

    /// Function that uses symmetry proof
    pub fn symmetry_example<T: ProvenPartialEq>(_a: &T, _b: &T)
        requires T::spec_eq(_a@, _b@) == T::spec_eq(_b@, _a@)
        ensures T::spec_eq(_b@, _a@) == T::spec_eq(_a@, _b@)
    {
        proof {
            T::proof_symmetry();
        }
    }

    /// Function that uses transitivity proof
    pub fn transitivity_example<T: ProvenPartialEq>(_a: &T, _b: &T, _c: &T)
        requires 
            T::spec_eq(_a@, _b@) == Some(true), 
            T::spec_eq(_b@, _c@) == Some(true)
        ensures T::spec_eq(_a@, _c@) == Some(true)
    {
        proof {
            T::proof_transitivity();
        }
    }

    /// Generic container that requires ProvenPartialEq
    pub struct EqPair<T: ProvenPartialEq> {
        pub first: T,
        pub second: T,
    }

    impl<T: ProvenPartialEq> EqPair<T> {
        pub fn are_same(&self) -> (result: Option<bool>)
            ensures result == T::spec_eq(self.first@, self.second@)
        {
            self.first.eq(&self.second)
        }
    }

} // verus!
}
