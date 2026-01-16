//! Runtime tests for ProvenEq (now with Option<bool> results)

#[allow(unused_imports)]
use verus_relations::proven_eq::proven_eq::{ProvenEq, MyInt};
use verus_relations::proven_partialeq::proven_partialeq::ProvenPartialEq;
use verus_relations::use_proven_eq::use_proven_eq::{are_equal, EqPair};

#[test]
fn test_i32_eq() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 99;
    
    // Test equality (eq is on ProvenPartialEq, inherited by ProvenEq)
    assert_eq!(<i32 as ProvenPartialEq>::eq(&a, &b), Some(true));
    assert_eq!(<i32 as ProvenPartialEq>::eq(&a, &c), Some(false));
    
    // Test inequality
    assert_eq!(<i32 as ProvenPartialEq>::ne(&a, &b), Some(false));
    assert_eq!(<i32 as ProvenPartialEq>::ne(&a, &c), Some(true));
}

#[test]
fn test_myint_eq() {
    let a = MyInt { val: 42 };
    let b = MyInt { val: 42 };
    let c = MyInt { val: 99 };
    
    // Test equality
    assert_eq!(<MyInt as ProvenPartialEq>::eq(&a, &b), Some(true));
    assert_eq!(<MyInt as ProvenPartialEq>::eq(&a, &c), Some(false));
    
    // Test inequality
    assert_eq!(<MyInt as ProvenPartialEq>::ne(&a, &b), Some(false));
    assert_eq!(<MyInt as ProvenPartialEq>::ne(&a, &c), Some(true));
}

#[test]
fn test_generic_are_equal() {
    let a: i32 = 10;
    let b: i32 = 10;
    let c: i32 = 20;
    
    assert_eq!(are_equal(&a, &b), Some(true));
    assert_eq!(are_equal(&a, &c), Some(false));
}

#[test]
fn test_eq_pair() {
    let same_pair: EqPair<i32> = EqPair { first: 5, second: 5 };
    let diff_pair: EqPair<i32> = EqPair { first: 5, second: 10 };
    
    assert_eq!(same_pair.are_same(), Some(true));
    assert_eq!(diff_pair.are_same(), Some(false));
}

#[test]
fn test_reflexivity_at_runtime() {
    // At runtime, reflexivity should hold for i32
    let values = vec![0, 1, -1, i32::MAX, i32::MIN, 42, -42];
    
    for v in values {
        assert_eq!(
            <i32 as ProvenPartialEq>::eq(&v, &v), 
            Some(true), 
            "reflexivity failed for {}", v
        );
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
        let ab = <i32 as ProvenPartialEq>::eq(&a, &b);
        let ba = <i32 as ProvenPartialEq>::eq(&b, &a);
        assert_eq!(ab, ba, "symmetry failed for ({}, {})", a, b);
    }
}

#[test]
fn test_transitivity_at_runtime() {
    // If a == b and b == c, then a == c
    let a: i32 = 7;
    let b: i32 = 7;
    let c: i32 = 7;
    
    assert_eq!(<i32 as ProvenPartialEq>::eq(&a, &b), Some(true));
    assert_eq!(<i32 as ProvenPartialEq>::eq(&b, &c), Some(true));
    assert_eq!(<i32 as ProvenPartialEq>::eq(&a, &c), Some(true));
    
    // Also check that unequal chains don't falsely trigger
    let x: i32 = 1;
    let y: i32 = 2;
    let z: i32 = 2;
    
    assert_eq!(<i32 as ProvenPartialEq>::eq(&x, &y), Some(false));
    assert_eq!(<i32 as ProvenPartialEq>::eq(&y, &z), Some(true));
    assert_eq!(<i32 as ProvenPartialEq>::eq(&x, &z), Some(false));  // transitivity doesn't apply when first link fails
}
