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
    #[inline(always)]
    fn consume(a: A) -> Self {
        vec![a]
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
