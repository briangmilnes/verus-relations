//! ProvenPartialEq: PartialEq with required proofs of symmetry and transitivity.
//!
//! Returns Option<bool> to model partial equality (None = undefined, e.g. NaN).
//!
//! Rust's PartialEq for reference:
//! ```rust,ignore
//! pub trait PartialEq<Rhs = Self> where Rhs: ?Sized {
//!     fn eq(&self, other: &Rhs) -> bool;
//!     fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
//! }
//! ```

pub mod proven_partialeq {
    use vstd::prelude::*;

verus! {

    pub trait ProvenPartialEq: View + Sized {
        spec fn spec_eq(a: Self::V, b: Self::V) -> Option<bool>;

        fn eq(&self, other: &Self) -> (result: Option<bool>)
            ensures result == Self::spec_eq(self@, other@);
        
        fn ne(&self, other: &Self) -> (result: Option<bool>)
            ensures result == match Self::spec_eq(self@, other@) {
                Some(b) => Some(!b),
                None => None,
            };
        
        proof fn proof_symmetry()
            ensures forall |x: Self::V, y: Self::V| 
                Self::spec_eq(x, y) == Self::spec_eq(y, x);
        
        proof fn proof_transitivity()
            ensures forall |x: Self::V, y: Self::V, z: Self::V|
                (Self::spec_eq(x, y) == Some(true) && Self::spec_eq(y, z) == Some(true)) 
                    ==> Self::spec_eq(x, z) == Some(true);

        fn is_eq(&self, other: &Self) -> (result: bool)
            ensures result == (Self::spec_eq(self@, other@) == Some(true))
        {
            match self.eq(other) {
                Some(true) => true,
                _ => false,
            }
        }

        fn is_ne(&self, other: &Self) -> (result: bool)
            ensures result == (Self::spec_eq(self@, other@) == Some(false))
        {
            match self.eq(other) {
                Some(false) => true,
                _ => false,
            }
        }

        fn is_comparable(&self, other: &Self) -> (result: bool)
            ensures result == Self::spec_eq(self@, other@).is_some()
        {
            self.eq(other).is_some()
        }
    }

    impl ProvenPartialEq for i32 {
        open spec fn spec_eq(a: i32, b: i32) -> Option<bool> { Some(a == b) }
        
        fn eq(&self, other: &Self) -> (result: Option<bool>) { Some(*self == *other) }
        fn ne(&self, other: &Self) -> (result: Option<bool>) { Some(*self != *other) }
        
        proof fn proof_symmetry() {}
        proof fn proof_transitivity() {}
    }

    pub struct MyInt { pub val: i32 }
    
    impl View for MyInt {
        type V = i32;
        open spec fn view(&self) -> i32 { self.val as i32 }
    }
    
    impl ProvenPartialEq for MyInt {
        open spec fn spec_eq(a: i32, b: i32) -> Option<bool> { Some(a == b) }
        
        fn eq(&self, other: &Self) -> (result: Option<bool>) { Some(self.val == other.val) }
        fn ne(&self, other: &Self) -> (result: Option<bool>) { Some(self.val != other.val) }
        
        proof fn proof_symmetry() {}
        proof fn proof_transitivity() {}
    }

} // verus!
}
