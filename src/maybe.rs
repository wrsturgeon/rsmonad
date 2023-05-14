//! `Maybe` monad.

use crate::monad;

monad! {
    /// Encodes the possibility of failure.
    /// # Use
    /// ```rust
    /// use rsmonad::*;
    /// fn successor(x: u8) -> Maybe<u8> {
    ///     x.checked_add(1).map_or(Nothing, Just)
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
    pub enum Maybe {
        /// No value. Invoking `>>` will immediately return `Nothing` as well.
        #[default]
        Nothing,
        /// Some value. Invoking `>>` on some function `f` will call `f` with that value as its argument.
        Just(A),
    }

    fn bind(self, f) {
        match self {
            Just(a) => f(a),
            Nothing => Nothing,
        }
    }

    fn consume(a) -> Self {
        Just(a)
    }
}
