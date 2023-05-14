//! `List` monad.

extern crate alloc;
use crate::monad;
use alloc::vec::Vec;

monad! {
    /// Encodes the possibility of failure.
    /// # Use
    /// Similar to Rust's `Option::map`:
    /// ```rust
    /// use rsmonad::*;
    /// // from the wonderful Haskell docs: https://en.wikibooks.org/wiki/Haskell/Understanding_monads/List
    /// fn bunny(s: &str) -> List<&str> {
    ///     List(vec![s, s, s])
    /// }
    /// assert_eq!(
    ///     List::consume("bunny") >> bunny,
    ///     List(vec!["bunny", "bunny", "bunny"]),
    /// );
    /// assert_eq!(
    ///     List::consume("bunny") >> bunny >> bunny,
    ///     List(vec!["bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny"]),
    /// );
    /// ```
    pub struct List(pub Vec<A>);

    fn bind(self, f) {
        let mut v = Vec::new();
        for a in self.0 {
            v.append(&mut f(a.into()).into().0);
        }
        List(v)
    }

    fn consume(a) -> Self {
        Self(alloc::vec![a.into()])
    }
}
