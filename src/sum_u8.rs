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
