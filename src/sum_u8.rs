/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Sum monoid.

#![allow(clippy::missing_trait_methods)]

use crate::prelude::*;

/// Sum monoid.
/// ```rust
/// use rsmonad::prelude::*;
/// # #[cfg(feature = "std")]
/// assert_eq!(
///     (list![1, 2, 3, 4, 5] % SumU8).unify(),
///     SumU8(15)
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, QuickCheck)]
pub struct SumU8(pub u8);

monoid! {
    SumU8:

    fn unit() { Self(0) }

    fn combine(self, other) { Self(self.0.wrapping_add(other.0)) }
}

// TODO: WrapSum, SatSum, etc.
