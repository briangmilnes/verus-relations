// Copyright (c) 2025 Brian G. Milnes
//! Use ProvenPartialEq as a trait bound
//!
//! GOAL: Demonstrate using ProvenPartialEq as a generic trait bound
//! RESULT: Yes - ProvenPartialEq works as generic bound; proof methods callable in proof blocks.

pub mod use_proven_partialeq {
    use vstd::prelude::*;
    use crate::proven_partialeq::proven_partialeq::ProvenPartialEq;

verus! {

    // Function with ProvenPartialEq bound
    pub fn are_equal<T: ProvenPartialEq>(a: &T, b: &T) -> (result: bool)
        ensures result == T::spec_eq(a@, b@)
    {
        a.eq(b)
    }

    // Function that uses the reflexivity proof
    pub fn reflexivity_example<T: ProvenPartialEq>(x: &T)
        ensures T::spec_eq(x@, x@)
    {
        proof {
            T::proof_reflexivity();
        }
    }

    // Function that uses symmetry proof
    pub fn symmetry_example<T: ProvenPartialEq>(a: &T, b: &T)
        requires T::spec_eq(a@, b@)
        ensures T::spec_eq(b@, a@)
    {
        proof {
            T::proof_symmetry();
        }
    }

    // Function that uses transitivity proof
    pub fn transitivity_example<T: ProvenPartialEq>(a: &T, b: &T, c: &T)
        requires T::spec_eq(a@, b@), T::spec_eq(b@, c@)
        ensures T::spec_eq(a@, c@)
    {
        proof {
            T::proof_transitivity();
        }
    }

    // Use with i32
    fn test_with_i32() {
        let x: i32 = 42;
        let y: i32 = 42;
        let z: i32 = 42;
        
        let eq = are_equal(&x, &y);
        assert(eq == (x@ == y@));
        
        reflexivity_example(&x);
        
        proof {
            i32::proof_reflexivity();
            assert(i32::spec_eq(x@, x@));
        }
    }

    // Generic container that requires ProvenPartialEq
    pub struct EqPair<T: ProvenPartialEq> {
        pub first: T,
        pub second: T,
    }

    impl<T: ProvenPartialEq> EqPair<T> {
        pub fn are_same(&self) -> (result: bool)
            ensures result == T::spec_eq(self.first@, self.second@)
        {
            self.first.eq(&self.second)
        }
    }

} // verus!
}

