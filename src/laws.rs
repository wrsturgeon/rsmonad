//! Haskell's monad laws as testable properties.

#![allow(clippy::arithmetic_side_effects)]

use crate::prelude::*;

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
pub fn hash_consume<M: crate::monad::Monad<u64, Monad<u64> = M>, H: core::hash::Hash>(h: H) -> M {
    consume(hash(h))
}

pub mod monad {
    //! [Haskell's monad laws](https://wiki.haskell.org/Monad_laws)

    use crate::prelude::*;

    /// Tests that `M::consume(a) >> f == f(a)`.
    #[inline]
    pub fn left_identity<
        A: Clone,
        B,
        MA: Monad<A> + core::ops::Shr<&'static F, Output = MA::Monad<B>>,
        F: FnOnce(A) -> MA::Monad<B>,
    >(
        a: A,
        f: &'static F,
    ) -> bool
    where
        MA::Monad<B>: PartialEq,
    {
        (MA::consume(a.clone()) >> f) == f(a)
    }

    /// Tests that `m >> M::consume == m`.
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn right_identity<A, MA: Monad<A, Monad<A> = MA> + Clone + PartialEq>(ma: MA) -> bool {
        ma.clone().bind::<A, _>(consume) == ma
    }

    /// Tests that `m >> f >> g == m >> |a| { f(a) >> g }`.
    #[inline]
    #[allow(clippy::trait_duplication_in_bounds)]
    pub fn associativity<
        A,
        B,
        C,
        M: Monad<A, Monad<A> = M>
            + Clone
            + core::ops::Shr<&'static F, Output = M::Monad<B>>
            + core::ops::Shr<fn(A) -> M::Monad<C>, Output = M::Monad<C>>,
        F: FnOnce(A) -> M::Monad<B>,
        G: FnOnce(B) -> M::Monad<C>,
    >(
        m: M,
        f: &'static F,
        g: &'static G,
    ) -> bool
    where
        M::Monad<B>: core::ops::Shr<&'static G, Output = M::Monad<C>>,
        M::Monad<C>: PartialEq,
    {
        #![allow(clippy::as_conversions)]
        ((m.clone() >> f) >> g) == m.bind::<C, _>(move |a| (f(a) >> g))
    }

    // It's surprisingly difficult to write anything that does *not* pass the monad laws and still compiles
}

pub mod functor {
    //! [Haskell's functor laws](https://wiki.haskell.org/Functor#Functor_Laws)

    use crate::prelude::*;

    /// Tests that `fmap`ping the identity function is a no-op.
    #[inline]
    pub fn identity<A, FA: Functor<A, Functor<A> = FA> + Clone + PartialEq>(fa: FA) -> bool {
        fa.clone() == fa.fmap(core::convert::identity)
    }

    /// Tests that `fmap`ping two functions separately is equivalent to `fmap`ping their composition.
    #[inline]
    pub fn composition<A, B, C, FA: Functor<A> + Clone, G: FnOnce(A) -> B, F: FnOnce(B) -> C>(
        fa: FA,
        f: F,
        g: G,
    ) -> bool
    where
        FA::Functor<C>: PartialEq<<FA::Functor<B> as Functor<B>>::Functor<C>>,
    {
        fa.clone().fmap(|x| f(g(x))) == fa.fmap(g).fmap(f)
    }
}
