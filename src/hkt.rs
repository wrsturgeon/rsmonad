//! Approximation to higher-kinded types.

use same_as::SameAs;

/// Approximation to higher-kinded types.
pub trait Hkt<A>: SameAs<Self::Hkt<A>> {
    /// In this `impl`, `Self` is really `Self<A>`, but we want to be able to make `Self<B>`.
    type Hkt<B>;
}
