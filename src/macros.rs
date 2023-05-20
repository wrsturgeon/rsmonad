//! Simple non-`proc` macros for implementing `Monad` after a definition.

pub use paste::paste;

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
    ($name:ident<A $(, $($g_ty:ident $(: $g_bound:ident $(+ $g_bounds:ident)*)?),+)?>: fn fmap($self:ident, $f:ident) $fmap:block) => {
        paste! {
            mod [<$name:snake _functor_impl>] {
                use rsmonad::prelude::*;
                #[allow(unused_imports)]
                use super::*;

                impl<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> Functor<A> for $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> {
                    type Functor<B> = $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>;
                    #[inline(always)] #[must_use] fn fmap<B, F: Fn(A) -> B>($self, $f: F) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> $fmap
                }

                impl<A, B, F: Fn(A) -> B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::BitOr<F> for $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> {
                    type Output = $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>;
                    #[inline(always)] #[must_use] fn bitor(self, f: F) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> { self.fmap(f) }
                }
            }
        }
    };
}
pub use functor;

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
    ($name:ident<A $(, $($g_ty:ident $(: $g_bound:ident $(+ $g_bounds:ident)*)?),+)?>: fn bind($self:ident, $f:ident) $bind:block fn consume($a:ident) $consume:block) => {
        paste! {
            rsmonad::prelude::functor! {
                $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>:

                fn fmap(self, f) {
                    self.bind(move |x| consume(f(x)))
                }
            }

            mod [<$name:snake _monad_impl>] {
                use rsmonad::prelude::*;
                #[allow(unused_imports)]
                use super::*;

                #[allow(clippy::missing_trait_methods)]
                impl<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> Monad<A> for $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> {
                    type Monad<B> = $name<B $(, $($g_ty $(: $g_bound $(+ $g_bound2)*)?),+)?>;
                    #[inline(always)] #[must_use] fn bind<B, F: Fn(A) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>>($self, $f: F) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> $bind
                    #[inline(always)] #[must_use] fn consume($a: A) -> Self $consume
                }

                impl<A, B, F: Fn(A) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::Shr<F> for $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> {
                    type Output = $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>;
                    #[inline(always)] #[must_use] fn shr(self, f: F) -> $name<B $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> { self.bind(f) }
                }

                quickcheck::quickcheck! {
                    fn prop_left_identity(a: u64) -> bool {
                        use rsmonad::entropy::hash_consume as f;
                        $name::<u64>::consume(a).bind(f) == f(a)
                    }
                    fn prop_right_identity(ma: $name<u64>) -> bool {
                        #![allow(clippy::arithmetic_side_effects)]
                        ma.clone() == (ma >> consume)
                    }
                    fn prop_associativity(ma: $name<u64>) -> bool {
                        #![allow(clippy::arithmetic_side_effects)]
                        use rsmonad::entropy::hash_consume as g;
                        use rsmonad::entropy::reverse_consume as h;
                        ((ma.clone() >> g) >> h) == (ma.bind(move |a| { let ga: $name<u64> = g(a); ga.bind(h) }))
                    }
                }
            }
        }
    };
}
pub use monad;
