//! Demonstrates using ProvenPartialEq as a trait bound.

pub mod use_proven_partialeq {
    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use crate::proven_partialeq::proven_partialeq::ProvenPartialEq;
    #[cfg(verus_keep_ghost)]
    use crate::proven_partialeq::proven_partialeq::group_proven_partialeq;

#[cfg(verus_keep_ghost)]
verus! {

    broadcast use group_proven_partialeq;

    pub fn are_equal<T: ProvenPartialEq>(a: &T, b: &T) -> (result: bool)
        requires T::obeys_eq_spec()
        ensures result == a.eq_spec(b)
    {
        PartialEq::eq(a, b)
    }

    pub fn symmetry_example<T: ProvenPartialEq>(a: &T, b: &T)
        ensures a.eq_spec(b) == b.eq_spec(a)
    {
    }

    pub fn transitivity_example<T: ProvenPartialEq>(a: &T, b: &T, c: &T)
        requires a.eq_spec(b), b.eq_spec(c)
        ensures a.eq_spec(c)
    {
    }

} // verus!
}
