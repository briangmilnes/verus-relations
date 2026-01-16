//! Generic functions using ProvenEq as a trait bound.

pub mod use_proven_eq {
    use vstd::prelude::*;
    use crate::proven_eq::proven_eq::ProvenEq;
    #[allow(unused_imports)]
    use crate::proven_partialeq::proven_partialeq::ProvenPartialEq;

verus! {

    pub fn are_equal<T: ProvenEq>(a: &T, b: &T) -> (result: Option<bool>)
        ensures result == T::spec_eq(a@, b@)
    {
        a.eq(b)
    }

    pub fn reflexivity_example<T: ProvenEq>(_x: &T)
        ensures T::spec_eq(_x@, _x@) == Some(true)
    {
        proof { T::proof_reflexivity(); }
    }

    pub fn symmetry_example<T: ProvenEq>(_a: &T, _b: &T)
        requires T::spec_eq(_a@, _b@) == T::spec_eq(_b@, _a@)
        ensures T::spec_eq(_b@, _a@) == T::spec_eq(_a@, _b@)
    {
    }

    pub fn transitivity_example<T: ProvenEq>(_a: &T, _b: &T, _c: &T)
        requires 
            T::spec_eq(_a@, _b@) == Some(true), 
            T::spec_eq(_b@, _c@) == Some(true)
        ensures T::spec_eq(_a@, _c@) == Some(true)
    {
        proof { T::proof_transitivity(); }
    }

    pub struct EqPair<T: ProvenEq> {
        pub first: T,
        pub second: T,
    }

    impl<T: ProvenEq> EqPair<T> {
        pub fn are_same(&self) -> (result: Option<bool>)
            ensures result == T::spec_eq(self.first@, self.second@)
        {
            self.first.eq(&self.second)
        }
        
        pub proof fn equivalence_class(&self)
            requires T::spec_eq(self.first@, self.second@) == Some(true)
            ensures
                T::spec_eq(self.first@, self.first@) == Some(true),
                T::spec_eq(self.second@, self.second@) == Some(true),
                T::spec_eq(self.second@, self.first@) == Some(true),
        {
            T::proof_reflexivity();
            T::proof_symmetry();
        }
    }

} // verus!
}
