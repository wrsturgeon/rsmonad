//! Haskell-style monads that support `>>=` out of the box with Rust's `>>`.

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)] // TODO: remove `not(test)` after https://github.com/rust-fuzz/arbitrary/pull/74
#![cfg_attr(feature = "nightly", feature(try_trait_v2))]
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
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::implicit_return,
    clippy::inline_always,
    clippy::mod_module_files,
    clippy::pub_use,
    clippy::separated_literal_suffix
)]

extern crate self as rsmonad;

pub mod prelude {
    //! In general, always import this with `use rsmonad::prelude::*;`.
    pub use rsmonad_macros::*;

    pub use super::functor::*;
    pub use super::hkt::*;
    pub use super::monad::*;

    pub use super::hazard::*;
    pub use super::list::*;
    pub use super::maybe::*;

    #[cfg(feature = "std")]
    pub use super::with_std::*;
}

pub mod monad_laws;

mod functor;
mod hkt;
mod monad;

mod hazard;
mod list;
mod maybe;

#[cfg(feature = "std")]
mod with_std;

#[cfg(test)]
mod test;
