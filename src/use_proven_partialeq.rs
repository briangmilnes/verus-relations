//! Generic functions using ProvenPartialEq as a trait bound.

pub mod use_proven_partialeq {
    use vstd::prelude::*;
    use crate::proven_partialeq::proven_partialeq::ProvenPartialEq;

verus! {

    pub fn are_equal<T: ProvenPartialEq>(a: &T, b: &T) -> (result: Option<bool>)
        ensures result == T::spec_eq(a@, b@)
    {
        a.eq(b)
    }

    pub fn symmetry_example<T: ProvenPartialEq>(_a: &T, _b: &T)
        requires T::spec_eq(_a@, _b@) == T::spec_eq(_b@, _a@)
        ensures T::spec_eq(_b@, _a@) == T::spec_eq(_a@, _b@)
    {
        proof { T::proof_symmetry(); }
    }

    pub fn transitivity_example<T: ProvenPartialEq>(_a: &T, _b: &T, _c: &T)
        requires 
            T::spec_eq(_a@, _b@) == Some(true), 
            T::spec_eq(_b@, _c@) == Some(true)
        ensures T::spec_eq(_a@, _c@) == Some(true)
    {
        proof { T::proof_transitivity(); }
    }

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
