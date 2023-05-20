//! `List` monad.

extern crate alloc;
use crate::prelude::*;
use alloc::vec::Vec;

/// Encodes nondeterminism.
/// # Use
/// ```rust
/// use rsmonad::prelude::*;
/// let li = list![1, 2, 3, 4, 5];
/// fn and_ten(x: u8) -> List<u8> { list![x, 10 * x] }
/// assert_eq!(li >> and_ten, list![1, 10, 2, 20, 3, 30, 4, 40, 5, 50]);
/// ```
/// ```rust
/// use rsmonad::prelude::*;
/// // from the wonderful Haskell docs: https://en.wikibooks.org/wiki/Haskell/Understanding_monads/List
/// fn bunny(s: &str) -> List<&str> {
///     list![s, s, s]
/// }
/// assert_eq!(
///     list!["bunny"] >> bunny,
///     list!["bunny", "bunny", "bunny"],
/// );
/// assert_eq!(
///     list!["bunny"] >> bunny >> bunny,
///     list!["bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny"],
/// );
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, QuickCheck)]
pub struct List<A>(Vec<A>);

/// Initialize an `rsmonad` List.
/// ```rust
/// use rsmonad::prelude::*;
/// let li = list![1, 2, 3, 4, 5];
/// fn and_ten(x: u8) -> List<u8> { list![x, 10 * x] }
/// assert_eq!(li >> and_ten, list![1, 10, 2, 20, 3, 30, 4, 40, 5, 50]);
/// ```
#[macro_export]
macro_rules! list {
    ($($tt:tt)*) => {
        <List<_> as From<Vec<_>>>::from(vec![$($tt)*])
    };
}
pub use list;

impl<A> From<Vec<A>> for List<A> {
    #[inline(always)]
    fn from(value: Vec<A>) -> Self {
        Self(value)
    }
}

impl<T> From<List<T>> for Vec<T> {
    #[inline(always)]
    fn from(value: List<T>) -> Self {
        value.0
    }
}

monad! {
    List<A>:

    fn bind(self, f) {
        let mut v = Vec::new();
        for a in self.0 {
            v.append(&mut f(a).0);
        }
        List(v)
    }

    fn consume(a) {
        Self(alloc::vec![a])
    }
}
