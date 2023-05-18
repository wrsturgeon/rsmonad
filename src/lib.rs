//! Haskell-style monads that support `>>=` out of the box with Rust's `>>`.

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)] // TODO: remove `not(test)` after https://github.com/rust-fuzz/arbitrary/pull/74
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
    clippy::mod_module_files,
    clippy::pub_use
)]

extern crate self as rsmonad;

pub mod prelude {
    //! In general, always import this with `use rsmonad::prelude::*;`.
    pub use rsmonad_macros::*;

    pub use super::monad::Monad;

    pub use super::list::*;
    pub use super::maybe::*;

    #[cfg(feature = "std")]
    pub use super::with_std::*;
}

pub mod monad_laws;

mod list;
mod maybe;
mod monad;

#[cfg(feature = "std")]
mod with_std;

#[cfg(test)]
mod test;
