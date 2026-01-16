//! Proven PartialEq with explicit axioms and Option result.
//!
//! GOAL: Create ProvenPartialEq requiring proofs of:
//!   - symmetry: forall x, y. eq(x, y) ==> eq(y, x)
//!   - transitivity: forall x, y, z. eq(x,y) && eq(y,z) ==> eq(x,z)
//!   - consistency: ne(a,b) <==> !eq(a,b) (when defined)
//!
//! NOTE: Returns Option<bool> to properly model partial equality:
//!   - Some(true) = equal
//!   - Some(false) = not equal
//!   - None = undefined/incomparable (e.g., NaN vs NaN)
//!
//! RESULT: Yes - proofs auto-verified for i32 equality (symmetry, transitivity).

pub mod proven_partialeq {
    use vstd::prelude::*;

verus! {

    /// ProvenPartialEq: Partial equality with proofs.
    ///
    /// Returns Option<bool> to model truly partial equality where some
    /// comparisons may be undefined (like NaN vs anything).
    ///
    /// - Some(true) = equal
    /// - Some(false) = not equal
    /// - None = undefined/incomparable
    pub trait ProvenPartialEq: View + Sized {
        /// Spec-level equality, returns Option to model partial equality
        spec fn spec_eq(a: Self::V, b: Self::V) -> Option<bool>;

        /// Runtime equality check
        fn eq(&self, other: &Self) -> (result: Option<bool>)
            ensures result == Self::spec_eq(self@, other@);
        
        /// Runtime inequality check (complement when defined)
        fn ne(&self, other: &Self) -> (result: Option<bool>)
            ensures result == match Self::spec_eq(self@, other@) {
                Some(b) => Some(!b),
                None => None,
            };
        
        /// Symmetry: equality is symmetric (when defined)
        proof fn proof_symmetry()
            ensures forall |x: Self::V, y: Self::V| 
                Self::spec_eq(x, y) == Self::spec_eq(y, x);
        
        /// Transitivity: equality is transitive (when all defined and equal)
        proof fn proof_transitivity()
            ensures forall |x: Self::V, y: Self::V, z: Self::V|
                (Self::spec_eq(x, y) == Some(true) && Self::spec_eq(y, z) == Some(true)) 
                    ==> Self::spec_eq(x, z) == Some(true);
    }

    // Implement directly for i32 (always defined)
    impl ProvenPartialEq for i32 {
        open spec fn spec_eq(a: i32, b: i32) -> Option<bool> { Some(a == b) }
        
        fn eq(&self, other: &Self) -> (result: Option<bool>) {
            Some(*self == *other)
        }
         
        fn ne(&self, other: &Self) -> (result: Option<bool>) {
            Some(*self != *other)
        }
        
        proof fn proof_symmetry() {
            // Verus proves: spec_eq(x, y) == spec_eq(y, x)
        }
        
        proof fn proof_transitivity() {
            // Verus proves: spec_eq(x,y)==Some(true) && spec_eq(y,z)==Some(true) ==> spec_eq(x,z)==Some(true)
        }
    }

    // Use i32 impl
    fn _test_use_i32(a: i32, b: i32) -> (result: Option<bool>)
        ensures result == Some(a@ == b@)
    {
        <i32 as ProvenPartialEq>::eq(&a, &b)
    }

    // Implement for a wrapper struct type
    pub struct MyInt { pub val: i32 }
    
    impl View for MyInt {
        type V = i32;
        open spec fn view(&self) -> i32 { self.val as i32 }
    }
    
    impl ProvenPartialEq for MyInt {
        open spec fn spec_eq(a: i32, b: i32) -> Option<bool> { Some(a == b) }
        
        fn eq(&self, other: &Self) -> (result: Option<bool>) {
            Some(self.val == other.val)
        }
        
        fn ne(&self, other: &Self) -> (result: Option<bool>) {
            Some(self.val != other.val)
        }
        
        proof fn proof_symmetry() {
            // Verus proves: spec_eq(x, y) == spec_eq(y, x)
        }
        
        proof fn proof_transitivity() {
            // Verus proves transitivity
        }
    }

    // Use MyInt impl
    fn _test_use_myint(a: MyInt, b: MyInt) -> (result: Option<bool>)
        ensures result == Some(a@ == b@)
    {
        a.eq(&b)
    }

} // verus!
}
