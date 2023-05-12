//! `Maybe` monad.

use crate::Monad;

/// Encodes the possibility of failure.
/// # Use
/// Similar to Rust's `Option::map`:
/// ```rust
/// use rsmonad::*;
/// fn successor(x: u8) -> Option<u8> { x.checked_add(1) }
/// assert_eq!(Just(4), Just(3) >> successor >> Option::unwrap);
/// assert_eq!(Nothing, Nothing >> successor >> Option::unwrap);
/// ```
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Maybe<A> {
    /// No value. Invoking `>>` will immediately return `Nothing` as well.
    #[default]
    Nothing,
    /// Some value. Invoking `>>` on some function `f` will call `f` with that value as its argument.
    Just(A),
}

pub use Maybe::{Just, Nothing};

impl<A> Monad<A> for Maybe<A> {
    type Constructor<B> = Maybe<B>;
    #[inline(always)]
    fn bind<B, F: Fn(A) -> B>(self, f: F) -> Self::Constructor<B> {
        if let Just(x) = self {
            Just(f(x))
        } else {
            Nothing
        }
    }
}

impl<A, B, F: Fn(A) -> B> core::ops::Shr<F> for Maybe<A> {
    type Output = <Self as Monad<A>>::Constructor<B>;
    #[inline(always)]
    fn shr(self, rhs: F) -> Self::Output {
        self.bind(rhs)
    }
}
