/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

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
            v.push(f.clone()(s));
        }
        v
    }
}
test_functor!(Vec<u64>);

impl<A: Clone> Applicative<A> for Vec<A> {
    type Applicative<B: Clone> = Vec<B>;
    #[inline(always)]
    fn consume(a: A) -> Self {
        vec![a]
    }
    #[inline(always)]
    fn tie<F: FnOnce(A) -> B + Clone, B: Clone>(
        self,
        af: Self::Applicative<F>,
    ) -> Self::Applicative<B> {
        self.bind(move |a| af.bind(move |f| consume(f(a))))
    }
}
test_applicative!(Vec<u64>);

impl<A: Clone> Alternative<A> for Vec<A> {
    type Alternative<B: Clone> = Vec<B>;
    #[inline(always)]
    fn empty() -> Self {
        vec![]
    }
    #[inline(always)]
    fn either<F: FnOnce() -> Self>(mut self, make_other: F) -> Self {
        self.append(&mut make_other());
        self
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
test_monad!(Vec<u64>);

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
test_monoid!(Vec<u64>);
