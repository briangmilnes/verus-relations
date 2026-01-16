//! Runtime tests for ProvenEq

use verus_relations::proven_eq::proven_eq::{ProvenEq, MyInt};
use verus_relations::use_proven_eq::use_proven_eq::{are_equal, EqPair};

#[test]
fn test_i32_eq() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 99;
    
    // Test equality
    assert!(ProvenEq::eq(&a, &b));
    assert!(!ProvenEq::eq(&a, &c));
    
    // Test inequality
    assert!(!ProvenEq::ne(&a, &b));
    assert!(ProvenEq::ne(&a, &c));
}

#[test]
fn test_myint_eq() {
    let a = MyInt { val: 42 };
    let b = MyInt { val: 42 };
    let c = MyInt { val: 99 };
    
    // Test equality
    assert!(a.eq(&b));
    assert!(!a.eq(&c));
    
    // Test inequality
    assert!(!a.ne(&b));
    assert!(a.ne(&c));
}

#[test]
fn test_generic_are_equal() {
    let a: i32 = 10;
    let b: i32 = 10;
    let c: i32 = 20;
    
    assert!(are_equal(&a, &b));
    assert!(!are_equal(&a, &c));
}

#[test]
fn test_eq_pair() {
    let same_pair: EqPair<i32> = EqPair { first: 5, second: 5 };
    let diff_pair: EqPair<i32> = EqPair { first: 5, second: 10 };
    
    assert!(same_pair.are_same());
    assert!(!diff_pair.are_same());
}

#[test]
fn test_reflexivity_at_runtime() {
    // At runtime, reflexivity should hold for i32
    let values = vec![0, 1, -1, i32::MAX, i32::MIN, 42, -42];
    
    for v in values {
        assert!(ProvenEq::eq(&v, &v), "reflexivity failed for {}", v);
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
        let ab = ProvenEq::eq(&a, &b);
        let ba = ProvenEq::eq(&b, &a);
        assert_eq!(ab, ba, "symmetry failed for ({}, {})", a, b);
    }
}

#[test]
fn test_transitivity_at_runtime() {
    // If a == b and b == c, then a == c
    let a: i32 = 7;
    let b: i32 = 7;
    let c: i32 = 7;
    
    assert!(ProvenEq::eq(&a, &b));
    assert!(ProvenEq::eq(&b, &c));
    assert!(ProvenEq::eq(&a, &c));
    
    // Also check that unequal chains don't falsely trigger
    let x: i32 = 1;
    let y: i32 = 2;
    let z: i32 = 2;
    
    assert!(!ProvenEq::eq(&x, &y));
    assert!(ProvenEq::eq(&y, &z));
    assert!(!ProvenEq::eq(&x, &z));  // transitivity doesn't apply when first link fails
}

