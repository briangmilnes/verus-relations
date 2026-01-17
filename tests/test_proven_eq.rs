//! Runtime tests for Eq (ProvenEq is verification-only)

#[test]
fn test_i32_eq() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 99;
    
    // Use Rust's == operator (from PartialEq/Eq)
    assert!(a == b);
    assert!(a != c);
}

#[test]
fn test_reflexivity_at_runtime() {
    // At runtime, reflexivity should hold for i32
    let values = vec![0, 1, -1, i32::MAX, i32::MIN, 42, -42];
    
    for v in values {
        assert!(v == v, "reflexivity failed for {}", v);
    }
}

#[test]
fn test_symmetry_at_runtime() {
    let pairs = vec![
        (1, 1),
        (1, 2),
        (42, 42),
        (-5, 5),
    ];
    
    for (a, b) in pairs {
        assert_eq!(a == b, b == a, "symmetry failed for ({}, {})", a, b);
    }
}

#[test]
fn test_transitivity_at_runtime() {
    // If a == b and b == c, then a == c
    let a: i32 = 7;
    let b: i32 = 7;
    let c: i32 = 7;
    
    assert!(a == b);
    assert!(b == c);
    assert!(a == c);
    
    // Also check that unequal chains don't falsely trigger
    let x: i32 = 1;
    let y: i32 = 2;
    let z: i32 = 2;
    
    assert!(x != y);
    assert!(y == z);
    assert!(x != z);  // transitivity doesn't apply when first link fails
}

// ProvenEq traits and their impls (MyInt, are_equal, EqPair)
// are verification-only (cfg(verus_keep_ghost)) and not available at runtime.
