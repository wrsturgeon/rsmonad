//! Implementations for types from outside this library that can nonetheless act like Haskell typeclasses, only without operator shorthand.

#![allow(clippy::mismatching_type_param_order, clippy::missing_trait_methods)]

use crate::prelude::*;

//////////////// Vec

impl<A: Clone> Functor<A> for Vec<A> {
    type Functor<B: Clone> = Vec<B>;
    #[inline(always)]
    fn fmap<B: Clone, F: FnOnce(A) -> B + Clone>(self, f: F) -> Self::Functor<B> {
        let mut v = vec![];
        v.reserve(self.len());
        for s in self {
            v.push(f.clone()(s))
        }
        v
    }
}

impl<A: Clone> Applicative<A> for Vec<A> {
    type Applicative<B: Clone> = Vec<B>;
    #[inline(always)]
    fn consume(a: A) -> Vec<A> {
        vec![a]
    }
    fn tie<B: Clone, C: Clone>(self, ab: Self::Applicative<B>) -> Self::Applicative<C>
    where
        A: FnOnce(B) -> C,
    {
        self.bind(move |f| ab.bind(move |b| consume(f(b))))
    }
}

impl<A: Clone> Monad<A> for Vec<A> {
    type Monad<B: Clone> = Vec<B>;
    #[inline(always)]
    fn bind<B: Clone, F: FnOnce(A) -> Self::Monad<B> + Clone>(self, f: F) -> Self::Monad<B> {
        let mut v = Vec::with_capacity(self.len());
        for a in self {
            v.append(&mut f.clone()(a));
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
