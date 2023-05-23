//! The Alternative typeclass.

use crate::prelude::*;

/// Alternative typeclass for picking a successful result out of two fallible operations.
pub trait Alternative<A: Clone>: Applicative<A> {
    /// Value representing failure/emptiness/nothingness.
    fn empty() -> Self;
    /// Return a successful result if we have one.
    #[must_use]
    fn either<F: FnOnce() -> Self>(self, make_other: F) -> Self;
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
