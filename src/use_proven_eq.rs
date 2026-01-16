//! Use ProvenEq as a trait bound.
//!
//! GOAL: Demonstrate using ProvenEq as a generic trait bound
//! RESULT: Yes - ProvenEq works as generic bound; all proof methods callable.

pub mod use_proven_eq {
    use vstd::prelude::*;
    use crate::proven_eq::proven_eq::ProvenEq;

verus! {

    /// Check equality using ProvenEq trait
    pub fn are_equal<T: ProvenEq>(a: &T, b: &T) -> (result: bool)
        ensures result == T::spec_eq(a@, b@)
    {
        a.eq(b)
    }

    /// Use the reflexivity proof - this is what distinguishes ProvenEq from ProvenPartialEq
    pub fn reflexivity_example<T: ProvenEq>(_x: &T)
        ensures T::spec_eq(_x@, _x@)
    {
        proof {
            T::proof_reflexivity();
        }
    }

    /// Use symmetry proof
    pub fn symmetry_example<T: ProvenEq>(_a: &T, _b: &T)
        requires T::spec_eq(_a@, _b@)
        ensures T::spec_eq(_b@, _a@)
    {
        proof {
            T::proof_symmetry();
        }
    }

    /// Use transitivity proof
    pub fn transitivity_example<T: ProvenEq>(_a: &T, _b: &T, _c: &T)
        requires T::spec_eq(_a@, _b@), T::spec_eq(_b@, _c@)
        ensures T::spec_eq(_a@, _c@)
    {
        proof {
            T::proof_transitivity();
        }
    }

    /// Demonstrate all three axioms together
    fn _test_with_i32() {
        let x: i32 = 42;
        let y: i32 = 42;
        let _z: i32 = 42;
        
        let _eq = are_equal(&x, &y);
        assert(_eq == (x@ == y@));
        
        // Reflexivity - the key addition over ProvenPartialEq
        reflexivity_example(&x);
        
        proof {
            i32::proof_reflexivity();
            assert(i32::spec_eq(x@, x@));
            
            i32::proof_symmetry();
            i32::proof_transitivity();
        }
    }

    /// Generic container requiring ProvenEq
    pub struct EqPair<T: ProvenEq> {
        pub first: T,
        pub second: T,
    }

    impl<T: ProvenEq> EqPair<T> {
        pub fn are_same(&self) -> (result: bool)
            ensures result == T::spec_eq(self.first@, self.second@)
        {
            self.first.eq(&self.second)
        }
        
        /// Prove that if first == second, they form an equivalence class
        pub proof fn equivalence_class(&self)
            requires T::spec_eq(self.first@, self.second@)
            ensures
                T::spec_eq(self.first@, self.first@),   // reflexivity on first
                T::spec_eq(self.second@, self.second@), // reflexivity on second
                T::spec_eq(self.second@, self.first@),  // symmetry
        {
            T::proof_reflexivity();
            T::proof_symmetry();
        }
    }

} // verus!
}

