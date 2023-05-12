//! `BlastDoor` monad.

use crate::UnwindMonad;
use core::panic::{RefUnwindSafe, UnwindSafe};

/// Encodes the possibility of panicking.
/// # Use
/// ```rust
/// use rsmonad::*;
/// fn afraid_of_circles(x: u8) { if x == 0 { panic!("aaaaaa!"); } }
/// assert_eq!(Phew(()), Phew(42) >> afraid_of_circles);
/// assert_eq!(Kaboom, Phew(0) >> afraid_of_circles);
/// ```
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    fn bind<B: UnwindSafe, F: Fn(A) -> B + RefUnwindSafe>(self, f: F) -> Self::Constructor<B> {
        if let Phew(x) = self {
            std::panic::catch_unwind(|| f(x)).map_or(Kaboom, Phew)
        } else {
            Kaboom
        }
    }
}

impl<A: UnwindSafe, B: UnwindSafe, F: Fn(A) -> B + RefUnwindSafe> core::ops::Shr<F>
    for BlastDoor<A>
{
    type Output = <Self as UnwindMonad<A>>::Constructor<B>;
    #[inline(always)]
    fn shr(self, rhs: F) -> Self::Output {
        self.bind(rhs)
    }
}
