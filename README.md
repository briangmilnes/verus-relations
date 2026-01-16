# verus-relations

Verified relations (equality, ordering) for [Verus](https://github.com/verus-lang/verus).

## Overview

This library provides traits that require proofs of mathematical properties:

- **ProvenPartialEq**: Requires proofs of reflexivity, symmetry, and transitivity for equality

## Building

```bash
# Verify with Verus
cd ~/projects/verus-relations
~/projects/verus/source/target-verus/release/verus --crate-type=lib src/lib.rs
```

## License

Apache-2.0

