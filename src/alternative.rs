/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! The Alternative typeclass.

use crate::prelude::*;

/// Alternative typeclass for picking a successful result out of two fallible operations.
pub trait Alternative<A: Clone>: Applicative<A> {
    /// Fucking pain in the ass redundancy. This has to be in this trait to avoid potential spooky action at a distance e.g. by redefining a separate Hkt later.
    type Alternative<B: Clone>: Alternative<B, Alternative<A> = Self>;
    /// Value representing failure/emptiness/nothingness.
    #[must_use]
    fn empty() -> Self;
    /// Return a successful result if we have one.
    #[must_use]
    fn either<F: FnOnce() -> Self>(self, make_other: F) -> Self;
    /// Return empty or a trivial value based on a predicate.
    #[inline(always)]
    #[must_use]
    fn guard(b: bool) -> Self::Alternative<()>
    where
        Self::Alternative<()>: Applicative<(), Applicative<()> = Self::Alternative<()>>,
    {
        if b {
            consume::<Self::Alternative<()>, ()>(())
        } else {
            empty()
        }
    }
}

/// Value representing failure/emptiness/nothingness.
#[inline(always)]
#[must_use]
pub fn empty<AA: Alternative<A>, A: Clone>() -> AA {
    AA::empty()
}

/// Return a successful result if we have one.
#[inline(always)]
#[must_use]
pub fn either<AA: Alternative<A>, A: Clone, F: FnOnce() -> AA>(f1: F, f2: F) -> AA {
    f1().either(f2)
}

/// Return empty or a trivial value based on a predicate.
#[inline(always)]
#[must_use]
pub fn guard<A: Alternative<(), Applicative<()> = A>>(b: bool) -> A {
    if b {
        consume(())
    } else {
        empty()
    }
}
