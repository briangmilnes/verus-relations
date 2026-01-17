//! ProvenPartialOrd: Certifies that a PartialOrd impl is well-behaved.
//!
//! Uses vstd's partial_cmp_spec, requires proofs of transitivity and duality.

pub mod proven_partialord {
    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdSpec;
    #[cfg(verus_keep_ghost)]
    use core::cmp::Ordering;

#[cfg(verus_keep_ghost)]
verus! {

/// ProvenPartialOrd certifies that a PartialOrd impl is well-behaved:
/// by proving transitive and dual.
pub trait ProvenPartialOrd: PartialOrdSpec + Sized {
    proof fn proof_obeys_partial_cmp_spec()
        ensures Self::obeys_partial_cmp_spec();

    proof fn proof_transitivity_lt()
        ensures forall |x: &Self, y: &Self, z: &Self|
            #![trigger x.partial_cmp_spec(y), y.partial_cmp_spec(z)]
            (x.partial_cmp_spec(y) == Some(Ordering::Less) && y.partial_cmp_spec(z) == Some(Ordering::Less))
                ==> x.partial_cmp_spec(z) == Some(Ordering::Less);

    // Duality: a < b iff b > a
    proof fn proof_duality()
        ensures forall |x: &Self, y: &Self| #[trigger] x.partial_cmp_spec(y) ==
            match y.partial_cmp_spec(x) {
                Some(Ordering::Less)    => Some(Ordering::Greater),
                Some(Ordering::Greater) => Some(Ordering::Less),
                Some(Ordering::Equal)   => Some(Ordering::Equal),
                None                    => None,
            };
}

// Broadcast lemmas
pub broadcast proof fn lemma_obeys_partial_cmp_spec<T: ProvenPartialOrd>()
    ensures #[trigger] T::obeys_partial_cmp_spec()
{
    T::proof_obeys_partial_cmp_spec();
}

pub broadcast proof fn lemma_transitivity_lt<T: ProvenPartialOrd>(x: &T, y: &T, z: &T)
    requires 
        #[trigger] x.partial_cmp_spec(y) == Some(Ordering::Less),
        #[trigger] y.partial_cmp_spec(z) == Some(Ordering::Less)
    ensures x.partial_cmp_spec(z) == Some(Ordering::Less)
{
    T::proof_transitivity_lt();
}

pub broadcast proof fn lemma_duality<T: ProvenPartialOrd>(x: &T, y: &T)
    ensures #[trigger] x.partial_cmp_spec(y) ==
        match y.partial_cmp_spec(x) {
            Some(Ordering::Less)    => Some(Ordering::Greater),
            Some(Ordering::Greater) => Some(Ordering::Less),
            Some(Ordering::Equal)   => Some(Ordering::Equal),
            None                    => None,
        }
{
    T::proof_duality();
}

pub broadcast group group_proven_partialord {
    lemma_obeys_partial_cmp_spec,
    lemma_transitivity_lt,
    lemma_duality,
}

// Signed integers
impl ProvenPartialOrd for i8 {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

impl ProvenPartialOrd for i16 {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

impl ProvenPartialOrd for i32 {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

impl ProvenPartialOrd for i64 {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

impl ProvenPartialOrd for i128 {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

impl ProvenPartialOrd for isize {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

// Unsigned integers
impl ProvenPartialOrd for u8 {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

impl ProvenPartialOrd for u16 {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

impl ProvenPartialOrd for u32 {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

impl ProvenPartialOrd for u64 {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

impl ProvenPartialOrd for u128 {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

impl ProvenPartialOrd for usize {
    proof fn proof_obeys_partial_cmp_spec() {}
    proof fn proof_transitivity_lt() {}
    proof fn proof_duality() {}
}

// Note: bool and char are omitted because vstd doesn't provide PartialOrdSpec for them

} // verus!
}
