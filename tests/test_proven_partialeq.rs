// Copyright (c) 2025 Brian G. Milnes
//! Runtime tests for ProvenPartialEq

use verus_relations::proven_partialeq::proven_partialeq::{ProvenPartialEq, MyInt};
use verus_relations::use_proven_partialeq::use_proven_partialeq::{are_equal, EqPair};

#[test]
fn test_i32_eq() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 17;
    
    assert!(ProvenPartialEq::eq(&a, &b));
    assert!(!ProvenPartialEq::eq(&a, &c));
}

#[test]
fn test_i32_ne() {
    let a: i32 = 42;
    let b: i32 = 42;
    let c: i32 = 17;
    
    assert!(!ProvenPartialEq::ne(&a, &b));
    assert!(ProvenPartialEq::ne(&a, &c));
}

#[test]
fn test_myint_eq() {
    let a = MyInt { val: 42 };
    let b = MyInt { val: 42 };
    let c = MyInt { val: 17 };
    
    assert!(a.eq(&b));
    assert!(!a.eq(&c));
}

#[test]
fn test_myint_ne() {
    let a = MyInt { val: 42 };
    let b = MyInt { val: 42 };
    let c = MyInt { val: 17 };
    
    assert!(!a.ne(&b));
    assert!(a.ne(&c));
}

#[test]
fn test_are_equal_generic() {
    let a: i32 = 100;
    let b: i32 = 100;
    let c: i32 = 200;
    
    assert!(are_equal(&a, &b));
    assert!(!are_equal(&a, &c));
}

#[test]
fn test_eq_pair() {
    let pair_same = EqPair { first: 5i32, second: 5i32 };
    let pair_diff = EqPair { first: 5i32, second: 10i32 };
    
    assert!(pair_same.are_same());
    assert!(!pair_diff.are_same());
}

#[test]
fn test_eq_pair_myint() {
    let pair_same = EqPair { first: MyInt { val: 7 }, second: MyInt { val: 7 } };
    let pair_diff = EqPair { first: MyInt { val: 7 }, second: MyInt { val: 14 } };
    
    assert!(pair_same.are_same());
    assert!(!pair_diff.are_same());
}

