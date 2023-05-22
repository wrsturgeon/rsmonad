//! Sum monoid.

#![allow(clippy::missing_trait_methods)]

use crate::prelude::*;

use core::ops::Add;

/// Sum monoid.
/// ```rust
/// use rsmonad::prelude::*;
/// assert_eq!(
///     (list![1_u8, 2, 3, 4, 5] | Sum).unify(),
///     Sum(15)
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sum<A: Add<A, Output = A> + From<u8>>(pub A);

impl<A: Add<A, Output = A> + From<u8> + ::quickcheck::Arbitrary> ::quickcheck::Arbitrary
    for Sum<A>
{
    #[inline]
    fn arbitrary(g: &mut ::quickcheck::Gen) -> Self {
        Self(<A as ::quickcheck::Arbitrary>::arbitrary(g))
    }
}

impl<A: Add<A, Output = A> + From<u8>> Monoid for Sum<A> {
    fn unit() -> Self {
        Self(0_u8.into())
    }
    fn combine(self, other: Self) -> Self {
        #[allow(clippy::arithmetic_side_effects)]
        Self(self.0 + other.0)
    }
}

// TODO: WrapSum, SatSum, etc.
