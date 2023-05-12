//! `List` monad.

extern crate alloc;

use crate::Monad;
use alloc::vec::Vec;

/// Encodes the possibility of failure.
/// # Use
/// Similar to Rust's `Option::map`:
/// ```rust
/// use rsmonad::*;
/// // from the wonderful Haskell docs: https://en.wikibooks.org/wiki/Haskell/Understanding_monads/List
/// fn bunny(s: &str) -> List<&str> {
///     List(vec![s, s, s])
/// }
/// assert_eq!(
///     List::consume("bunny") >> bunny,
///     List(vec!["bunny", "bunny", "bunny"]),
/// );
/// assert_eq!(
///     List::consume("bunny") >> bunny >> bunny,
///     List(vec!["bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny"]),
/// );
/// ```
#[allow(clippy::exhaustive_structs)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct List<A>(pub Vec<A>);

impl<A> Monad<A> for List<A> {
    type Constructor<B> = List<B>;
    #[inline(always)]
    fn bind<B, F: Fn(A) -> Self::Constructor<B>>(self, f: F) -> Self::Constructor<B> {
        let mut v = Vec::new();
        for a in self.0 {
            v.append(&mut f(a).0);
        }
        List(v)
    }
    #[inline(always)]
    fn consume(a: A) -> Self {
        Self(alloc::vec![a])
    }
}

impl<A, B, F: Fn(A) -> List<B>> core::ops::Shr<F> for List<A> {
    type Output = List<B>;
    #[inline(always)]
    fn shr(self, rhs: F) -> Self::Output {
        self.bind(rhs)
    }
}

impl<A, B> core::ops::BitAnd<List<B>> for List<A> {
    type Output = List<B>;
    #[inline(always)]
    fn bitand(self, rhs: List<B>) -> Self::Output {
        rhs
    }
}
