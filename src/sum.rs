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
/// assert_eq!(
///     (list![1, 2, 3, 4, 5] | Sum).unify(),
///     Sum(15)
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, QuickCheck)]
pub struct Sum(pub usize);

monoid! {
    Sum:

    fn unit() { Self(0) }

    fn combine(self, other) { Self(self.0.wrapping_add(other.0)) }
}

// TODO: WrapSum, SatSum, etc.
