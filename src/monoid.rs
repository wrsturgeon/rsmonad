//! Monoids: anytime you can combine two elements of the same type to get a third, and there's an element that does nothing.
//! E.g. lists with concatenation (`[]` does nothing), numbers with addition and zero, or numbers with multiplication and one.

use crate::prelude::*;

/// Anytime you can combine two elements of the same type to get a third, and there's an element that does nothing.
/// E.g. lists with concatenation (`[]` does nothing), numbers with addition and zero, or numbers with multiplication and one.
pub trait Monoid {
    /// The identity element.
    #[must_use]
    fn unit() -> Self;
    /// Combine two values to produce a third. `unit` on either side should do nothing.
    #[must_use]
    fn combine(self, other: Self) -> Self;
    /// Fold a monoid onto itself in the way you'd think. Concretely, we by default use `unit` as the initial value and `combine` as the combinator.
    #[inline(always)]
    #[must_use]
    fn unify<F: Fold<Item = Self>>(f: F) -> Self
    where
        Self: Sized,
        F::IntoIter: DoubleEndedIterator,
    {
        f.foldr(combine, unit())
    }
}

/// The identity element of a monoid.
#[inline(always)]
#[must_use]
pub fn combine<M: Monoid>(a: M, b: M) -> M {
    M::combine(a, b)
}

/// Combine two values to produce a third. `unit` on either side should do nothing.
#[inline(always)]
#[must_use]
pub fn unit<M: Monoid>() -> M {
    M::unit()
}

/// Fold a monoid onto itself in the way you'd think. Concretely, we by default use `unit` as the initial value and `combine` as the combinator.
#[inline(always)]
#[must_use]
pub fn unify<M: Monoid, F: Fold<Item = M>>(f: F) -> M
where
    F::IntoIter: DoubleEndedIterator,
{
    M::unify(f)
}
