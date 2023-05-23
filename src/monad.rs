//! Trait definition.

use crate::prelude::*;

/// Original Haskell definition:
/// ```haskell
/// class Monad m where
/// (>>=)  :: m a -> (a -> m b) -> m b
/// (>>)   :: m a ->       m b  -> m b
/// return :: a -> m a
/// ```
pub trait Monad<A: Clone>: Applicative<A> {
    // TODO: once for<T> lands, use it to restrict `Monad` to `for<F: FnOnce(A) -> MB> core::ops::Shr<F>`

    /// Fucking pain in the ass redundancy. This has to be in this trait to avoid potential spooky action at a distance e.g. by redefining a separate Hkt later.
    type Monad<B: Clone>: Monad<B, Monad<A> = Self>;
    /// Mutate internal state with some function.
    fn bind<B: Clone, F: FnOnce(A) -> Self::Monad<B> + Clone>(self, f: F) -> Self::Monad<B>;
    /// Flatten a nested monad into a single-layer monad.
    #[inline(always)]
    fn join<Flat: Clone, MFlat: Monad<Flat, Monad<MFlat> = Self> + Clone>(self) -> MFlat
    where
        Self: Sized + Monad<MFlat, Monad<Flat> = MFlat>,
    {
        self.bind::<Flat, _>(core::convert::identity::<MFlat>)
    }
}

/// Mutate internal state with some function.
#[inline(always)]
pub fn bind<A: Clone, B: Clone, MA: Monad<A>, F: FnOnce(A) -> MA::Monad<B> + Clone>(
    ma: MA,
    f: F,
) -> MA::Monad<B> {
    ma.bind(f)
}

/// Flatten a nested monad into its enclosing monad.
/// # Use
/// ```rust
/// # #[cfg(feature = "std")] {
/// use rsmonad::prelude::*;
/// let li = List::consume(List::consume(0_u8)); // List<List<u8>>
/// let joined = join(li);                       // -->  List<u8>!
/// assert_eq!(joined, List::consume(0_u8));
/// # }
/// ```
/// Or via the `Monad` method:
/// ```rust
/// # #[cfg(feature = "std")] {
/// # use rsmonad::prelude::*;
/// let li = List::consume(List::consume(0_u8));
/// let joined = li.join();
/// assert_eq!(joined, List::consume(0_u8));
/// # }
/// ```
/// Trippy Haskell signature when defined in terms of `id`:
/// ```haskell
/// join :: m (m a) -> m a
/// join mma = mma >>= id
/// -- >>= :: m a -> (a -> m b) -> m b
/// -- above, a => (m a) and b => a
/// -- so >>= :: m (m a) -> (m a -> m a) -> m a
/// -- and the middle argument is clearly id
/// ```
#[inline(always)]
pub fn join<MMA: Monad<MA, Monad<A> = MA>, MA: Monad<A, Monad<MA> = MMA> + Clone, A: Clone>(
    mma: MMA,
) -> MA {
    mma.join()
}
