//! Implementations for types from outside this library that can nonetheless act like Haskell typeclasses, only without operator shorthand.

#![allow(clippy::mismatching_type_param_order, clippy::missing_trait_methods)]

use crate::prelude::*;

//////////////// Vec

impl<A> Functor<A> for Vec<A> {
    type Functor<B> = Vec<B>;
    #[inline(always)]
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::Functor<B> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Applicative<A> for Vec<A> {
    type Applicative<B> = Vec<B>;
    #[inline(always)]
    fn consume(a: A) -> Self {
        vec![a]
    }
    fn tie<B, C>(self, ab: Self::Applicative<B>) -> Self::Applicative<C>
    where
        A: Fn(B) -> C,
    {
        self.bind(move |f| ab.bind(move |b| consume(f(b))))
    }
}

impl<A> Monad<A> for Vec<A> {
    type Monad<B> = Vec<B>;
    #[inline(always)]
    fn bind<B, F: Fn(A) -> Self::Monad<B>>(self, f: F) -> Self::Monad<B> {
        let mut v = Vec::with_capacity(self.len());
        for a in self {
            v.append(&mut f(a));
        }
        v
    }
}

impl<A> Fold for Vec<A> {
    type Item = A;
}

impl<A> Monoid for Vec<A> {
    #[inline(always)]
    fn unit() -> Self {
        vec![]
    }
    #[inline(always)]
    fn combine(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}
