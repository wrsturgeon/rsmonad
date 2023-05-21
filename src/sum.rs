//! Sum monoid.

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, QuickCheck)]
struct Sum<A: core::ops::Add + From<u8>>(A);

monad! {
    Sum<A: core::ops::Add + From<u8>>:

    fn bind(self, f) {
        f(self.0)
    }

    fn consume(a) {
        Self(a)
    }
}

monoid! {
    Sum<A: core::ops::Add + From<u8>>:

    fn unit() { consume(0_u8.into()) }

    fn combine(self, other) { consume(self.0 + other.0) }
}
