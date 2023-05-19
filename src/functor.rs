//! Functors.

use same_as::SameAs;

/// Container over which we can map a function.
/// ```rust
/// use rsmonad::prelude::*;
/// let li = List(vec![1, 2, 3, 4, 5]);
/// assert_eq!(li | u8::is_power_of_two, List(vec![true, true, false, true, false]));
/// ```
pub trait Functor<A>: SameAs<Self::Functor<A>> {
    /// Fucking pain in the ass redundancy. This has to be in this trait to avoid potential spooky action at a distance e.g. by redefining a separate Hkt later.
    type Functor<B>: Functor<B>;
    /// Map a function over this functor.
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::Functor<B>;
}

/// Map a function over a container.
#[inline(always)]
pub fn fmap<A, B, FA: Functor<A>, F: Fn(A) -> B>(f: F, fa: FA) -> FA::Functor<B> {
    fa.fmap(f)
}
