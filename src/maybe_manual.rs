//! `Maybe` monad.

use crate::Monad;

/// Encodes the possibility of failure.
/// # Use
/// Similar to Rust's `Option::map`:
/// ```rust
/// use rsmonad::*;
/// fn successor(x: u8) -> Maybe<u8> {
///     x.checked_add(1).map_or(Nothing, Just)
/// }
/// assert_eq!(
///     Just(3) >> successor,
///     Just(4),
/// );
/// assert_eq!(
///     Nothing >> successor,
///     Nothing,
/// );
/// assert_eq!(
///     Just(255) >> successor,
///     Nothing,
/// );
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
    fn bind<B, F: Fn(A) -> Self::Constructor<B>>(self, f: F) -> Self::Constructor<B> {
        if let Just(x) = self {
            f(x)
        } else {
            Nothing
        }
    }
    #[inline(always)]
    fn consume(a: A) -> Self {
        Just(a)
    }
}

impl<A, B, F: Fn(A) -> Maybe<B>> core::ops::Shr<F> for Maybe<A> {
    type Output = Maybe<B>;
    #[inline(always)]
    fn shr(self, rhs: F) -> Self::Output {
        self.bind(rhs)
    }
}

impl<A, B> core::ops::BitAnd<Maybe<B>> for Maybe<A> {
    type Output = Maybe<B>;
    #[inline(always)]
    fn bitand(self, rhs: Maybe<B>) -> Self::Output {
        rhs
    }
}
