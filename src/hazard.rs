//! `Hazard` monad.

use crate::prelude::*;

/// Currently `String` but may be set to `&str` in the future.
type ErrMsg = String;

monad! {
    /// Encodes the possibility of failure with a reason.
    /// # Use
    /// ```rust
    /// use rsmonad::prelude::*;
    /// fn successor(x: u8) -> Hazard<u8> {
    ///     x.checked_add(1).map_or_else(|| Failure("Overflow!".to_owned()), Success)
    /// }
    /// assert_eq!(
    ///     Success(3_u8) >> successor,
    ///     Success(4),
    /// );
    /// assert_eq!(
    ///     Failure("No value provided".to_owned()) >> successor,
    ///     Failure("No value provided".to_owned()),
    /// );
    /// assert_eq!(
    ///     Success(255_u8) >> successor,
    ///     Failure("Overflow!".to_owned()),
    /// );
    /// ```
    pub enum Hazard<A> {
        /// Error with a message. Invoking `>>` will immediately return this failure as well.
        Failure(ErrMsg),
        /// A value that hasn't failed (yet). Invoking `>>` on some function `f` will call `f` with that value as its argument.
        Success(A),
    }

    fn bind(self, f) {
        match self {
            Success(a) => f(a),
            Failure(s) => Failure(s),
        }
    }

    fn consume(a) {
        Success(a)
    }
}

impl<A> Default for Hazard<A> {
    #[inline]
    fn default() -> Self {
        Failure("Hazard<_> instantiated as a default but not changed".to_owned())
    }
}

/// Convenience (D.R.Y.).
#[cfg(feature = "nightly")]
type Residual = Hazard<core::convert::Infallible>;

#[cfg(feature = "nightly")]
impl<A> core::ops::Try for Hazard<A> {
    type Output = A;
    type Residual = Residual;
    #[inline]
    fn from_output(a: A) -> Self {
        consume(a)
    }
    #[inline]
    fn branch(self) -> core::ops::ControlFlow<Residual, A> {
        match self {
            Success(a) => core::ops::ControlFlow::Continue(a),
            Failure(s) => core::ops::ControlFlow::Break(Failure(s)),
        }
    }
}

#[cfg(feature = "nightly")]
impl<A> core::ops::FromResidual<Residual> for Hazard<A> {
    #[inline]
    #[track_caller]
    fn from_residual(r: Residual) -> Self {
        match r {
            Failure(s) => Failure(s),
            // SAFETY:
            // Type is literally uninstantiable. If we somehow hit this branch, there were much bigger problems upstream.
            Success(_) => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod nightly_tests {
    use super::*;

    fn should_short_circuit() -> Hazard<()> {
        Failure("Intentional short-circuit".to_owned())?;
        Success(())
    }

    #[test]
    fn hazard_question_mark() {
        match should_short_circuit() {
            Success(()) => panic!("Didn't exit early!"),
            Failure(s) => assert_eq!(s, "Intentional short-circuit"),
        }
    }
}
