/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Functors.

/// Container over which we can map a function.
/// ```rust
/// # #[cfg(feature = "std")] {
/// use rsmonad::prelude::*;
/// let li = list![1, 2, 3, 4, 5];
/// assert_eq!(li % u8::is_power_of_two, list![true, true, false, true, false]);
/// # }
/// ```
pub trait Functor<A: Clone> {
    /// Fucking pain in the ass redundancy. This has to be in this trait to avoid potential spooky action at a distance e.g. by redefining a separate Hkt later.
    type Functor<B: Clone>: Functor<B, Functor<A> = Self>;
    /// Map a function over this functor.
    fn fmap<B: Clone, F: FnOnce(A) -> B + Clone>(self, f: F) -> Self::Functor<B>;
}

/// Map a function over a container.
#[inline(always)]
pub fn fmap<A: Clone, B: Clone, FA: Functor<A>, F: FnOnce(A) -> B + Clone>(
    f: F,
    fa: FA,
) -> FA::Functor<B> {
    fa.fmap(f)
}
