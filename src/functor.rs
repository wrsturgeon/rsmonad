//! Functor automatically derived from Monad.
//! # TODO
//! Should be a *superclass* of Monad (i.e. `Monad<A>: Functor<A>`, not vice versa), but also automatically defined in terms of Monad if Monad is defined.
//! We could `QuickCheck` their equivalence or automatically define Functor in `monad!`, or we could just pretend that this is a subclass of Monad.
//! Let's go with the automatic definition for now, but we should also defined `functor!` in the future.

use same_as::SameAs;

/// Container over which we can map a function.
pub trait Functor<A>: SameAs<Self::Hkt<A>> {
    /// In this `impl`, `Self` is really `Self<A>`, but we want to be able to make `Self<B>`.
    type Hkt<B>;
    /// Map a function over this functor.
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::Hkt<B>;
}

/// Map a function over a container.
#[inline(always)]
pub fn fmap<A, B, T: Functor<A>, F: Fn(A) -> B>(f: F, t: T) -> T::Hkt<B> {
    t.fmap(f)
}
