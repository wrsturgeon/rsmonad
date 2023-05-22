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
