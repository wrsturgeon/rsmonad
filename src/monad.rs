//! Trait definition.

use crate::prelude::Functor;

/// Original Haskell definition:
/// ```haskell
/// class Monad m where
/// (>>=)  :: m a -> (a -> m b) -> m b
/// (>>)   :: m a ->       m b  -> m b
/// return :: a -> m a
/// ```
pub trait Monad<A>: Functor<A> {
    // TODO: once for<T> lands, use it to restrict `Monad` to `for<F: Fn(A) -> MB> core::ops::Shr<F>`

    /// Fucking pain in the ass redundancy. This has to be in this trait to avoid potential spooky action at a distance e.g. by redefining a separate Hkt later.
    type Monad<B>: Monad<B, Monad<A> = Self>;
    /// Mutate internal state with some function.
    fn bind<B, F: Fn(A) -> Self::Monad<B>>(self, f: F) -> Self::Monad<B>;
    /// Construct a monad from a value.
    fn consume(a: A) -> Self;
    /// Flatten a nested monad into a single-layer monad.
    #[inline(always)]
    fn join<Flat, MFlat: Monad<Flat, Monad<MFlat> = Self>>(self) -> MFlat
    where
        Self: Sized + Monad<MFlat, Monad<Flat> = MFlat>,
    {
        self.bind::<Flat, _>(core::convert::identity::<MFlat>)
    }
}

/// Mutate internal state with some function.
#[inline(always)]
pub fn bind<A, B, MA: Monad<A>, F: Fn(A) -> MA::Monad<B>>(ma: MA, f: F) -> MA::Monad<B> {
    ma.bind(f)
}

/// Construct a monad from a value.
#[inline(always)]
pub fn consume<A, MA: Monad<A>>(a: A) -> MA {
    MA::consume(a)
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
pub fn join<MMA: Monad<MA, Monad<A> = MA>, MA: Monad<A, Monad<MA> = MMA>, A>(mma: MMA) -> MA {
    mma.join()
}
