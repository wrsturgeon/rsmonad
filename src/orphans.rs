//! Implementations for types from outside this library that can nonetheless act like Haskell typeclasses, only without operator shorthand.

#![allow(clippy::missing_trait_methods)]

use crate::prelude::*;

//////////////// Option

impl<A: Clone> Functor<A> for Option<A> {
    type Functor<B: Clone> = Option<B>;
    #[inline(always)]
    fn fmap<B: Clone, F: FnOnce(A) -> B>(self, f: F) -> Self::Functor<B> {
        self.map(f)
    }
}

impl<A: Clone> Applicative<A> for Option<A> {
    type Applicative<B: Clone> = Option<B>;
    #[inline(always)]
    fn consume(a: A) -> Option<A> {
        Some(a)
    }
    #[inline(always)]
    fn tie<B: Clone, C: Clone>(self, ab: Self::Applicative<B>) -> Self::Applicative<C>
    where
        A: FnOnce(B) -> C,
    {
        match (self, ab) {
            (Some(f), Some(b)) => Some(f(b)),
            _ => None,
        }
    }
}

impl<A: Clone> Monad<A> for Option<A> {
    type Monad<B: Clone> = Option<B>;
    #[inline(always)]
    fn bind<B: Clone, F: FnOnce(A) -> Self::Monad<B> + Clone>(self, f: F) -> Self::Monad<B> {
        self.and_then(f)
    }
}

impl<A> Fold for Option<A> {
    type Item = A;
}

//////////////// [A; N]

impl<A, const N: usize> Fold for [A; N] {
    type Item = A;
}
