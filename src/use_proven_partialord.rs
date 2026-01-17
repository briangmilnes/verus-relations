//! Demonstrates using ProvenPartialOrd as a trait bound.
pub mod use_proven_partialord {
    #[cfg(verus_keep_ghost)]
    use crate::proven_partialord::proven_partialord::group_proven_partialord;
    #[cfg(verus_keep_ghost)]
    use crate::proven_partialord::proven_partialord::ProvenPartialOrd;
    #[cfg(verus_keep_ghost)]
    use core::cmp::Ordering;
    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    verus! {

broadcast use group_proven_partialord;

pub fn compare<T: ProvenPartialOrd>(a: &T, b: &T) -> (result: Option<Ordering>)
    requires
        T::obeys_partial_cmp_spec(),
    ensures
        result == a.partial_cmp_spec(b),
{
    PartialOrd::partial_cmp(a, b)
}

pub fn duality_example<T: ProvenPartialOrd>(a: &T, b: &T)
    requires
        a.partial_cmp_spec(b) == Some(Ordering::Less),
    ensures
        b.partial_cmp_spec(a) == Some(Ordering::Greater),
{
}

pub fn transitivity_example<T: ProvenPartialOrd>(a: &T, b: &T, c: &T)
    requires
        a.partial_cmp_spec(b) == Some(Ordering::Less),
        b.partial_cmp_spec(c) == Some(Ordering::Less),
    ensures
        a.partial_cmp_spec(c) == Some(Ordering::Less),
{
}

} // verus!
}
