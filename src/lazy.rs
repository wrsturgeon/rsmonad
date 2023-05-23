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
