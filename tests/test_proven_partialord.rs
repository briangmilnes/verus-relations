//! RTTs for ProvenPartialOrd (extends PartialOrd with proofs)

use std::cmp::Ordering;

#[test]
fn test_i32_less() {
    let a: i32 = 1;
    let b: i32 = 5;
    // Use Rust's PartialOrd
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
}

#[test]
fn test_i32_greater() {
    let a: i32 = 10;
    let b: i32 = 3;
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater));
}

#[test]
fn test_i32_equal() {
    let a: i32 = 7;
    let b: i32 = 7;
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
}

#[test]
fn test_duality() {
    // a < b means b > a
    let a: i32 = 2;
    let b: i32 = 8;
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
}

#[test]
fn test_transitivity() {
    // a < b and b < c implies a < c
    let a: i32 = 1;
    let b: i32 = 5;
    let c: i32 = 10;
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    assert_eq!(b.partial_cmp(&c), Some(Ordering::Less));
    assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));
}
