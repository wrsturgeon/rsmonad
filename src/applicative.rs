//! Applicative trait.

use crate::prelude::*;

/// Apply a function wrapped in a functor to an argument wrapped in a functor.
pub trait Applicative<A>: Functor<A> {
    /// Fucking pain in the ass redundancy. This has to be in this trait to avoid potential spooky action at a distance e.g. by redefining a separate Hkt later.
    type Applicative<B>: Applicative<B, Applicative<A> = Self>;
    /// Construct an Applicative from a value.
    fn consume(a: A) -> Self;
    /// Apply a function wrapped in a functor to an argument wrapped in a functor.
    fn tie<B, C>(self, ab: Self::Applicative<B>) -> Self::Applicative<C>
    where
        A: Fn(B) -> C;
}

/// Construct an Applicative from a value.
#[inline(always)]
pub fn consume<A, Ap: Applicative<A>>(a: A) -> Ap {
    Ap::consume(a)
}

/// Apply a function wrapped in a functor to an argument wrapped in a functor.
#[inline(always)]
#[must_use]
pub fn tie<Ap: Applicative<A>, A: Fn(B) -> C, B, C>(
    aa: Ap,
    ab: Ap::Applicative<B>,
) -> Ap::Applicative<C> {
    aa.tie(ab)
}
