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
    fn foldr<B, F: Fn(<Self as Fold>::Item, B) -> B>(self, f: F, b: B) -> B {
        let mut acc = b;
        for a in self.into_iter().rev() {
            acc = f(a, acc);
        }
        acc
    }
    /// Takes an initial value and uses `f` to combine it with each element in the list from right to left.
    #[inline]
    fn foldl<B, F: Fn(B, <Self as Fold>::Item) -> B>(self, f: F, b: B) -> B {
        let mut acc = b;
        for a in self {
            acc = f(acc, a);
        }
        acc
    }
    /// Folds a monoid. Unless overriden, uses the initial value `mempty` and the combinator `mconcat`.
    #[inline]
    fn mconcat(self) -> <Self as Fold>::Item
    where
        <Self as Fold>::Item: Monoid,
    {
        Monoid::mconcat(self)
    }
}
