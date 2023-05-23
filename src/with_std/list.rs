//! `List` monad.

use crate::{just_alternative, prelude::*};

/// Encodes nondeterminism.
/// # Use
/// ```rust
/// use rsmonad::prelude::*;
/// let li = list![1, 2, 3, 4, 5];
/// fn and_ten(x: u8) -> List<u8> { list![x, 10 * x] }
/// assert_eq!(li >> and_ten, list![1, 10, 2, 20, 3, 30, 4, 40, 5, 50]);
/// ```
/// ```rust
/// use rsmonad::prelude::*;
/// // from the wonderful Haskell docs: https://en.wikibooks.org/wiki/Haskell/Understanding_monads/List
/// fn bunny(s: &str) -> List<&str> {
///     list![s, s, s]
/// }
/// assert_eq!(
///     list!["bunny"] >> bunny,
///     list!["bunny", "bunny", "bunny"],
/// );
/// assert_eq!(
///     list!["bunny"] >> bunny >> bunny,
///     list!["bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny"],
/// );
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, QuickCheck)]
pub struct List<A>(Vec<A>);

/// Initialize an `rsmonad` List.
/// ```rust
/// use rsmonad::prelude::*;
/// let li = list![1, 2, 3, 4, 5];
/// fn and_ten(x: u8) -> List<u8> { list![x, 10 * x] }
/// assert_eq!(li >> and_ten, list![1, 10, 2, 20, 3, 30, 4, 40, 5, 50]);
/// ```
#[macro_export]
macro_rules! list {
    ($($tt:tt)*) => {
        <List<_> as From<Vec<_>>>::from(vec![$($tt)*])
    };
}
pub use list;

impl<A> From<Vec<A>> for List<A> {
    #[inline(always)]
    fn from(value: Vec<A>) -> Self {
        Self(value)
    }
}

impl<T> From<List<T>> for Vec<T> {
    #[inline(always)]
    fn from(value: List<T>) -> Self {
        value.0
    }
}

impl<A> List<A> {
    /// Wraps `Vec::is_empty`. Check if this list is empty (zero elements).
    #[inline(always)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Wraps `Vec::contains`. Check if a value is in this list.
    #[inline(always)]
    pub fn contains(&self, a: &A) -> bool
    where
        A: PartialEq,
    {
        self.0.contains(a)
    }
    /// Wraps `Vec::starts_with`. Check if this list starts with the given sublist.
    #[inline(always)]
    pub fn starts_with(&self, needle: &[A]) -> bool
    where
        A: PartialEq,
    {
        self.0.starts_with(needle)
    }
    /// Wraps `Vec::ends_with`. Check if this list ends with the given sublist.
    #[inline(always)]
    pub fn ends_with(&self, needle: &[A]) -> bool
    where
        A: PartialEq,
    {
        self.0.ends_with(needle)
    }
    // note: `Vec::concat` is just monadic `join`
    /// Wraps `Vec::capacity`.
    #[inline(always)]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    /// Wraps `Vec::reserve`.
    #[inline(always)]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
    /// Wraps `Vec::reserve`.
    #[inline(always)]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional);
    }
    /// Wraps `Vec::truncate`. Keeps the first `n` elements and drops the rest.
    #[inline(always)]
    pub fn truncate(&mut self, n: usize) {
        self.0.truncate(n);
    }
    /// Wraps `Vec::len`. Length of the list.
    #[inline(always)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Wraps `Vec::insert`. Inserts an element at an index, pushing the rest back an index.
    #[inline(always)]
    pub fn insert(&mut self, index: usize, element: A) {
        self.0.insert(index, element);
    }
    /// Wraps `Vec::remove`. Removes the element at an index and returns it, pushing the rest left an index.
    #[inline(always)]
    /// Not `#[must_use]`, but could use.
    pub fn remove(&mut self, index: usize) -> A {
        self.0.remove(index)
    }
    /// Wraps `Vec::swap_remove`. Returns the given index and replaces it with *the last element*, moving it there from the back.
    #[inline(always)]
    pub fn swap_remove(&mut self, index: usize) -> A {
        self.0.swap_remove(index)
    }
    /// Wraps `Vec::retain`. Drops all elements except those satisfying the given predicate.
    #[inline(always)]
    pub fn retain<F: for<'a> core::ops::FnMut(&'a A) -> bool>(&mut self, f: F) {
        self.0.retain(f);
    }
    /// Wraps `Vec::push`. Adds an element at the end of the list.
    #[inline(always)]
    pub fn push(&mut self, a: A) {
        self.0.push(a);
    }
    /// Wraps `Vec::pop`. Removes and returns the last element of the list, if any.
    #[inline(always)]
    pub fn pop(&mut self) -> Maybe<A> {
        self.0.pop().into()
    }
    /// Wraps `Vec::append`. Moves all elements onto the end of this list.
    #[inline(always)]
    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0);
    }
}

impl<A> IntoIterator for List<A> {
    type Item = A;
    type IntoIter = <Vec<A> as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<A> core::ops::AddAssign for List<A> {
    #[inline(always)]
    fn add_assign(&mut self, mut rhs: Self) {
        self.append(&mut rhs);
    }
}

monad! {
    List<A>:

    fn consume(b) {
        list![b]
    }

    fn bind(self, f) {
        List(self.0.bind(move |a| f(a).0))
    }
}

just_alternative! {
    List<A>:

    fn empty() {
        list![]
    }

    fn either(self, make_other) {
        self.0.append(&mut make_other().0);
        self
    }
}

fold! {
    List<A>:

    type Item = A;
}

mod list_monoid_impl {
    #![allow(
        clippy::arithmetic_side_effects,
        clippy::missing_docs_in_private_items,
        clippy::missing_trait_methods,
        clippy::wildcard_imports
    )]
    use super::*;
    impl<A> Monoid for List<A> {
        #[inline(always)]
        #[must_use]
        fn unit() -> Self {
            <Self as From<Vec<_>>>::from(Vec::new())
        }
        #[inline(always)]
        #[must_use]
        fn combine(mut self, mut other: Self) -> Self {
            self.append(&mut other);
            self
        }
    }
    impl<A> core::ops::Add<Self> for List<A> {
        type Output = Self;
        #[inline(always)]
        #[must_use]
        fn add(self, other: Self) -> Self {
            self.combine(other)
        }
    }
    crate::test_monoid! { List<u64> }
}
