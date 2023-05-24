/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Product monoid.

#![allow(clippy::missing_trait_methods)]

use crate::prelude::*;

/// Sum monoid.
/// ```rust
/// use rsmonad::prelude::*;
/// # #[cfg(feature = "std")]
/// assert_eq!(
///     (list![1, 2, 3, 4, 5] % ProductU8).unify(),
///     ProductU8(120)
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, QuickCheck)]
pub struct ProductU8(pub u8);

monoid! {
    ProductU8:

    fn unit() { Self(1) }

    fn combine(self, other) { Self(self.0.wrapping_mul(other.0)) }
}

// TODO: WrapSum, SatSum, etc.
