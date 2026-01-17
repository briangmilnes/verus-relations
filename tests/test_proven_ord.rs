//! RTTs for ProvenOrd (extends Ord with proofs)

use std::cmp::Ordering;

#[test]
fn test_i32_less() {
    let a: i32 = 1;
    let b: i32 = 5;
    // Use Rust's Ord
    assert_eq!(a.cmp(&b), Ordering::Less);
}

#[test]
fn test_i32_greater() {
    let a: i32 = 10;
    let b: i32 = 3;
    assert_eq!(a.cmp(&b), Ordering::Greater);
}

#[test]
fn test_i32_equal() {
    let a: i32 = 7;
    let b: i32 = 7;
    assert_eq!(a.cmp(&b), Ordering::Equal);
}

#[test]
fn test_totality() {
    // Total order always returns Some when using partial_cmp
    let a: i32 = 42;
    let b: i32 = 17;
    assert!(a.partial_cmp(&b).is_some());
}

#[test]
fn test_antisymmetry() {
    // a <= b and b <= a implies a == b
    let a: i32 = 5;
    let b: i32 = 5;
    assert!(a.cmp(&b) != Ordering::Greater);
    assert!(b.cmp(&a) != Ordering::Greater);
    assert_eq!(a.cmp(&b), Ordering::Equal);
}
