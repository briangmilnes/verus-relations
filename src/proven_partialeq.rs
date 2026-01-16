//! Proven PartialEq with explicit axioms.
//!
//! GOAL: Create ProvenPartialEq requiring proofs of:
//!   - symmetry: forall x, y. eq(x, y) ==> eq(y, x)
//!   - transitivity: forall x, y, z. eq(x,y) && eq(y,z) ==> eq(x,z)
//!   - consistency: ne(a,b) <==> !eq(a,b) (proven by construction via ensures)
//!
//! NOTE: Rust's PartialEq does NOT require reflexivity (forall x. eq(x, x)) because
//! IEEE 754 NaN != NaN. Rust is making the exception the rule here, which really
//! weakens PartialEq by not requiring reflexivity for types where it should hold.
//!
//! RESULT: Yes - proofs auto-verified for i32 equality (symmetry, transitivity).

pub mod proven_partialeq {
    use vstd::prelude::*;

verus! {

    // Trait with explicit axioms.
    // NOTE: Rust's PartialEq omits reflexivity because IEEE NaN != NaN.
    // This makes the exception the rule, weakening PartialEq for types where reflexivity holds.
    pub trait ProvenPartialEq: View + Sized {
        spec fn spec_eq(a: Self::V, b: Self::V) -> bool;

        fn eq(&self, other: &Self) -> (result: bool)
            ensures result == Self::spec_eq(self@, other@);
        
        fn ne(&self, other: &Self) -> (result: bool)
            ensures result == !Self::spec_eq(self@, other@);  // Consistency by construction
        
        // Reflexivity commented out to match Rust's PartialEq (IEEE NaN != NaN exception)
        // proof fn proof_reflexivity()
        //     ensures forall |x: Self::V| Self::spec_eq(x, x);
        
        proof fn proof_symmetry()
            ensures forall |x: Self::V, y: Self::V| 
                Self::spec_eq(x, y) ==> Self::spec_eq(y, x);
        
        proof fn proof_transitivity()
            ensures forall |x: Self::V, y: Self::V, z: Self::V|
                (Self::spec_eq(x, y) && Self::spec_eq(y, z)) ==> Self::spec_eq(x, z);
    }

    // Implement directly for i32
    impl ProvenPartialEq for i32 {
        open spec fn spec_eq(a: i32, b: i32) -> bool { a == b }
        
        fn eq(&self, other: &Self) -> (result: bool) {
            *self == *other
        }
         
        fn ne(&self, other: &Self) -> (result: bool) {
            *self != *other
        }
        
        // Reflexivity commented out (IEEE NaN exception made the rule)
        // proof fn proof_reflexivity() {
        //     // Verus proves: forall x. x == x
        // }
        
        proof fn proof_symmetry() {
            // Verus proves: x == y ==> y == x
        }
        
        proof fn proof_transitivity() {
            // Verus proves: x == y && y == z ==> x == z
        }
    }

    // Use i32 impl (disambiguate from std PartialEq)
    fn test_use_i32(a: i32, b: i32) -> (result: bool)
        ensures result == (a@ == b@)
    {
        ProvenPartialEq::eq(&a, &b)
    }

    // Implement for a wrapper struct type
    pub struct MyInt { pub val: i32 }
    
    impl View for MyInt {
        type V = i32;
        open spec fn view(&self) -> i32 { self.val as i32 }
    }
    
    impl ProvenPartialEq for MyInt {
        open spec fn spec_eq(a: i32, b: i32) -> bool { a == b }
        
        fn eq(&self, other: &Self) -> (result: bool) {
            self.val == other.val
        }
        
        fn ne(&self, other: &Self) -> (result: bool) {
            self.val != other.val
        }
        
        // Reflexivity commented out (IEEE NaN exception made the rule)
        // proof fn proof_reflexivity() {
        //     // Verus proves: forall x. x == x
        // }
        
        proof fn proof_symmetry() {
            // Verus proves: x == y ==> y == x
        }
        
        proof fn proof_transitivity() {
            // Verus proves: x == y && y == z ==> x == z
        }
    }

    // Use MyInt impl
    fn test_use_myint(a: MyInt, b: MyInt) -> (result: bool)
        ensures result == (a@ == b@)
    {
        a.eq(&b)
    }

} // verus!
}

