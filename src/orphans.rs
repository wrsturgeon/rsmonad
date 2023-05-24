/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

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
test_functor!(Option<u64>);

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
test_applicative!(Option<u64>);

impl<A: Clone> Alternative<A> for Option<A> {
    type Alternative<B: Clone> = Option<B>;
    #[inline(always)]
    fn empty() -> Self {
        None
    }
    #[inline(always)]
    fn either<F: FnOnce() -> Self>(self, make_other: F) -> Self {
        if matches!(self, Some(_)) {
            self
        } else {
            make_other()
        }
    }
}
test_alternative!(Option<u64>);

impl<A: Clone> Monad<A> for Option<A> {
    type Monad<B: Clone> = Option<B>;
    #[inline(always)]
    fn bind<B: Clone, F: FnOnce(A) -> Self::Monad<B> + Clone>(self, f: F) -> Self::Monad<B> {
        self.and_then(f)
    }
}
test_monad!(Option<u64>);

impl<A> Fold for Option<A> {
    type Item = A;
}

//////////////// [A; N]

impl<A, const N: usize> Fold for [A; N] {
    type Item = A;
}

//////////////// bool

impl Monoid for bool {
    #[inline(always)]
    fn unit() -> Self {
        false
    }
    #[inline(always)]
    fn combine(self, other: Self) -> Self {
        self || other
    }
}

//////////////// Poll

impl<A: Clone> Functor<A> for core::task::Poll<A> {
    type Functor<B: Clone> = core::task::Poll<B>;
    #[inline(always)]
    fn fmap<B: Clone, F: FnOnce(A) -> B + Clone>(self, f: F) -> Self::Functor<B> {
        if let Self::Ready(a) = self {
            core::task::Poll::Ready(f(a))
        } else {
            core::task::Poll::Pending
        }
    }
}

impl<A: Clone> Applicative<A> for core::task::Poll<A> {
    type Applicative<B: Clone> = core::task::Poll<B>;
    #[inline(always)]
    fn consume(a: A) -> Self {
        Self::Ready(a)
    }
    #[inline(always)]
    fn tie<F: FnOnce(A) -> B + Clone, B: Clone>(
        self,
        af: Self::Applicative<F>,
    ) -> Self::Applicative<B> {
        self.bind(move |a| af.bind(move |f: F| consume(f(a))))
    }
}

impl<A: Clone> Alternative<A> for core::task::Poll<A> {
    type Alternative<B: Clone> = core::task::Poll<B>;
    #[inline(always)]
    fn empty() -> Self {
        Self::Pending
    }
    #[inline(always)]
    fn either<F: FnOnce() -> Self>(self, make_other: F) -> Self {
        if self.is_ready() {
            self
        } else {
            make_other()
        }
    }
}

impl<A: Clone> Monad<A> for core::task::Poll<A> {
    type Monad<B: Clone> = core::task::Poll<B>;
    #[inline(always)]
    fn bind<B: Clone, F: FnOnce(A) -> Self::Monad<B> + Clone>(self, f: F) -> Self::Monad<B> {
        if let Self::Ready(a) = self {
            f(a)
        } else {
            core::task::Poll::Pending
        }
    }
}
