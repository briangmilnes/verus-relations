//! Proven Eq with explicit axioms including reflexivity.
//!
//! GOAL: Create ProvenEq requiring proofs of all equivalence properties:
//!   - reflexivity: forall x. eq(x, x) (what Eq adds over PartialEq)
//!   - symmetry: forall x, y. eq(x, y) ==> eq(y, x)
//!   - transitivity: forall x, y, z. eq(x,y) && eq(y,z) ==> eq(x,z)
//!   - consistency: ne(a,b) <==> !eq(a,b) (proven by construction via ensures)
//!
//! NOTE: Rust's Eq is a marker trait that PROMISES reflexivity but doesn't verify it.
//! This trait REQUIRES a proof of reflexivity, making it a true equivalence relation.
//!
//! RESULT: Yes - all proofs auto-verified for i32 (reflexivity, symmetry, transitivity).

pub mod proven_eq {
    use vstd::prelude::*;

verus! {

    /// ProvenEq: A trait requiring proofs of all equivalence relation properties.
    ///
    /// Unlike Rust's Eq (which is just a marker trait promising reflexivity),
    /// ProvenEq requires an actual proof that reflexivity holds.
    ///
    /// Properties proven:
    /// - Reflexivity: forall x. eq(x, x)
    /// - Symmetry: forall x, y. eq(x, y) ==> eq(y, x)
    /// - Transitivity: forall x, y, z. eq(x,y) && eq(y,z) ==> eq(x,z)
    /// - Consistency: ne(a,b) <==> !eq(a,b) (by construction in ensures)
    pub trait ProvenEq: View + Sized {
        spec fn spec_eq(a: Self::V, b: Self::V) -> bool;

        fn eq(&self, other: &Self) -> (result: bool)
            ensures result == Self::spec_eq(self@, other@);
        
        fn ne(&self, other: &Self) -> (result: bool)
            ensures result == !Self::spec_eq(self@, other@);  // Consistency by construction
        
        /// Reflexivity: every element equals itself.
        /// This is what Rust's Eq promises but doesn't verify.
        proof fn proof_reflexivity()
            ensures forall |x: Self::V| Self::spec_eq(x, x);
        
        /// Symmetry: equality is symmetric.
        proof fn proof_symmetry()
            ensures forall |x: Self::V, y: Self::V| 
                Self::spec_eq(x, y) ==> Self::spec_eq(y, x);
        
        /// Transitivity: equality is transitive.
        proof fn proof_transitivity()
            ensures forall |x: Self::V, y: Self::V, z: Self::V|
                (Self::spec_eq(x, y) && Self::spec_eq(y, z)) ==> Self::spec_eq(x, z);
    }

    // Implement ProvenEq for i32
    impl ProvenEq for i32 {
        open spec fn spec_eq(a: i32, b: i32) -> bool { a == b }
        
        fn eq(&self, other: &Self) -> (result: bool) {
            *self == *other
        }
         
        fn ne(&self, other: &Self) -> (result: bool) {
            *self != *other
        }
        
        proof fn proof_reflexivity() {
            // Verus proves: forall x. x == x
        }
        
        proof fn proof_symmetry() {
            // Verus proves: x == y ==> y == x
        }
        
        proof fn proof_transitivity() {
            // Verus proves: x == y && y == z ==> x == z
        }
    }

    // Test using i32 ProvenEq (disambiguate from std PartialEq)
    fn _test_use_i32(a: i32, b: i32) -> (result: bool)
        ensures result == (a@ == b@)
    {
        ProvenEq::eq(&a, &b)
    }

    // Implement ProvenEq for a wrapper struct
    pub struct MyInt { pub val: i32 }
    
    impl View for MyInt {
        type V = i32;
        open spec fn view(&self) -> i32 { self.val as i32 }
    }
    
    impl ProvenEq for MyInt {
        open spec fn spec_eq(a: i32, b: i32) -> bool { a == b }
        
        fn eq(&self, other: &Self) -> (result: bool) {
            self.val == other.val
        }
        
        fn ne(&self, other: &Self) -> (result: bool) {
            self.val != other.val
        }
        
        proof fn proof_reflexivity() {
            // Verus proves: forall x. x == x
        }
        
        proof fn proof_symmetry() {
            // Verus proves: x == y ==> y == x
        }
        
        proof fn proof_transitivity() {
            // Verus proves: x == y && y == z ==> x == z
        }
    }

    // Test using MyInt
    fn _test_use_myint(a: MyInt, b: MyInt) -> (result: bool)
        ensures result == (a@ == b@)
    {
        a.eq(&b)
    }

} // verus!
}

