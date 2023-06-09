/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Crazy test cases that will seem like magic.

#![allow(clippy::arithmetic_side_effects, clippy::panic)]

use crate::prelude::*;

#[test]
fn powers_of_two() {
    assert_eq!(
        vec![0, 1, 2, 3, 4, 5].fmap(u8::is_power_of_two),
        vec![false, true, true, false, true, false]
    );
}

#[test]
fn monoidal_sum() {
    assert_eq!(vec![1, 2, 3, 4, 5].fmap(SumU8).unify(), SumU8(15));
}

#[test]
fn first_valid_int() {
    // The monadic type system not only works but is so strong we can do this with zero annotations:
    let inputs = lazy_list![
        "this string isn't a number",
        "67",
        "that was, but not binary",
        "1010101010",
        "that was, but more than 8 bits",
        "101010",
        "that one should work!",
        panic!("lazy evaluation!")
    ];
    assert_eq!(
        (inputs % |x| move || u8::from_str_radix(x(), 2).ok()).asum(),
        Some(42)
    );
}

#[test]
fn first_valid_int_vec() {
    assert_eq!(
        vec![
            || "this string isn't a number",
            || "67",
            || "that was, but not binary",
            || "1010101010",
            || "that was, but more than 8 bits",
            || "101010",
            || "that one should work!",
            || panic!("lazy evaluation!"),
        ]
        .fmap(|x| move || u8::from_str_radix(x(), 2).ok())
        .asum(),
        Some(42)
    );
}

#[test]
fn is_power_of_two() {
    assert_eq!(
        list![0, 1, 2, 3, 4, 5] % u8::is_power_of_two,
        list![false, true, true, false, true, false]
    );
}

#[test]
fn pythagoraean_triples() {
    assert_eq!(
        list![1..20]
            >> |x| {
                list![{ x }..20]
                    >> |y| {
                        list![{ y }..20]
                            >> |z| guard::<List<()>>(x * x + y * y == z * z) >> |_| list![(x, y, z)]
                    }
            },
        list![
            // Including "non-primitive" (i.e. multiples of smaller) triples
            (3, 4, 5),
            (5, 12, 13),
            (6, 8, 10),
            (8, 15, 17),
            (9, 12, 15)
        ]
    );
}
