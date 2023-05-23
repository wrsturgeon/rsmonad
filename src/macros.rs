//! Simple non-`proc` macros for implementing `Monad` after a definition.

pub use paste::paste;

/// Test the functor laws.
#[macro_export]
macro_rules! test_functor {
    ($name:ident<A>) => {
        quickcheck::quickcheck! {
            fn prop_identity(fa: $name<u64>) -> bool {
                fa.clone() == (fa | core::convert::identity)
            }
            fn prop_composition(fa: $name<u64>) -> bool {
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
    ($name:ident<A $(, $($g_ty:ident $(: $g_bound:path $(, $g_bounds:path)*)?),+)?>: fn fmap($self:ident, $f:ident) $fmap:block) => {
        paste! {
            mod [<$name:snake _functor_impl>] {
                #![allow(unused_imports, unused_mut)]
                use $crate::prelude::*;
                use super::*;

                impl<A: Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> Functor<A> for $name<A $(, $($g_ty),+)?> {
                    type Functor<B: Clone> = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn fmap<B: Clone, F: FnOnce(A) -> B + Clone>(mut $self, mut $f: F) -> $name<B $(, $($g_ty),+)?> $fmap
                }

                impl<A: Clone, B: Clone, F: FnOnce(A) -> B + Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::BitOr<F> for $name<A $(, $($g_ty),+)?> {
                    type Output = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn bitor(mut self, mut f: F) -> $name<B $(, $($g_ty),+)?> { self.fmap(f) }
                }

                $crate::test_functor! { $name<A> }
            }
        }
    };
}
pub use functor;

/// Test the Applicative laws.
#[macro_export]
macro_rules! test_applicative {
    ($name:ident<A>) => {
        quickcheck::quickcheck! {
            fn prop_fmap(ab: $name<u64>) -> bool {
                use $crate::entropy::hash as f;
                (ab.clone() | f) == (ab * consume(f))
            }
            fn prop_identity(ab: $name<u64>) -> bool {
                ab.clone() == (ab * consume(core::convert::identity))
            }
            fn prop_homomorphism(b: u64) -> bool {
                use $crate::entropy::hash as f;
                consume::<$name<_>, _>(f(b)) == (consume::<$name<_>, _>(b) * consume(f))
            }
            /*
            fn prop_interchange(b: u64) -> bool {
                use $crate::entropy::hash as f;
                let af: $name<_> = consume(f);
                (consume::<$name<_>, _>(b) * af) == consume::<$name<_>, _>(move |g: fn(u64) -> u64| g(b)).tie(af)
            }
            */
            /*
            fn prop_composition(b: $name<u64>) -> bool {
                use $crate::entropy::hash as in_f;
                use $crate::entropy::reverse as in_g;
                let f: $name<_> = consume(in_f);
                let g: $name<_> = consume(in_g);
                f.tie(g.tie(b)) == consume::<$name<_>, _>(move |lf: fn(u64) -> u64, lg: fn(u64) -> u64, x: u64| lf(lg(x))).tie(f).tie(g).tie(b)
            }
            */
        }
    };
}
pub use test_applicative;

/// Implement `Applicative` (and its superclasses automatically) after a definition.
#[macro_export]
macro_rules! applicative {
    ($name:ident<A $(, $($g_ty:ident $(: $g_bound:tt $(+ $g_bounds:tt)*)?),+)?>: fn consume($a:ident) $consume:block fn tie($self:ident, $ab:ident) $tie:block) => {
        paste! {
            $crate::prelude::functor! {
                $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>:

                fn fmap(self, f) {
                    self * consume(f)
                }
            }

            mod [<$name:snake _applicative_impl>] {
                #![allow(clippy::arithmetic_side_effects, unused_imports, unused_mut)]
                use $crate::prelude::*;
                use super::*;

                impl<A: Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> Applicative<A> for $name<A $(, $($g_ty),+)?> {
                    type Applicative<B: Clone> = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn consume(mut $a: A) -> Self $consume
                    #[inline(always)] #[must_use] fn tie<B: Clone, C: Clone>(mut $self, mut $ab: Self::Applicative<B>) -> Self::Applicative<C> where A: FnOnce(B) -> C + Clone $tie
                }

                impl<A: Clone, B: Clone, F: FnOnce(A) -> B + Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::Mul<$name<F $(, $($g_ty),+)?>> for $name<A $(, $($g_ty),+)?> {
                    type Output = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn mul(mut self, mut af: $name<F $(, $($g_ty),+)?>) -> Self::Output { af.tie(self) }
                }

                $crate::test_applicative! { $name<A> }
            }
        }
    };
}
pub use applicative;

/// Test the monad laws.
#[macro_export]
macro_rules! test_monad {
    ($name:ident<A>) => {
        quickcheck::quickcheck! {
            fn prop_left_identity(a: u64) -> bool {
                use $crate::entropy::hash_consume as f;
                consume::<$name<u64>, _>(a).bind(f) == f(a)
            }
            fn prop_right_identity(ma: $name<u64>) -> bool {
                #![allow(clippy::arithmetic_side_effects)]
                ma.clone() == (ma >> consume)
            }
            fn prop_associativity(ma: $name<u64>) -> bool {
                #![allow(clippy::arithmetic_side_effects)]
                use $crate::entropy::hash_consume as g;
                use $crate::entropy::reverse_consume as h;
                ((ma.clone() >> g) >> h) == (ma.bind(move |a| { let ga: $name<_> = g(a); ga.bind(h) }))
            }
        }
    };
}
pub use test_monad;

/// Implement `Monad` (and its superclasses automatically) after a definition.
/// ```rust
/// use rsmonad::prelude::*;
///
/// #[derive(Clone, Debug, PartialEq)]
/// struct Pointless<A>(A);
///
/// monad! {
///     Pointless<A>:
///
///     fn consume(a) {
///         Pointless(a)
///     }
///
///     fn bind(self, f) {
///         f(self.0)
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
    ($name:ident<A $(, $($g_ty:ident $(: $g_bound:tt $(+ $g_bounds:tt)*)?),+)?>: fn consume($a:ident) $consume:block fn bind($self:ident, $f:ident) $bind:block) => {
        paste! {
            $crate::prelude::applicative! {
                $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>:

                fn consume($a) $consume

                fn tie(self, ab) {
                    self.bind(move |f| ab.bind(move |b| consume(f(b))))
                }
            }

            mod [<$name:snake _monad_impl>] {
                #![allow(unused_mut)]
                use $crate::prelude::*;
                #[allow(unused_imports)]
                use super::*;

                #[allow(clippy::missing_trait_methods)]
                impl<A: Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> Monad<A> for $name<A $(, $($g_ty),+)?> {
                    type Monad<B: Clone> = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn bind<B: Clone, F: FnOnce(A) -> $name<B $(, $($g_ty),+)?> + Clone>(mut $self, mut $f: F) -> $name<B $(, $($g_ty),+)?> $bind
                }

                impl<A: Clone, B: Clone, F: FnOnce(A) -> $name<B $(, $($g_ty),+)?> + Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::Shr<F> for $name<A $(, $($g_ty),+)?> {
                    type Output = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn shr(mut self, mut f: F) -> $name<B $(, $($g_ty),+)?> { self.bind(f) }
                }

                test_monad! { $name<A> }
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

                test_monoid! { $name$(<$($g_ty),+>)? }
            }
        }
    }
}
pub use monoid;
