//! `BlastDoor` monad.

use crate::prelude::*;
use core::panic::UnwindSafe;

/// Encodes the possibility of panicking.
/// # Use
/// ```rust
/// use rsmonad::prelude::*;
/// fn afraid_of_circles(x: u8) -> BlastDoor<()> {
///     if x == 0 { panic!("aaaaaa!"); }
///     Phew(())
/// }
/// assert_eq!(
///     Phew(42) >> afraid_of_circles,
///     Phew(())
/// );
/// assert_eq!(
///     Phew(0) >> afraid_of_circles,
///     Kaboom,
/// );
/// ```
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, QuickCheck)]
pub enum BlastDoor<A: UnwindSafe> {
    /// Panicked: no value. Invoking `>>` will immediately return `Kaboom` as well.
    #[default]
    Kaboom,
    /// Some value. Invoking `>>` on some function `f` will call `f` with that value as its argument.
    Phew(A),
}

pub use BlastDoor::{Kaboom, Phew};

impl<A: UnwindSafe> UnwindMonad<A> for BlastDoor<A> {
    type Constructor<B: UnwindSafe> = BlastDoor<B>;
    #[inline(always)]
    fn bind<B: UnwindSafe, F: FnOnce(A) -> BlastDoor<B> + UnwindSafe>(
        self,
        f: F,
    ) -> Self::Constructor<B> {
        if let Phew(x) = self {
            std::panic::catch_unwind(|| f(x)).map_or(Kaboom, core::convert::identity)
        } else {
            Kaboom
        }
    }
    #[inline(always)]
    fn consume(a: A) -> Self {
        Phew(a)
    }
}

impl<A: UnwindSafe, B: UnwindSafe, F: FnOnce(A) -> BlastDoor<B> + UnwindSafe> core::ops::Shr<F>
    for BlastDoor<A>
{
    type Output = BlastDoor<B>;
    #[inline(always)]
    fn shr(self, rhs: F) -> Self::Output {
        self.bind(rhs)
    }
}

// TODO: if specialization or negative traits ever get implemented
/*
impl<A: UnwindSafe, F: FnOnce(A) -> () + RefUnwindSafe> core::ops::Shr<F> for BlastDoor<A> {
    type Output = BlastDoor<()>;
    #[inline(always)]
    fn shr(self, rhs: F) -> Self::Output {
        self.bind(rhs)
    }
}
*/

impl<A: UnwindSafe, B: UnwindSafe> core::ops::BitAnd<BlastDoor<B>> for BlastDoor<A> {
    type Output = BlastDoor<B>;
    #[inline(always)]
    fn bitand(self, rhs: BlastDoor<B>) -> Self::Output {
        rhs
    }
}
