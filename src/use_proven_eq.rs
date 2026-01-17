//! Demonstrates using ProvenEq as a trait bound.
pub mod use_proven_eq {
    #[cfg(verus_keep_ghost)]
    use crate::proven_eq::proven_eq::group_proven_eq;
    #[cfg(verus_keep_ghost)]
    use crate::proven_eq::proven_eq::ProvenEq;
    #[cfg(verus_keep_ghost)]
    use crate::proven_partialeq::proven_partialeq::group_proven_partialeq;
    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    verus! {

broadcast use {group_proven_partialeq, group_proven_eq};

pub fn are_equal<T: ProvenEq>(a: &T, b: &T) -> (result: bool)
    requires
        T::obeys_eq_spec(),
    ensures
        result == a.eq_spec(b),
{
    PartialEq::eq(a, b)
}

pub fn reflexivity_example<T: ProvenEq>(x: &T)
    ensures
        x.eq_spec(x),
{
}

pub fn symmetry_example<T: ProvenEq>(a: &T, b: &T)
    ensures
        a.eq_spec(b) == b.eq_spec(a),
{
}

pub fn transitivity_example<T: ProvenEq>(a: &T, b: &T, c: &T)
    requires
        a.eq_spec(b),
        b.eq_spec(c),
    ensures
        a.eq_spec(c),
{
}

} // verus!
}
