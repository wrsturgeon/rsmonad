/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Trait definition for a monad safe to unwind during a `panic`.

use core::panic::UnwindSafe;
use same_as::SameAs;

/// Identical to Monad but with an inductive guarantee of panic-unwind safety.
pub trait UnwindMonad<A: UnwindSafe>: SameAs<Self::Constructor<A>> {
    // TODO: once for<T> lands, use it to restrict `Monad` to `for<F: FnOnce(A) -> B> core::ops::Shr<F>`
    /// In this `impl`, `Self` is really `Self<A>`, but we want to be able to make `Self<B>`.
    type Constructor<B: UnwindSafe>: UnwindMonad<B>;
    /// Mutate internal state with some function.
    fn bind<B: UnwindSafe, F: FnOnce(A) -> Self::Constructor<B> + UnwindSafe>(
        self,
        f: F,
    ) -> Self::Constructor<B>;
    /// Construct a monad from a value.
    fn consume(a: A) -> Self;
}
