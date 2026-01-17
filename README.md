# verus-relations

Formally verified relation traits for [Verus](https://github.com/verus-lang/verus).

## The Problem

Rust's comparison trait hierarchy (`PartialEq`, `Eq`, `PartialOrd`, `Ord`) has a design flaw:
to accommodate IEEE 754 floating-point NaN semantics, the entire trait system was weakened.

- `PartialEq` doesn't require reflexivity (because `NaN != NaN`)
- `Eq` is just a marker trait—"trust me, I'm reflexive"
- `PartialOrd` returns `Option<Ordering>` (because `NaN` is incomparable)
- `Ord` is also a marker trait—"trust me, I'm total"

**The exception drove the rule into a ditch.** 99.99% of types are perfectly well-behaved,
but the 0.01% (floats) weakened the guarantees for everyone.

Rust documents the required mathematical properties but cannot verify them:

```rust
// This compiles and is WRONG
impl Ord for MyType {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Less  // Always? Rust can't stop you.
    }
}
```

See [docs/SpecifyingRustRelations.md](docs/SpecifyingRustRelations.md) for a detailed analysis.

## The Solution

This library provides **proven** variants of Rust's comparison traits that require
formal proofs of the mathematical properties:

| Trait | Proves | Extends |
|-------|--------|---------|
| `ProvenPartialEq` | symmetry, transitivity | `PartialEqSpec` |
| `ProvenEq` | + reflexivity | `Eq + ProvenPartialEq` |
| `ProvenPartialOrd` | transitivity, duality | `PartialOrdSpec` |
| `ProvenOrd` | + totality, antisymmetry, consistency with eq | `Ord + ProvenPartialOrd` |

Each trait comes with **broadcast lemmas** and a **broadcast group** that makes the
proven properties automatically available to the SMT solver—no manual lemma application needed.

## Example

```rust
use crate::proven_eq::proven_eq::ProvenEq;

// ProvenEq requires reflexivity, symmetry, transitivity
// i32 implements all of these with empty proof bodies (SMT knows arithmetic)
fn equivalence_class<T: ProvenEq>(a: &T, b: &T, c: &T)
    requires a.eq_spec(b), b.eq_spec(c)
    ensures a.eq_spec(c)  // Verus proves this automatically via broadcast group
{
    // No explicit proof calls needed!
}
```

## Implementations

All integer primitives implement the full trait stack:

| Type | ProvenPartialEq | ProvenEq | ProvenPartialOrd | ProvenOrd |
|------|-----------------|----------|------------------|-----------|
| `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | ✓ | ✓ | ✓ | ✓ |
| `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | ✓ | ✓ | ✓ | ✓ |

Note: `bool` and `char` are omitted because vstd doesn't provide the necessary specs.

## Usage with Collections

The `use_hash_map` module demonstrates using `HashMapWithView` with `ProvenEq` key types:

```rust
// ProvenEq bound ensures keys have verified equality properties
pub fn new_proven_hash_map<K: ProvenEq + View + Hash, V>() 
    -> HashMapWithView<K, V>
```

## Building

```bash
# Verify with Verus
cd ~/projects/verus-relations
~/projects/verus/source/target-verus/release/verus --crate-type=lib src/lib.rs

# Run tests
cargo test

# Format
~/projects/verusfmt/target/release/verusfmt src/*.rs
```

## Design

This library builds on vstd's existing external specifications for `PartialEq`, `Eq`,
`PartialOrd`, and `Ord`. Rather than duplicating specs, we require proofs of properties
on vstd's `eq_spec`, `partial_cmp_spec`, and `cmp_spec`.

The key insight: for well-behaved types like integers, the proofs are trivial (empty bodies)
because the SMT solver already knows arithmetic. For custom types, you must prove the
properties hold—which is exactly what Rust's marker traits should have required all along.

I did not think I could specify this in Verus, as I could in F*, but am very pleased
with Verus's expressiveness.

## Files

```
src/
├── proven_partialeq.rs    # ProvenPartialEq trait + impls
├── proven_eq.rs           # ProvenEq trait + impls  
├── proven_partialord.rs   # ProvenPartialOrd trait + impls
├── proven_ord.rs          # ProvenOrd trait + impls
├── use_proven_partialeq.rs # Usage examples
├── use_proven_eq.rs       # Usage examples
├── use_proven_partialord.rs # Usage examples
├── use_proven_ord.rs      # Usage examples
├── use_hash_map.rs        # HashMap with ProvenEq keys
├── complex.rs             # Custom type examples (Complex, ComplexEq)
└── lib.rs
```

## Status

- **228 verified**, 0 errors
- **17 runtime tests** pass

## License

Apache-2.0
