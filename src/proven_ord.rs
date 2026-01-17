//! ProvenOrd: Certifies that an Ord impl is well-behaved.
//!
//! Uses vstd's cmp_spec, requires proofs of totality, antisymmetry,
//! consistency with eq, and full transitivity.

pub mod proven_ord {
    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::{OrdSpec, PartialEqSpec};
    #[cfg(verus_keep_ghost)]
    use core::cmp::Ordering;
    #[cfg(verus_keep_ghost)]
    use crate::proven_partialord::proven_partialord::ProvenPartialOrd;

#[cfg(verus_keep_ghost)]
verus! {

/// ProvenOrd certifies that an Ord impl is well-behaved:
/// by proving totality, antisymmetry, consistency with eq, and transitivity.
pub trait ProvenOrd: OrdSpec + ProvenPartialOrd {
    proof fn proof_obeys_cmp_spec()
        ensures Self::obeys_cmp_spec();

    // Totality: partial_cmp always returns Some for total orders
    proof fn proof_totality()
        ensures forall |x: &Self, y: &Self| 
            #[trigger] x.partial_cmp_spec(y) == Some(x.cmp_spec(y));

    // Antisymmetry: a <= b && b <= a ==> a == b
    proof fn proof_antisymmetry()
        ensures forall |x: &Self, y: &Self|
            #![trigger x.cmp_spec(y), y.cmp_spec(x)]
            (x.cmp_spec(y) != Ordering::Greater && y.cmp_spec(x) != Ordering::Greater)
                ==> x.cmp_spec(y) == Ordering::Equal;

    // Consistency with Eq: cmp(a, b) == Equal iff a.eq_spec(b)
    proof fn proof_consistency_with_eq()
        ensures forall |x: &Self, y: &Self|
            #![trigger x.cmp_spec(y), x.eq_spec(y)]
            (x.cmp_spec(y) == Ordering::Equal) <==> x.eq_spec(y);

    // Transitivity for Less: a < b && b < c ==> a < c
    proof fn proof_cmp_transitivity_lt()
        ensures forall |x: &Self, y: &Self, z: &Self|
            #![trigger x.cmp_spec(y), y.cmp_spec(z)]
            (x.cmp_spec(y) == Ordering::Less && y.cmp_spec(z) == Ordering::Less)
                ==> x.cmp_spec(z) == Ordering::Less;

    // Transitivity for LessOrEqual: a <= b && b <= c ==> a <= c
    proof fn proof_cmp_transitivity_le()
        ensures forall |x: &Self, y: &Self, z: &Self|
            #![trigger x.cmp_spec(y), y.cmp_spec(z)]
            (x.cmp_spec(y) != Ordering::Greater && y.cmp_spec(z) != Ordering::Greater)
                ==> x.cmp_spec(z) != Ordering::Greater;

    // Transitivity for Greater: a > b && b > c ==> a > c
    proof fn proof_cmp_transitivity_gt()
        ensures forall |x: &Self, y: &Self, z: &Self|
            #![trigger x.cmp_spec(y), y.cmp_spec(z)]
            (x.cmp_spec(y) == Ordering::Greater && y.cmp_spec(z) == Ordering::Greater)
                ==> x.cmp_spec(z) == Ordering::Greater;

    // Transitivity for GreaterOrEqual: a >= b && b >= c ==> a >= c
    proof fn proof_cmp_transitivity_ge()
        ensures forall |x: &Self, y: &Self, z: &Self|
            #![trigger x.cmp_spec(y), y.cmp_spec(z)]
            (x.cmp_spec(y) != Ordering::Less && y.cmp_spec(z) != Ordering::Less)
                ==> x.cmp_spec(z) != Ordering::Less;
}

// Broadcast lemmas
pub broadcast proof fn lemma_obeys_cmp_spec<T: ProvenOrd>()
    ensures #[trigger] T::obeys_cmp_spec()
{
    T::proof_obeys_cmp_spec();
}

pub broadcast proof fn lemma_totality<T: ProvenOrd>(x: &T, y: &T)
    ensures #[trigger] x.partial_cmp_spec(y) == Some(x.cmp_spec(y))
{
    T::proof_totality();
}

pub broadcast proof fn lemma_antisymmetry<T: ProvenOrd>(x: &T, y: &T)
    requires 
        #[trigger] x.cmp_spec(y) != Ordering::Greater,
        #[trigger] y.cmp_spec(x) != Ordering::Greater
    ensures x.cmp_spec(y) == Ordering::Equal
{
    T::proof_antisymmetry();
}

pub broadcast proof fn lemma_consistency_with_eq<T: ProvenOrd>(x: &T, y: &T)
    ensures (#[trigger] x.cmp_spec(y) == Ordering::Equal) <==> x.eq_spec(y)
{
    T::proof_consistency_with_eq();
}

pub broadcast proof fn lemma_cmp_transitivity_lt<T: ProvenOrd>(x: &T, y: &T, z: &T)
    requires 
        #[trigger] x.cmp_spec(y) == Ordering::Less,
        #[trigger] y.cmp_spec(z) == Ordering::Less
    ensures x.cmp_spec(z) == Ordering::Less
{
    T::proof_cmp_transitivity_lt();
}

pub broadcast proof fn lemma_cmp_transitivity_le<T: ProvenOrd>(x: &T, y: &T, z: &T)
    requires 
        #[trigger] x.cmp_spec(y) != Ordering::Greater,
        #[trigger] y.cmp_spec(z) != Ordering::Greater
    ensures x.cmp_spec(z) != Ordering::Greater
{
    T::proof_cmp_transitivity_le();
}

pub broadcast proof fn lemma_cmp_transitivity_gt<T: ProvenOrd>(x: &T, y: &T, z: &T)
    requires 
        #[trigger] x.cmp_spec(y) == Ordering::Greater,
        #[trigger] y.cmp_spec(z) == Ordering::Greater
    ensures x.cmp_spec(z) == Ordering::Greater
{
    T::proof_cmp_transitivity_gt();
}

pub broadcast proof fn lemma_cmp_transitivity_ge<T: ProvenOrd>(x: &T, y: &T, z: &T)
    requires 
        #[trigger] x.cmp_spec(y) != Ordering::Less,
        #[trigger] y.cmp_spec(z) != Ordering::Less
    ensures x.cmp_spec(z) != Ordering::Less
{
    T::proof_cmp_transitivity_ge();
}

pub broadcast group group_proven_ord {
    lemma_obeys_cmp_spec,
    lemma_totality,
    lemma_antisymmetry,
    lemma_consistency_with_eq,
    lemma_cmp_transitivity_lt,
    lemma_cmp_transitivity_le,
    lemma_cmp_transitivity_gt,
    lemma_cmp_transitivity_ge,
}

// Signed integers
impl ProvenOrd for i8 {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

impl ProvenOrd for i16 {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

impl ProvenOrd for i32 {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

impl ProvenOrd for i64 {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

impl ProvenOrd for i128 {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

impl ProvenOrd for isize {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

// Unsigned integers
impl ProvenOrd for u8 {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

impl ProvenOrd for u16 {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

impl ProvenOrd for u32 {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

impl ProvenOrd for u64 {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

impl ProvenOrd for u128 {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

impl ProvenOrd for usize {
    proof fn proof_obeys_cmp_spec() {}
    proof fn proof_totality() {}
    proof fn proof_antisymmetry() {}
    proof fn proof_consistency_with_eq() {}
    proof fn proof_cmp_transitivity_lt() {}
    proof fn proof_cmp_transitivity_le() {}
    proof fn proof_cmp_transitivity_gt() {}
    proof fn proof_cmp_transitivity_ge() {}
}

// Note: bool and char are omitted because vstd doesn't provide OrdSpec for them

} // verus!
}
