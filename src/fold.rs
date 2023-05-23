//! Foldable types.

use crate::prelude::*;

/// Implement `foldl` and `foldr`, which take an initial value and combine it with each element of a list in  order.
/// `foldl` works from left to right and `foldr` the reverse.
pub trait Fold: Sized + IntoIterator<Item = <Self as Fold>::Item>
where
    Self::IntoIter: DoubleEndedIterator,
{
    /// Type of each element in this collection.
    type Item;
    /// Takes an initial value and uses `f` to combine it with each element in the list from right to left.
    #[inline]
    fn foldr<B, F: FnOnce(<Self as Fold>::Item, B) -> B + Clone>(self, f: F, b: B) -> B {
        let mut acc = b;
        for a in self.into_iter().rev() {
            acc = f.clone()(a, acc);
        }
        acc
    }
    /// Takes an initial value and uses `f` to combine it with each element in the list from right to left.
    #[inline]
    fn foldl<B, F: FnOnce(B, <Self as Fold>::Item) -> B + Clone>(self, f: F, b: B) -> B {
        let mut acc = b;
        for a in self {
            acc = f.clone()(acc, a);
        }
        acc
    }
    /// Folds a monoid. Unless overriden, uses the initial value `unit` and the combinator `combine`.
    #[inline]
    fn unify(self) -> <Self as Fold>::Item
    where
        <Self as Fold>::Item: Monoid,
    {
        Monoid::unify(self)
    }
}
