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
test_functor!(Option<A>);

impl<A: Clone> Applicative<A> for Option<A> {
    type Applicative<B: Clone> = Option<B>;
    #[inline(always)]
    fn consume(a: A) -> Self {
        Some(a)
    }
    #[inline(always)]
    fn tie<F: FnOnce(A) -> B + Clone, B: Clone>(
        self,
        af: Self::Applicative<F>,
    ) -> Self::Applicative<B> {
        self.bind(move |a| af.bind(move |f| consume(f(a))))
    }
}
test_applicative!(Option<A>);

impl<A: Clone> Monad<A> for Option<A> {
    type Monad<B: Clone> = Option<B>;
    #[inline(always)]
    fn bind<B: Clone, F: FnOnce(A) -> Self::Monad<B> + Clone>(self, f: F) -> Self::Monad<B> {
        self.and_then(f)
    }
}
test_monad!(Option<A>);

impl<A> Fold for Option<A> {
    type Item = A;
}

//////////////// [A; N]

impl<A, const N: usize> Fold for [A; N] {
    type Item = A;
}
