//! Trait definition.

use same_as::SameAs;

/// Original Haskell definition:
/// ```haskell
/// class Monad m where
/// (>>=)  :: m a -> (a -> m b) -> m b
/// (>>)   :: m a ->       m b  -> m b
/// return :: a -> m a
/// ```
pub trait Monad<A>: SameAs<Self::M<A>> {
    // TODO: once for<T> lands, use it to restrict `Monad` to `for<F: Fn(A) -> B> core::ops::Shr<F>`

    /// In this `impl`, `Self` is really `Self<A>`, but we want to be able to make `Self<B>`.
    type M<B>: Monad<B>;
    /// Mutate internal state with some function.
    fn bind<B, F: Fn(A) -> Self::M<B>>(self, f: F) -> Self::M<B>;
    /// Construct a monad from a value.
    fn consume(a: A) -> Self::M<A>;
}

/// Flatten a nested monad into its enclosing monad.
/// # Use
/// ```rust
/// use rsmonad::prelude::*;
/// let li = List::consume(List::consume(0_u8)); // List<List<u8>>
/// let joined = join(li);                       // -->  List<u8>!
/// assert_eq!(joined, List::consume(0_u8));
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
#[inline]
pub fn join<M1: Monad<M2, M<A> = M2>, M2: Monad<A>, A>(mma: M1) -> M2 {
    mma.bind::<A, _>(core::convert::identity)
}
