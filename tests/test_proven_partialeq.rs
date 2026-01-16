//! Runtime tests for ProvenPartialEq (now with Option<bool> results)

use verus_relations::proven_partialeq::proven_partialeq::{ProvenPartialEq, MyInt};
use verus_relations::use_proven_partialeq::use_proven_partialeq::{are_equal, EqPair};

//
// Tests for convenience methods: is_eq, is_ne, is_comparable
//

#[test]
fn test_is_eq() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 99;
    
    assert!(<i32 as ProvenPartialEq>::is_eq(&a, &b));
    assert!(!<i32 as ProvenPartialEq>::is_eq(&a, &c));
}

#[test]
fn test_is_ne() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 99;
    
    assert!(!<i32 as ProvenPartialEq>::is_ne(&a, &b));
    assert!(<i32 as ProvenPartialEq>::is_ne(&a, &c));
}

#[test]
fn test_is_comparable() {
    let a: i32 = 42;
    let b: i32 = 99;
    
    // i32 is always comparable
    assert!(<i32 as ProvenPartialEq>::is_comparable(&a, &b));
    assert!(<i32 as ProvenPartialEq>::is_comparable(&a, &a));
}

#[test]
fn test_myint_convenience_methods() {
    let a = MyInt { val: 10 };
    let b = MyInt { val: 10 };
    let c = MyInt { val: 20 };
    
    assert!(<MyInt as ProvenPartialEq>::is_eq(&a, &b));
    assert!(!<MyInt as ProvenPartialEq>::is_eq(&a, &c));
    
    assert!(!<MyInt as ProvenPartialEq>::is_ne(&a, &b));
    assert!(<MyInt as ProvenPartialEq>::is_ne(&a, &c));
    
    assert!(<MyInt as ProvenPartialEq>::is_comparable(&a, &c));
}

//
// Original tests for eq/ne returning Option<bool>
//

#[test]
fn test_i32_eq() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 99;
    
    // Test equality - now returns Option<bool>
    assert_eq!(<i32 as ProvenPartialEq>::eq(&a, &b), Some(true));
    assert_eq!(<i32 as ProvenPartialEq>::eq(&a, &c), Some(false));
}

#[test]
fn test_i32_ne() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 99;
    
    // Test inequality - now returns Option<bool>
    assert_eq!(<i32 as ProvenPartialEq>::ne(&a, &b), Some(false));
    assert_eq!(<i32 as ProvenPartialEq>::ne(&a, &c), Some(true));
}

#[test]
fn test_myint_eq() {
    let a = MyInt { val: 42 };
    let b = MyInt { val: 42 };
    let c = MyInt { val: 99 };
    
    assert_eq!(<MyInt as ProvenPartialEq>::eq(&a, &b), Some(true));
    assert_eq!(<MyInt as ProvenPartialEq>::eq(&a, &c), Some(false));
}

#[test]
fn test_myint_ne() {
    let a = MyInt { val: 42 };
    let b = MyInt { val: 42 };
    let c = MyInt { val: 99 };
    
    assert_eq!(<MyInt as ProvenPartialEq>::ne(&a, &b), Some(false));
    assert_eq!(<MyInt as ProvenPartialEq>::ne(&a, &c), Some(true));
}

#[test]
fn test_are_equal_generic() {
    let a: i32 = 10;
    let b: i32 = 10;
    let c: i32 = 20;
    
    assert_eq!(are_equal(&a, &b), Some(true));
    assert_eq!(are_equal(&a, &c), Some(false));
}

#[test]
fn test_eq_pair() {
    let same: EqPair<i32> = EqPair { first: 5, second: 5 };
    let diff: EqPair<i32> = EqPair { first: 5, second: 10 };
    
    assert_eq!(same.are_same(), Some(true));
    assert_eq!(diff.are_same(), Some(false));
}

#[test]
fn test_eq_pair_myint() {
    let same: EqPair<MyInt> = EqPair { 
        first: MyInt { val: 7 }, 
        second: MyInt { val: 7 } 
    };
    let diff: EqPair<MyInt> = EqPair { 
        first: MyInt { val: 7 }, 
        second: MyInt { val: 8 } 
    };
    
    assert_eq!(same.are_same(), Some(true));
    assert_eq!(diff.are_same(), Some(false));
}
