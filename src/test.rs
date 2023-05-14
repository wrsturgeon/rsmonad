extern crate self as rsmonad;

use crate::monad;

monad! {
    /// Encodes your mom's box
    pub enum Maybe {
        #[default]
        Nothing,
        Just(A),
    }

    fn bind(self, f) {
        match self {
            Just(a) => f(a.into()).into(),
            Nothing => Nothing,
        }
    }

    fn consume(a) -> Self { Just(a.into()) }
}

monad! {
    /// Encodes jack shit
    pub struct BraceStruct{
        val: A
    }

    fn bind(self, f) { f(self.val.into()).into() }

    fn consume(a) -> Self { Self { val: a.into() } }
}

monad! {
    /// Encodes the answer to life, the universe, and everything
    pub struct TupleStruct(A);

    fn bind(self, f) { f(self.0.into()).into() }

    fn consume(a) -> Self { Self(a.into()) }
}

// TODO: figure out how/whether to derive PartialEq for unions
/*
monad! {
    /// Encodes 42
    pub union UnionTest {
        first: A,
        second: A,
    }
}
*/
