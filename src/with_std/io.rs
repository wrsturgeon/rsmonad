/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! `Io` monad.

use crate::prelude::*;

/// Encodes the possibility of failure.
/// # Use
/// ```rust
/// use rsmonad::prelude::*;
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
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, QuickCheck)]
pub struct Io<A>(A);
monad! {
    Io<A>:

    fn consume(b) {
        Io(b)
    }

    fn bind(self, f) {
        f(self.0)
    }
}

/// Reads a single line from `stdin`.
#[must_use]
#[inline(always)]
pub fn get_line_stdin() -> Hazard<Io<String>, String> {
    let mut s = String::new();
    match std::io::stdin().read_line(&mut s) {
        Ok(_) => Success(consume(s)),
        Err(e) => Failure(e.to_string()),
    }
}

/// Prints without a newline.
#[inline(always)]
pub fn put<S: core::fmt::Display>(s: S) -> Io<()> {
    #![allow(clippy::print_stdout)]
    consume(print!("{s}"))
}

/// Prints with a newline.
#[inline(always)]
pub fn put_line<S: core::fmt::Display>(s: S) -> Io<()> {
    #![allow(clippy::print_stdout)]
    consume(println!("{s}"))
}
