//! Demonstrates using vstd's HashMap with ProvenEq key types.
//!
//! ProvenEq ensures the key type has verified equality properties,
//! which strengthens the HashMap's correctness guarantees.
pub mod use_hash_map {
    #[cfg(verus_keep_ghost)]
    use crate::proven_eq::proven_eq::group_proven_eq;
    #[cfg(verus_keep_ghost)]
    use crate::proven_eq::proven_eq::ProvenEq;
    #[cfg(verus_keep_ghost)]
    use crate::proven_partialeq::proven_partialeq::group_proven_partialeq;
    #[cfg(verus_keep_ghost)]
    use core::hash::Hash;
    #[cfg(verus_keep_ghost)]
    use vstd::hash_map::HashMapWithView;
    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    verus! {

broadcast use {group_proven_partialeq, group_proven_eq};

/// Create a new empty HashMap with a ProvenEq key type.
/// The ProvenEq bound ensures the key type has verified equality axioms.
pub fn new_proven_hash_map<K: ProvenEq + View + Hash, V>() -> (result: HashMapWithView<K, V>) where
    <K as View>::V: core::marker::Sized,

    requires
        vstd::std_specs::hash::obeys_key_model::<K>(),
        forall|k1: K, k2: K| k1@ == k2@ ==> k1 == k2,
    ensures
        result@ == Map::<<K as View>::V, V>::empty(),
{
    HashMapWithView::new()
}

/// Insert into a HashMap with a ProvenEq key type.
pub fn insert_proven<K: ProvenEq + View + Hash, V>(
    map: &mut HashMapWithView<K, V>,
    key: K,
    value: V,
) where <K as View>::V: core::marker::Sized
    ensures
        map@ == old(map)@.insert(key@, value),
{
    map.insert(key, value);
}

/// Get from a HashMap with a ProvenEq key type.
pub fn get_proven<'a, K: ProvenEq + View + Hash, V>(
    map: &'a HashMapWithView<K, V>,
    key: &K,
) -> (result: Option<&'a V>) where <K as View>::V: core::marker::Sized
    ensures
        match result {
            Some(v) => map@.contains_key(key@) && *v == map@[key@],
            None => !map@.contains_key(key@),
        },
{
    map.get(key)
}

/// Demonstrate HashMap operations with i32 keys (which implement ProvenEq).
pub fn demo_i32_hash_map() {
    let mut map: HashMapWithView<i32, u64> = HashMapWithView::new();

    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);

    proof {
        assert(map@.contains_key(1));
        assert(map@.contains_key(2));
        assert(map@.contains_key(3));
        assert(map@[1] == 100);
        assert(map@[2] == 200);
        assert(map@[3] == 300);
    }

    let v = map.get(&2);
    proof {
        assert(v.is_some());
        assert(*v.unwrap() == 200);
    }

    map.remove(&2);
    proof {
        assert(!map@.contains_key(2));
        assert(map@.contains_key(1));
        assert(map@.contains_key(3));
    }
}

/// Unit type for map values
pub struct Unit;

/// Demonstrate HashMap<i32, Unit> - essentially a HashSet
pub fn demo_i32_to_unit_map() {
    let mut map: HashMapWithView<i32, Unit> = HashMapWithView::new();

    map.insert(42, Unit);
    map.insert(17, Unit);

    proof {
        assert(map@.contains_key(42));
        assert(map@.contains_key(17));
        assert(!map@.contains_key(0));
    }

    // Runtime checks
    let has_42 = map.contains_key(&42);
    let has_17 = map.contains_key(&17);
    let has_0 = map.contains_key(&0);

    proof {
        assert(has_42);
        assert(has_17);
        assert(!has_0);
    }
}

} // verus!
}
