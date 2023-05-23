//! Applicative trait.

use crate::prelude::*;

/// Apply a function wrapped in a functor to an argument wrapped in a functor.
pub trait Applicative<A: Clone>: Functor<A> {
    /// Fucking pain in the ass redundancy. This has to be in this trait to avoid potential spooky action at a distance e.g. by redefining a separate Hkt later.
    type Applicative<B: Clone>: Applicative<B, Applicative<A> = Self>;
    /// Construct an Applicative from a value.
    fn consume(a: A) -> Self;
    /// Apply a function wrapped in a functor to an argument wrapped in a functor.
    fn tie<F: FnOnce(A) -> B + Clone, B: Clone>(
        self,
        af: Self::Applicative<F>,
    ) -> Self::Applicative<B>;
}

/// Construct an Applicative from a value.
#[inline(always)]
pub fn consume<Ap: Applicative<A, Applicative<A> = Ap>, A: Clone>(a: A) -> Ap {
    Ap::consume(a)
}

/// Apply a function wrapped in a functor to an argument wrapped in a functor.
#[inline(always)]
#[must_use]
pub fn tie<Ap: Applicative<A>, A: Clone, F: FnOnce(A) -> B + Clone, B: Clone>(
    aa: Ap,
    af: Ap::Applicative<F>,
) -> Ap::Applicative<B> {
    aa.tie(af)
}
