//! Product monoid.

#![allow(clippy::missing_trait_methods)]

use crate::prelude::*;

/// Sum monoid.
/// ```rust
/// use rsmonad::prelude::*;
/// assert_eq!(
///     (list![1, 2, 3, 4, 5] | Product).unify(),
///     Product(120)
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, QuickCheck)]
pub struct Product(pub usize);

monoid! {
    Product:

    fn unit() { Self(1) }

    fn combine(self, other) { Self(self.0.wrapping_mul(other.0)) }
}

// TODO: WrapSum, SatSum, etc.
