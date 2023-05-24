/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Lazy evaluation of a single function. Saves one argument for application later.

/*
/// Save an argument for saving for later.
#[inline(always)]
pub fn lazy<F: FnOnce(A) -> B + 'static, A: 'static, B: 'static, R>(
    f: F,
) -> impl Fn(A) -> Box<dyn Fn() -> B> {
    |a| Box::new(|| f(a))
}
*/
