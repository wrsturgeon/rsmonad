//! Implementations for types from outside this library that can nonetheless act like Haskell typeclasses, only without operator shorthand.

#![allow(clippy::missing_trait_methods)]

use crate::prelude::*;

//////////////// Option

impl<A> Functor<A> for Option<A> {
    type Functor<B> = Option<B>;
    #[inline(always)]
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::Functor<B> {
        self.map(f)
    }
}

impl<A> Monad<A> for Option<A> {
    type Monad<B> = Option<B>;
    #[inline(always)]
    fn bind<B, F: Fn(A) -> Self::Monad<B>>(self, f: F) -> Self::Monad<B> {
        self.and_then(f)
    }
    #[inline(always)]
    fn consume(a: A) -> Self {
        Some(a)
    }
}

impl<A> Fold for Option<A> {
    type Item = A;
}

//////////////// [A; N]

impl<A, const N: usize> Fold for [A; N] {
    type Item = A;
}
