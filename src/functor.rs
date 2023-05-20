//! Functors.

/// Container over which we can map a function.
/// ```rust
/// use rsmonad::prelude::*;
/// let li = list![1, 2, 3, 4, 5];
/// assert_eq!(li | u8::is_power_of_two, list![true, true, false, true, false]);
/// ```
pub trait Functor<A> {
    /// Fucking pain in the ass redundancy. This has to be in this trait to avoid potential spooky action at a distance e.g. by redefining a separate Hkt later.
    type Functor<B>: Functor<B, Functor<A> = Self>;
    /// Map a function over this functor.
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::Functor<B>;
}

/// Map a function over a container.
#[inline(always)]
pub fn fmap<A, B, FA: Functor<A>, F: Fn(A) -> B>(f: F, fa: FA) -> FA::Functor<B> {
    fa.fmap(f)
}
