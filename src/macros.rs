//! Simple non-`proc` macros for implementing `Monad` after a definition.

pub use paste::paste;

/// Test the functor laws.
#[macro_export]
macro_rules! test_functor {
    ($name:ident<u64>) => {
        quickcheck::quickcheck! {
            fn prop_functor_identity(fa: $name<u64>) -> bool {
                fa.clone() == fa.fmap(core::convert::identity)
            }
            fn prop_functor_composition(fa: $name<u64>) -> bool {
                use $crate::entropy::hash as g;
                use $crate::entropy::reverse as h;
                fa.clone().fmap(move |a| g(h(a))) == fa.fmap(h).fmap(g)
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
/// assert_eq!(Pointless(4) % u8::is_power_of_two, Pointless(true));
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

                impl<A: Clone, B: Clone, F: FnOnce(A) -> B + Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::Rem<F> for $name<A $(, $($g_ty),+)?> {
                    type Output = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn rem(mut self, mut f: F) -> $name<B $(, $($g_ty),+)?> { self.fmap(f) }
                }

                $crate::test_functor!($name<u64>);
            }
        }
    };
}
pub use functor;

/// Test the Applicative laws.
#[macro_export]
macro_rules! test_applicative {
    ($name:ident<u64>) => {
        quickcheck::quickcheck! {
            fn prop_applicative_fmap(ab: $name<u64>) -> bool {
                use $crate::entropy::hash as f;
                ab.clone().fmap(f) == ab.tie(consume(f))
            }
            fn prop_applicative_identity(ab: $name<u64>) -> bool {
                ab.clone() == ab.tie(consume(core::convert::identity))
            }
            fn prop_applicative_homomorphism(b: u64) -> bool {
                use $crate::entropy::hash as f;
                consume::<$name<_>, _>(f(b)) == consume::<$name<_>, _>(b).tie(consume(f))
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
    ($name:ident<A $(, $($g_ty:ident $(: $g_bound:tt $(+ $g_bounds:tt)*)?),+)?>: fn consume($a:ident) $consume:block fn tie($self:ident, $af:ident) $tie:block) => {
        paste! {
            $crate::prelude::functor! {
                $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>:

                fn fmap(self, f) {
                    self.tie(consume(f))
                }
            }

            mod [<$name:snake _applicative_impl>] {
                #![allow(clippy::arithmetic_side_effects, unused_imports, unused_mut)]
                use $crate::prelude::*;
                use super::*;

                impl<A: Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> Applicative<A> for $name<A $(, $($g_ty),+)?> {
                    type Applicative<B: Clone> = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn consume(mut $a: A) -> Self $consume
                    #[inline(always)] #[must_use] fn tie<F: FnOnce(A) -> B + Clone, B: Clone>(mut $self, mut $af: Self::Applicative<F>) -> Self::Applicative<B> $tie
                }

                impl<A: Clone, B: Clone, F: FnOnce(A) -> B + Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::Mul<$name<F $(, $($g_ty),+)?>> for $name<A $(, $($g_ty),+)?> {
                    type Output = $name<B $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn mul(mut self, mut af: $name<F $(, $($g_ty),+)?>) -> Self::Output { self.tie(af) }
                }

                $crate::test_applicative!($name<u64>);
            }
        }
    };
}
pub use applicative;

/// Test the Alternative laws.
#[macro_export]
macro_rules! test_alternative {
    ($name:ident<u64>) => {
        quickcheck::quickcheck! {
            fn prop_monoid_associativity(ma: $name<u64>, mb: $name<u64>, mc: $name<u64>) -> bool {
                ma.clone().either(|| mb.clone().either(|| mc.clone())) == ma.either(move || mb).either(move || mc)
            }
            fn prop_monoid_left_identity(ma: $name<u64>) -> bool {
                ma.clone() == empty::<$name<u64>, _>().either(move || ma)
            }
            fn prop_monoid_right_identity(ma: $name<u64>) -> bool {
                ma.clone() == ma.either(empty::<$name<u64>, _>)
            }
        }
    };
}
pub use test_alternative;

/// Implement ONLY `Alternative` (not its superclasses) after a definition.
#[macro_export]
macro_rules! just_alternative {
    ($name:ident<A $(, $($g_ty:ident $(: $g_bound:tt $(+ $g_bounds:tt)*)?),+)?>: fn empty() $empty:block fn either($self:ident, $make_other:ident) $either:block) => {
        paste! {
            mod [<$name:snake _alternative_impl>] {
                #![allow(unused_mut)]
                use $crate::prelude::*;
                #[allow(unused_imports)]
                use super::*;

                #[allow(clippy::missing_trait_methods)]
                impl<A: Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> Alternative<A> for $name<A $(, $($g_ty),+)?> {
                    #[inline(always)] #[must_use] fn empty() -> Self $empty
                    #[inline(always)] #[must_use] fn either<F: FnOnce() -> Self>(mut $self, mut $make_other: F) -> Self $either
                }

                impl<F: FnOnce() -> $name<A>, A: Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::BitOr<F> for $name<A $(, $($g_ty),+)?> {
                    type Output = $name<A $(, $($g_ty),+)?>;
                    #[inline(always)] #[must_use] fn bitor(mut self, mut make_other: F) -> $name<A $(, $($g_ty),+)?> { self.either(make_other) }
                }

                test_alternative!($name<u64>);
            }
        }
    }
}

/// Implement `Alternative` (and its superclasses automatically) after a definition.
#[macro_export]
macro_rules! alternative {
    ($name:ident<A $(, $($g_ty:ident $(: $g_bound:tt $(+ $g_bounds:tt)*)?),+)?>: fn empty() $empty:block fn either($self:ident, $make_other:ident) $either:block) => {
        paste! {
            $crate::prelude::applicative! {
                $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>:

                fn consume($a) $consume

                fn tie(self, af) {
                    self.bind(move |a| af.bind(move |f| consume(f(a))))
                }
            }

            $crate::prelude::just_alternative! {
                $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>:

                fn empty() $empty

                fn either($self, $make_other) $either
            }
        }

    };
}

/// Test the monad laws.
#[macro_export]
macro_rules! test_monad {
    ($name:ident<u64>) => {
        quickcheck::quickcheck! {
            fn prop_monad_left_identity(a: u64) -> bool {
                use $crate::entropy::hash_consume as f;
                core::cmp::PartialEq::<$name<u64>>::eq(&consume::<$name<u64>, _>(a).bind(f), &f(a))
            }
            fn prop_monad_right_identity(ma: $name<u64>) -> bool {
                #![allow(clippy::arithmetic_side_effects)]
                ma.clone() == ma.bind(consume)
            }
            fn prop_monad_associativity(ma: $name<u64>) -> bool {
                #![allow(clippy::arithmetic_side_effects)]
                use $crate::entropy::hash_consume as g;
                use $crate::entropy::reverse_consume as h;
                ma.clone().bind(g).bind(h) == (ma.bind(move |a| { let ga: $name<_> = g(a); ga.bind(h) }))
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
/// assert_eq!(Pointless(4) % u8::is_power_of_two, Pointless(true));
/// # }
/// ```
#[macro_export]
macro_rules! monad {
    ($name:ident<A $(, $($g_ty:ident $(: $g_bound:tt $(+ $g_bounds:tt)*)?),+)?>: fn consume($a:ident) $consume:block fn bind($self:ident, $f:ident) $bind:block) => {
        paste! {
            $crate::prelude::applicative! {
                $name<A $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?>:

                fn consume($a) $consume

                fn tie(self, af) {
                    self.bind(move |a| af.bind(move |f| consume(f(a))))
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

                impl<A: Clone $(, $($g_ty $(: $g_bound $(+ $g_bounds)*)?),+)?> core::ops::BitAnd<Self> for $name<A $(, $($g_ty),+)?> {
                    type Output = Self;
                    #[inline(always)] #[must_use] fn bitand(mut self, mut other: Self) -> Self { self.seq(other) }
                }


                test_monad!($name<u64>);
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
            fn prop_monoid_associativity(ma: $name, mb: $name, mc: $name) -> bool {
                ma.clone().combine(mb.clone().combine(mc.clone())) == ma.combine(mb).combine(mc)
            }
            fn prop_monoid_left_identity(ma: $name) -> bool {
                ma.clone() == unit::<$name>().combine(ma)
            }
            fn prop_monoid_right_identity(ma: $name) -> bool {
                ma.clone() == ma.combine(unit())
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

                test_monoid!($name$(<$($g_ty),+>)?);
            }
        }
    }
}
pub use monoid;
