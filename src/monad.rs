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
