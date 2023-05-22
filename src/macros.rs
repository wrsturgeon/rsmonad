//! Simple non-`proc` macros for implementing `Monad` after a definition.

pub use paste::paste;

/// Test the functor laws.
#[macro_export]
macro_rules! test_functor {
    ($name:ty) => {
        quickcheck::quickcheck! {
            fn prop_identity(fa: $name) -> bool {
                fa.clone() == (fa | core::convert::identity)
            }
            fn prop_composition(fa: $name) -> bool {
                use $crate::entropy::hash as g;
                use $crate::entropy::reverse as h;
                (fa.clone() | (move |a| g(h(a)))) == (fa | h | g)
            }
        }
    };
}
pub use test_functor;

/// Implement `Functor` (and its superclasses automatically) after a definition.
/// ```rust
/// use rsmonad::prelude::*;
///
/// #[derive(Debug, PartialEq)]
/// struct Pointless<A>(A);
///
/// functor! {
///     Pointless<A>:
///
///     fn fmap(self, f) {
///         Pointless(f(self.0))
///     }
/// }
///
/// # fn main() {
/// assert_eq!(Pointless(4) | u8::is_power_of_two, Pointless(true));
/// # }
/// ```
#[macro_export]
macro_rules! functor {
    ($name:ident<A$(: $a_bound:path $(, $a_bounds:path)*)? $(, $($g_ty:ident $(: $g_bound:path $(, $g_bounds:path)*)?),+)?>: fn fmap($self:ident, $f:ident) $fmap:block) => {
        paste! {
            mod [<$name:snake _functor_impl>] {
                #![allow(unused_mut)]
                use $crate::prelude::*;
                #[allow(unused_imports)]
                use super::*;

                impl<A$(: $a_bound $(+ $a_bounds)*)? $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> Functor<A> for $name<A $(, $($g_ty),+)?> {
                    type Functor<B> = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn fmap<B, F: Fn(A) -> B>(mut $self, mut $f: F) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> $fmap
                }

                impl<A$(: $a_bound $(+ $a_bounds)*)?, B, F: Fn(A) -> B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::BitOr<F> for $name<A $(, $($g_ty),+)?> {
                    type Output = $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>;
                    #[inline(always)] #[must_use] fn bitor(mut self, mut f: F) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> { self.fmap(f) }
                }

                $crate::test_functor! { $name<u64> }
            }
        }
    };
}
pub use functor;

/// Test the monad laws.
#[macro_export]
macro_rules! test_monad {
    ($name:ty) => {
        quickcheck::quickcheck! {
            fn prop_left_identity(a: u64) -> bool {
                use $crate::entropy::hash_consume as f;
                consume::<u64, $name>(a).bind(f) == f(a)
            }
            fn prop_right_identity(ma: $name) -> bool {
                #![allow(clippy::arithmetic_side_effects)]
                ma.clone() == (ma >> consume)
            }
            fn prop_associativity(ma: $name) -> bool {
                #![allow(clippy::arithmetic_side_effects)]
                use $crate::entropy::hash_consume as g;
                use $crate::entropy::reverse_consume as h;
                ((ma.clone() >> g) >> h) == (ma.bind(move |a| { let ga: $name = g(a); ga.bind(h) }))
            }
        }
    };
}
pub use test_monad;

/// Implement `Monad` (and its superclasses automatically) after a definition.
/// ```rust
/// use rsmonad::prelude::*;
///
/// #[derive(Debug, PartialEq)]
/// struct Pointless<A>(A);
///
/// monad! {
///     Pointless<A>:
///
///     fn bind(self, f) {
///         f(self.0)
///     }
///
///     fn consume(a) {
///         Pointless(a)
///     }
/// }
///
/// # fn main() {
/// // Monad
/// fn pointless_pow2(x: u8) -> Pointless<bool> { consume(x.is_power_of_two()) }
/// assert_eq!(Pointless(4) >> pointless_pow2, Pointless(true));
///
/// // Functor
/// assert_eq!(Pointless(4) | u8::is_power_of_two, Pointless(true));
/// # }
/// ```
#[macro_export]
macro_rules! monad {
    ($name:ident<A$(: $a_bound:tt $(+ $a_bounds:tt)*)? $(, $($g_ty:ident $(: $g_bound:tt $(+ $g_bounds:tt)*)?),+)?>: fn bind($self:ident, $f:ident) $bind:block fn consume($a:ident) $consume:block) => {
        paste! {
            $crate::prelude::functor! {
                $name<A$(: $a_bound $(, $a_bounds)*)? $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>:

                fn fmap(self, f) {
                    self.bind(move |x| consume(f(x)))
                }
            }

            mod [<$name:snake _monad_impl>] {
                #![allow(unused_mut)]
                use $crate::prelude::*;
                #[allow(unused_imports)]
                use super::*;

                #[allow(clippy::missing_trait_methods)]
                impl<A$(: $a_bound $(, $a_bounds)*)? $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> Monad<A> for $name<A $(, $($g_ty),+)?> {
                    type Monad<B> = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn bind<B, F: Fn(A) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>>(mut $self, mut $f: F) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> $bind
                    #[inline(always)] #[must_use] fn consume(mut $a: A) -> Self $consume
                }

                impl<A$(: $a_bound $(, $a_bounds)*)?, B, F: Fn(A) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::Shr<F> for $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> {
                    type Output = $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>;
                    #[inline(always)] #[must_use] fn shr(mut self, mut f: F) -> $name<B $(, $($g_ty),+)?> { self.bind(f) }
                }

                test_monad! { $name<u64> }
            }
        }
    };
}
pub use monad;

/// Implement `Fold` (and its superclasses automatically) after a definition.
/// ```rust
/// use rsmonad::prelude::*;
///
/// struct Pair<A>(pub [A; 2]);
/// # impl<A> IntoIterator for Pair<A> { type Item = A; type IntoIter = <[A; 2] as IntoIterator>::IntoIter; fn into_iter(self) -> <Self as IntoIterator>::IntoIter { self.0.into_iter() } }
///
/// fold! {
///     Pair<A>:
///
///     type Item = A;
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! fold {
    ($name:ident$(<$($g_ty:ident $(: $g_bound:path $(, $g_bounds:path)*)?),+>)?: type Item = $item:ty;) => {
        paste! {
            mod [<$name:snake _fold_impl>] {
                use $crate::prelude::*;
                #[allow(unused_imports)]
                use super::*;

                #[allow(clippy::missing_trait_methods)]
                impl$(<$($g_ty $(: $g_bound $(+ $g_bounds)*)?),+>)? Fold for $name$(<$($g_ty),+>)? {
                    type Item = $item;
                }
            }
        }
    };
}
pub use fold;

/// Test the monoid laws.
#[macro_export]
macro_rules! test_monoid {
    ($name:ty) => {
        quickcheck::quickcheck! {
            fn prop_associativity(ma: $name, mb: $name, mc: $name) -> bool {
                (ma.clone() + (mb.clone() + mc.clone())) == ((ma + mb) + mc)
            }
            fn prop_left_identity(ma: $name) -> bool {
                ma.clone() == ma + unit()
            }
            fn prop_right_identity(ma: $name) -> bool {
                ma.clone() == ma + unit()
            }
        }
    };
}
pub use test_monoid;

/// Implement `Monoid` (and its superclasses automatically) after a definition.
/// ```rust
/// use rsmonad::prelude::*;
///
/// #[derive(Debug, PartialEq)]
/// struct Summand(pub u8);
///
/// monoid! {
///     Summand:
///
///     fn unit() { Summand(0) }
///
///     fn combine(self, other) { Summand(self.0 + other.0) }
/// }
///
/// # fn main() {
/// # #[cfg(feature = "std")] {
/// // `+` is shorthand for `combine`, even for monoids that don't look like addition
/// assert_eq!(Summand(1).combine(Summand(2)).combine(Summand(3)), Summand(6));
/// assert_eq!(Summand(1) + Summand(2) + Summand(3), Summand(6));
///
/// assert_eq!(list![Summand(1), Summand(2), Summand(3)].unify(), Summand(6));
/// # }
/// # }
/// ```
#[macro_export]
macro_rules! monoid {
    ($name:ident$(<$($g_ty:ident $(: $g_bound:path $(, $g_bounds:path)*)?),+>)?: fn unit() $unit:block fn combine($self:ident, $other:ident) $combine:block) => {
        paste! {
            mod [<$name:snake _monoid_impl>] {
                #![allow(clippy::arithmetic_side_effects, unused_mut)]

                use $crate::prelude::*;
                #[allow(unused_imports)]
                use super::*;

                #[allow(clippy::missing_trait_methods)]
                impl$(<$($g_ty $(: $g_bound $(+ $g_bounds)*)?),+>)? Monoid for $name$(<$($g_ty),+>)? {
                    #[inline(always)] #[must_use] fn unit() -> Self $unit
                    #[inline(always)] #[must_use] fn combine(mut $self, mut $other: Self) -> Self $combine
                }

                impl$(<$($g_ty $(: $g_bound $(+ $g_bounds)*)?),+>)? core::ops::Add<Self> for $name$(<$($g_ty),+>)? {
                    type Output = Self;
                    #[inline(always)] #[must_use] fn add(mut self, mut other: Self) -> Self { self.combine(other) }
                }

                test_monoid! { $name }
            }
        }
    }
}
pub use monoid;
