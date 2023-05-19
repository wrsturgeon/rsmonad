//! Haskell's monad laws as testable properties.

#![allow(clippy::arithmetic_side_effects)]

use crate::prelude::*;
use core::ops::Shr;

// https://wiki.haskell.org/Monad_laws

/// Tests that `M::consume(a) >> f == f(a)`.
#[inline]
pub fn left_identity<
    A: Clone,
    B,
    M: Monad<A, Hkt<A> = M> + Shr<&'static F, Output = M::Hkt<B>>,
    F: Fn(A) -> M::Hkt<B>,
>(
    a: A,
    f: &'static F,
) -> bool
where
    M::Hkt<B>: PartialEq,
{
    (M::consume(a.clone()) >> f) == f(a)
}

/// Tests that `m >> M::consume == m`.
#[inline]
#[allow(clippy::needless_pass_by_value)]
pub fn right_identity<A, M: Monad<A, Hkt<A> = M> + Clone + PartialEq>(m: M) -> bool {
    m.clone().bind::<A, _>(M::consume) == m
}

/// Tests that `m >> f >> g == m >> |a| { f(a) >> g }`.
#[inline]
#[allow(clippy::trait_duplication_in_bounds)]
pub fn associativity<
    A,
    B,
    C,
    M: Monad<A, Hkt<A> = M>
        + Clone
        + Shr<&'static F, Output = M::Hkt<B>>
        + Shr<fn(A) -> M::Hkt<C>, Output = M::Hkt<C>>,
    F: Fn(A) -> M::Hkt<B>,
    G: Fn(B) -> M::Hkt<C>,
>(
    m: M,
    f: &'static F,
    g: &'static G,
) -> bool
where
    M::Hkt<B>: Shr<&'static G, Output = M::Hkt<C>>,
    M::Hkt<C>: PartialEq,
{
    #![allow(clippy::as_conversions)]
    ((m.clone() >> f) >> g) == m.bind::<C, _>(move |a| (f(a) >> g))
}

/// Hashes anything hashable into a `u64`.
#[inline]
pub fn hash<H: core::hash::Hash>(h: H) -> u64 {
    use core::hash::Hasher;
    let mut hasher = ahash::AHasher::default();
    h.hash(&mut hasher);
    hasher.finish()
}

/// Hashes anything hashable into a `u64` then calls `consume` on it.
#[inline]
pub fn hash_consume<M: crate::monad::Monad<u64, Hkt<u64> = M>, H: core::hash::Hash>(h: H) -> M {
    consume(hash(h))
}

// It's surprisingly difficult to write anything that does *not* pass the monad laws and still compiles
