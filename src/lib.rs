//! Haskell-style monads that support `>>=` out of the box with Rust's `>>`.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(warnings)]
#![warn(
    clippy::all,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    clippy::cargo,
    missing_docs,
    rustdoc::all
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::inline_always,
    clippy::pub_use
)]

use same_as::SameAs;

mod maybe;

pub use maybe::*;

#[cfg(feature = "std")]
mod blastdoor;

#[cfg(feature = "std")]
pub use blastdoor::*;

/// Original Haskell definition:
/// ```haskell
/// class Monad m where
/// (>>=)  :: m a -> (a -> m b) -> m b
/// (>>)   :: m a ->       m b  -> m b
/// return :: a -> m a
/// ```
pub trait Monad<A>: SameAs<Self::Constructor<A>> {
    // TODO: once for<T> lands, use it to restrict `Monad` to `for<F: Fn(A) -> B> core::ops::Shr<F>`
    /// In this `impl`, `Self` is really `Self<A>`, but we want to be able to make `Self<B>`.
    type Constructor<B>: Monad<B>;
    /// Mutate internal state with some function.
    fn bind<B, F: Fn(A) -> B>(self, f: F) -> Self::Constructor<B>;
    /// Construct a monad from a value.
    fn consume(a: A) -> Self;
}

#[cfg(feature = "std")]
use core::panic::{RefUnwindSafe, UnwindSafe};
/// Identical to Monad above but with an inductive guarantee of panic-unwind safety.
#[cfg(feature = "std")]
pub trait UnwindMonad<A: UnwindSafe>: SameAs<Self::Constructor<A>> {
    // TODO: once for<T> lands, use it to restrict `Monad` to `for<F: Fn(A) -> B> core::ops::Shr<F>`
    /// In this `impl`, `Self` is really `Self<A>`, but we want to be able to make `Self<B>`.
    type Constructor<B: UnwindSafe>: UnwindMonad<B>;
    /// Mutate internal state with some function.
    fn bind<B: UnwindSafe, F: Fn(A) -> B + RefUnwindSafe>(self, f: F) -> Self::Constructor<B>;
    /// Construct a monad from a value.
    fn consume(a: A) -> Self;
}
