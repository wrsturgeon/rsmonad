//! `Maybe` monad.

use crate::prelude::*;

/// Encodes the possibility of failure.
/// # Use
/// ```rust
/// use rsmonad::prelude::*;
/// fn successor(x: u8) -> Maybe<u8> {
///     x.checked_add(1).into()
/// }
/// assert_eq!(
///     Just(3_u8) >> successor,
///     Just(4),
/// );
/// assert_eq!(
///     Nothing >> successor,
///     Nothing,
/// );
/// assert_eq!(
///     Just(255_u8) >> successor,
///     Nothing,
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, QuickCheck)]
pub enum Maybe<A> {
    /// No value. Invoking `>>` will immediately return `Nothing` as well.
    #[default]
    Nothing,
    /// Some value. Invoking `>>` on some function `f` will call `f` with that value as its argument.
    Just(A),
}
pub use Maybe::{Just, Nothing};

monad! {
    Maybe<A>:

    fn consume(a) {
        Just(a)
    }

    fn bind(self, f) {
        match self {
            Just(a) => f(a),
            Nothing => Nothing,
        }
    }
}

impl<A> From<Maybe<A>> for Option<A> {
    #[inline(always)]
    fn from(value: Maybe<A>) -> Self {
        match value {
            Just(a) => Some(a),
            Nothing => None,
        }
    }
}

impl<A> From<Option<A>> for Maybe<A> {
    #[inline(always)]
    fn from(value: Option<A>) -> Self {
        value.map_or(Nothing, Just)
    }
}

/// Convenience (D.R.Y.).
#[cfg(feature = "nightly")]
type Residual = Maybe<core::convert::Infallible>;

#[cfg(feature = "nightly")]
impl<A: Clone> core::ops::Try for Maybe<A> {
    type Output = A;
    type Residual = Residual;
    #[inline]
    fn from_output(a: A) -> Self {
        consume(a)
    }
    #[inline]
    fn branch(self) -> core::ops::ControlFlow<Residual, A> {
        match self {
            Just(a) => core::ops::ControlFlow::Continue(a),
            Nothing => core::ops::ControlFlow::Break(Nothing),
        }
    }
}

#[cfg(feature = "nightly")]
impl<A: Clone> core::ops::FromResidual<Residual> for Maybe<A> {
    #[inline]
    #[track_caller]
    fn from_residual(r: Residual) -> Self {
        match r {
            Nothing => Nothing,
            // SAFETY:
            // Type is literally uninstantiable. If we somehow hit this branch, there were much bigger problems upstream.
            Just(_) => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod nightly_tests {
    use super::*;

    fn should_short_circuit() -> Maybe<()> {
        Nothing?;
        Just(())
    }

    #[test]
    fn hazard_question_mark() {
        match should_short_circuit() {
            Just(()) => panic!("Didn't exit early!"),
            Nothing => (),
        }
    }
}
