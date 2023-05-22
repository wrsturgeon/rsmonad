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
    clippy::separated_literal_suffix,
    clippy::single_char_lifetime_names
)]

pub mod prelude {
    //! In general, always import this with `use rsmonad::prelude::*;`.

    pub use derive_quickcheck::QuickCheck;
    pub use quickcheck::quickcheck;

    pub use super::entropy::*;
    pub use super::macros::*;

    pub use super::fold::*;
    pub use super::functor::*;
    pub use super::monad::*;
    pub use super::monoid::*;

    pub use super::hazard::*;
    pub use super::maybe::*;
    pub use super::prod_u8::*;
    pub use super::sum_u8::*;

    #[cfg(feature = "std")]
    pub use super::with_std::*;
}

mod entropy;
mod macros;

mod fold;
mod functor;
mod monad;
mod monoid;

mod hazard;
mod maybe;
mod prod_u8;
mod sum_u8;

mod orphans;

#[cfg(feature = "std")]
mod with_std;
