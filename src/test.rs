use crate::prelude::*;

monad! {
    /// Encodes your mom's box
    pub enum TestMaybe<A> {
        #[default]
        TestNothing,
        TestJust(A),
    }

    fn bind(self, f) {
        match self {
            TestJust(a) => f(a),
            TestNothing => TestNothing,
        }
    }

    fn consume(a) { TestJust(a) }
}

monad! {
    /// Encodes jack shit
    pub struct BraceStruct<A> {
        val: A
    }

    fn bind(self, f) { f(self.val) }

    fn consume(a) { Self { val: a } }
}

monad! {
    /// Encodes the answer to life, the universe, and everything
    pub struct TupleStruct<A>(A);

    fn bind(self, f) { f(self.0) }

    fn consume(a) { Self(a) }
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
