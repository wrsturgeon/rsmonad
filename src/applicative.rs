//! Applicative trait.

use crate::prelude::*;

/// Apply a function wrapped in a functor to an argument wrapped in a functor.
pub trait Applicative<A: Clone>: Functor<A> {
    /// Fucking pain in the ass redundancy. This has to be in this trait to avoid potential spooky action at a distance e.g. by redefining a separate Hkt later.
    type Applicative<B: Clone>: Applicative<B, Applicative<A> = Self>;
    /// Construct an Applicative from a value.
    fn consume(a: A) -> Self;
    /// Apply a function wrapped in a functor to an argument wrapped in a functor.
    fn tie<B: Clone, C: Clone>(self, ab: Self::Applicative<B>) -> Self::Applicative<C>
    where
        A: FnOnce(B) -> C;
}

/// Construct an Applicative from a value.
#[inline(always)]
pub fn consume<Ap: Applicative<A, Applicative<A> = Ap>, A: Clone>(a: A) -> Ap {
    Ap::consume(a)
}

/// Apply a function wrapped in a functor to an argument wrapped in a functor.
#[inline(always)]
#[must_use]
pub fn tie<Ap: Applicative<A>, A: FnOnce(B) -> C + Clone, B: Clone, C: Clone>(
    aa: Ap,
    ab: Ap::Applicative<B>,
) -> Ap::Applicative<C> {
    aa.tie(ab)
}
