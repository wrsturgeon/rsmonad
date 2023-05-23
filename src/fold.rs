//! Foldable types.

use crate::prelude::*;

/// Implement `foldl` and `foldr`, which take an initial value and combine it with each element of a list in order.
/// `foldl` works from left to right and `foldr` the reverse.
pub trait Fold: Sized + IntoIterator<Item = <Self as Fold>::Item>
where
    Self::IntoIter: DoubleEndedIterator,
{
    /// Type of each element in this collection.
    type Item;
    /// Takes an initial value and uses `f` to combine it with each element in the list from right to left.
    #[inline(always)]
    fn foldr<B, F: FnOnce(<Self as Fold>::Item, B) -> B + Clone>(self, f: F, b: B) -> B {
        let mut acc = b;
        for a in self {
            acc = f.clone()(a, acc);
        }
        acc
    }
    /// Takes an initial value and uses `f` to combine it with each element in the list from right to left.
    #[inline(always)]
    fn foldl<B, F: FnOnce(B, <Self as Fold>::Item) -> B + Clone>(self, f: F, b: B) -> B {
        let mut acc = b;
        for a in self.into_iter().rev() {
            acc = f.clone()(acc, a);
        }
        acc
    }
    /// Folds a monoid. Unless overriden, uses the initial value `unit` and the combinator `combine`.
    #[inline(always)]
    fn unify(self) -> <Self as Fold>::Item
    where
        <Self as Fold>::Item: Monoid,
    {
        Monoid::unify(self)
    }
    /// Folds a collection of lazy Alternatives. Starts with `empty` and combines with `either`.
    #[inline(always)]
    fn asum<AA: Alternative<A>, A: Clone>(self) -> AA
    where
        <Self as Fold>::Item: FnOnce() -> AA,
    {
        self.foldr(move |f, acc| acc.either(f), empty())
    }
    /// Folds a collection of Alternatives. Starts with `empty` and combines with `either`.
    #[inline(always)]
    fn eager_asum<A: Clone>(self) -> <Self as Fold>::Item
    where
        <Self as Fold>::Item: Alternative<A>,
    {
        self.foldr(move |f, acc| acc.either(|| f), empty())
    }
}

#[cfg(feature = "std")]
#[test]
fn test_list_lazy_asum() {
    assert_eq!(
        list![|| list![1_u8, 2, 3], || list![4], || list![5, 6]].asum(),
        list![1, 2, 3, 4, 5, 6]
    );
}
