/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Allow different bind implementations for non-returning (i.e. `()`) functions and those that directly return a value.
//! This way we don't have to explicitly write `Phew(())`, `Nothing`, etc. on the last line.

use crate::Monad;

trait FlipBind<M: Monad<A>, A, B> {
    fn flip_bind<F: FnOnce(A) -> Self>(lhs: M, rhs: F) -> M::Constructor<B>;
}

pub struct Flip<B>(pub core::marker::PhantomData<B>);

impl<B> Flip<B> {
    fn flip_bind<F: FnOnce(A) -> R, R: FlipBind<M, A, B>, M: Monad<A>, A>(
        lhs: M,
        rhs: F,
    ) -> M::Constructor<B> {
        R::flip_bind(lhs, rhs)
    }
}

impl<M: Monad<A>, A, B> FlipBind<M, A, B> for ()
where
    M::Constructor<B>: Default,
{
    fn flip_bind<F: FnOnce(A) -> Self>(lhs: M, rhs: F) -> M::Constructor<B> {
        lhs.bind(move |x| {
            rhs(x);
            Default::default()
        })
    }
}

impl<M: Monad<A>, A, B> FlipBind<M, A, B> for M::Constructor<B> {
    fn flip_bind<F: FnOnce(A) -> Self>(lhs: M, rhs: F) -> M::Constructor<B> {
        lhs.bind(rhs)
    }
}
