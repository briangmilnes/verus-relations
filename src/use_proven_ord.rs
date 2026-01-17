//! Demonstrates using ProvenOrd as a trait bound.

pub mod use_proven_ord {
    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use core::cmp::Ordering;
    #[cfg(verus_keep_ghost)]
    use crate::proven_ord::proven_ord::ProvenOrd;
    #[cfg(verus_keep_ghost)]
    use crate::proven_ord::proven_ord::group_proven_ord;
    #[cfg(verus_keep_ghost)]
    use crate::proven_partialord::proven_partialord::group_proven_partialord;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpec;

#[cfg(verus_keep_ghost)]
verus! {

    broadcast use { group_proven_partialord, group_proven_ord };

    pub fn compare<T: ProvenOrd>(a: &T, b: &T) -> (result: Ordering)
        requires T::obeys_cmp_spec()
        ensures result == a.cmp_spec(b)
    {
        Ord::cmp(a, b)
    }

    pub fn totality_example<T: ProvenOrd>(a: &T, b: &T)
        ensures a.partial_cmp_spec(b).is_some()
    {
    }

    pub fn antisymmetry_example<T: ProvenOrd>(a: &T, b: &T)
        requires 
            a.cmp_spec(b) != Ordering::Greater,
            b.cmp_spec(a) != Ordering::Greater
        ensures a.cmp_spec(b) == Ordering::Equal
    {
    }

    // Consistency with Eq: cmp == Equal iff eq_spec
    pub fn consistency_with_eq_example<T: ProvenOrd>(a: &T, b: &T)
        requires a.cmp_spec(b) == Ordering::Equal
        ensures a.eq_spec(b)
    {
    }

    pub fn consistency_with_eq_reverse<T: ProvenOrd>(a: &T, b: &T)
        requires a.eq_spec(b)
        ensures a.cmp_spec(b) == Ordering::Equal
    {
    }

    // Transitivity for all orderings
    pub fn transitivity_lt_example<T: ProvenOrd>(a: &T, b: &T, c: &T)
        requires 
            a.cmp_spec(b) == Ordering::Less,
            b.cmp_spec(c) == Ordering::Less
        ensures a.cmp_spec(c) == Ordering::Less
    {
    }

    pub fn transitivity_le_example<T: ProvenOrd>(a: &T, b: &T, c: &T)
        requires 
            a.cmp_spec(b) != Ordering::Greater,  // a <= b
            b.cmp_spec(c) != Ordering::Greater   // b <= c
        ensures a.cmp_spec(c) != Ordering::Greater  // a <= c
    {
    }

    pub fn transitivity_gt_example<T: ProvenOrd>(a: &T, b: &T, c: &T)
        requires 
            a.cmp_spec(b) == Ordering::Greater,
            b.cmp_spec(c) == Ordering::Greater
        ensures a.cmp_spec(c) == Ordering::Greater
    {
    }

    pub fn transitivity_ge_example<T: ProvenOrd>(a: &T, b: &T, c: &T)
        requires 
            a.cmp_spec(b) != Ordering::Less,  // a >= b
            b.cmp_spec(c) != Ordering::Less   // b >= c
        ensures a.cmp_spec(c) != Ordering::Less  // a >= c
    {
    }

} // verus!
}
