//! Monoids: anytime you can combine two elements of the same type to get a third, and there's an element that does nothing.
//! E.g. lists with concatenation (`[]` does nothing), numbers with addition and zero, or numbers with multiplication and one.

use crate::prelude::*;

/// Anytime you can combine two elements of the same type to get a third, and there's an element that does nothing.
/// E.g. lists with concatenation (`[]` does nothing), numbers with addition and zero, or numbers with multiplication and one.
pub trait Monoid {
    /// The identity element.
    #[must_use]
    fn mempty() -> Self;
    /// Combine two values to produce a third. `mempty` on either side should do nothing.
    #[must_use]
    fn mappend(self, other: Self) -> Self;
    /// Fold a monoid onto itself in the way you'd think. Concretely, we by default use `mempty` as the initial value and `mappend` as the combinator.
    #[inline(always)]
    #[must_use]
    fn mconcat<F: Fold<Item = Self>>(f: F) -> Self
    where
        Self: Sized,
        F::IntoIter: DoubleEndedIterator,
    {
        f.foldr(mappend, mempty())
    }
}

/// The identity element of a monoid.
#[inline(always)]
#[must_use]
pub fn mappend<M: Monoid>(a: M, b: M) -> M {
    M::mappend(a, b)
}

/// Combine two values to produce a third. `mempty` on either side should do nothing.
#[inline(always)]
#[must_use]
pub fn mempty<M: Monoid>() -> M {
    M::mempty()
}

/// Fold a monoid onto itself in the way you'd think. Concretely, we by default use `mempty` as the initial value and `mappend` as the combinator.
#[inline(always)]
#[must_use]
pub fn mconcat<M: Monoid, F: Fold<Item = M>>(f: F) -> M
where
    F::IntoIter: DoubleEndedIterator,
{
    M::mconcat(f)
}
