//! Haskell's monad laws as testable properties.

#![allow(clippy::arithmetic_side_effects)]

use crate::prelude::*;
use core::ops::Shr;

// https://wiki.haskell.org/Monad_laws

/// Tests that `M::consume(a) >> f == f(a)`.
#[inline]
pub fn left_identity<A: Clone, B, M: Monad<A>, F: Fn(A) -> M::M<B>>(a: A, f: &'static F) -> bool
where
    M::M<A>: Shr<&'static F, Output = M::M<B>>,
    M::M<B>: PartialEq,
{
    (M::consume(a.clone()) >> f) == f(a)
}

/// Tests that `m >> M::consume == m`.
#[inline]
#[allow(clippy::needless_pass_by_value)]
pub fn right_identity<A, M: Monad<A> + Clone>(m: M) -> bool
where
    M::M<A>: PartialEq<M>,
{
    // (m >> M::consume) == m
    M::bind(m.clone(), M::consume) == m
}

/// Tests that `m >> f >> g == m >> |a| { f(a) >> g }`.
#[inline]
#[allow(clippy::trait_duplication_in_bounds)]
pub fn associativity<
    A,
    B,
    C,
    M: Monad<A> + Clone + Shr<&'static F, Output = M::M<B>> + Shr<fn(A) -> M::M<C>, Output = M::M<C>>,
    F: Fn(A) -> M::M<B>,
    G: Fn(B) -> M::M<C>,
>(
    m: M,
    f: &'static F,
    g: &'static G,
) -> bool
where
    M::M<B>: Shr<&'static G, Output = M::M<C>>,
    M::M<C>: PartialEq,
{
    #![allow(clippy::as_conversions)]
    ((m.clone() >> f) >> g) == M::bind(m, move |a| (f(a) >> g))
}

/// Hashes anything hashable into a `u64`.
#[inline]
pub fn hash<H: core::hash::Hash>(h: H) -> u64 {
    let mut hasher = ahash::AHasher::default();
    core::hash::Hash::hash(&h, &mut hasher);
    core::hash::Hasher::finish(&hasher)
}

/// Hashes anything hashable into a `u64` then calls `consume` on it.
#[inline]
pub fn hash_consume<M: crate::monad::Monad<u64>, H: core::hash::Hash>(h: H) -> M::M<u64> {
    M::consume(hash(h))
}

// It's surprisingly difficult to write anything that does *not* pass the monad laws and still compiles
