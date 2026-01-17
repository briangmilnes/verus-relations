//! Runtime tests for PartialEq (ProvenPartialEq is verification-only)

#[test]
fn test_i32_eq() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 99;
    
    // Use Rust's == operator (from PartialEq)
    assert!(a == b);
    assert!(a != c);
}

#[test]
fn test_i32_symmetry() {
    let a: i32 = 42;
    let b: i32 = 99;
    
    assert_eq!(a == b, b == a);
    assert_eq!(a == a, a == a);
}

#[test]
fn test_i32_transitivity() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 42;
    
    assert!(a == b);
    assert!(b == c);
    assert!(a == c);
}

// ProvenPartialEq traits and their impls (MyInt, are_equal, EqPair)
// are verification-only (cfg(verus_keep_ghost)) and not available at runtime.
