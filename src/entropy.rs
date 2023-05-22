//! Simple hash functions for property testing.

use crate::prelude::*;

/// Hashes anything hashable into a `u64`.
#[inline]
#[must_use]
pub fn hash<H: core::hash::Hash>(h: H) -> u64 {
    use core::hash::Hasher;
    let mut hasher = ahash::AHasher::default();
    h.hash(&mut hasher);
    hasher.finish()
}

/// Hashes anything hashable into a `u64` then calls `consume` on it.
#[inline]
#[must_use]
pub fn hash_consume<M: Monad<u64, Monad<u64> = M>, H: core::hash::Hash>(h: H) -> M {
    consume(hash(h))
}

/// Reverses bits.
#[inline]
#[must_use]
pub fn reverse(a: u64) -> u64 {
    a.reverse_bits()
}

/// Reverses bits then consumes into a monad.
#[inline]
#[must_use]
pub fn reverse_consume<M: Monad<u64>>(a: u64) -> M {
    consume(reverse(a))
}
