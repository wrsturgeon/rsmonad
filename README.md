# Monads, Functors, & More to Come in Rust

Haskell-style monads with Rust syntax.

## Syntax

Rust requires `>>=` to be self-modifying, so we use `>>` instead of `>>=` and `consume` instead of `return` (keyword).
For functors, you can use `fmap(f, x)` or `x.fmap(f)`, or you can _pipe_ it: `x | f | g | ...`.
At the moment, Haskell's monadic `>>` seems unnecessary in an eager language like Rust, but I could easily be overlooking something!

## Use

Just write a `monad! { ...` and you get all its superclasses like `Functor` for free, plus property-based tests of the monad laws:
```rust
use rsmonad::prelude::*;

enum Maybe<A> {
    Just(A),
    Nothing,
}

monad! {
    Maybe<A>:

    fn bind(self, f) {
        match self {
            Just(a) => f(a),
            Nothing => Nothing,
        }
    }

    fn consume(a) {
        Just(a)
    }
}

// And these just work:

// Monad
assert_eq(Just(4) >> |x| u8::checked_add(x, 1).into(), Just(5));
assert_eq(Nothing >> |x| u8::checked_add(x, 1).into(), Nothing);
assert_eq(Just(255) >> |x| u8::checked_add(x, 1).into(), Nothing);

// Functor
assert_eq!(Just(4) | u8::is_power_of_two, Just(true));
assert_eq!(Nothing | u8::is_power_of_two, Nothing);
```

## Examples

The logic of Haskell lists with the speed of Rust vectors:
```rust
use rsmonad::prelude::*;
let li = list![1, 2, 3, 4, 5];
fn and_ten(x: u8) -> List<u8> { list![x, 10 * x] }
assert_eq!(li >> and_ten, list![1, 10, 2, 20, 3, 30, 4, 40, 5, 50]);
```

Catch `panic`s without worrying about the details:
```rust
fn afraid_of_circles(x: u8) -> BlastDoor<()> {
    if x == 0 { panic!("aaaaaa!"); }
    Phew(())
}
assert_eq!(
    Phew(42) >> afraid_of_circles,
    Phew(())
);
assert_eq!(
    Phew(0) >> afraid_of_circles,
    Kaboom,
);
```

_N_-fold bind without type annotations:
```rust
// from the wonderful Haskell docs: https://en.wikibooks.org/wiki/Haskell/Understanding_monads/List
fn bunny(s: &str) -> List<&str> {
    list![s, s, s]
}
assert_eq!(
    list!["bunny"] >> bunny,
    list!["bunny", "bunny", "bunny"],
);
assert_eq!(
    list!["bunny"] >> bunny >> bunny,
    list!["bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny"],
);
```

And even the notoriously tricky `join`-in-terms-of-`bind` with no type annotations necessary:
```rust
let li = list![list![0_u8]]; // List<List<u8>>
let joined = li.join();      // -->  List<u8>!
assert_eq!(joined, list![0]);
```

## Sharp edges

Right now, you can use `>>` for `bind` only when you have a _concrete instance_ of `Monad` like `Maybe` but not a general `<M: Monad<A>>`.
The latter still works but requires an explicit call to `m.bind(f)` (or, if you don't `use` the trait, `Monad::<A>::bind(m, f)`).
This should be fixed with the Rust's non-lifetime binder feature when it rolls out.

### "Cannot find type `...` in this scope" in a doctest

Doctests try to guess where to place a `fn main { ... }` if you don't provide one, and sometimes it reads an `rsmonad` macro as something that should be in a `main` block.
Try adding an explicit `fn main () { ... }` around the statements you want to run.
If you don't want `fn main() { ... }` to show up in documentation but can't fix this error, comment it out:
```rust
/// monad! { ... }
/// # fn main {
/// a();
/// b();
/// c();
/// # }
```

## `#![no_std]`

Disable default features:

```toml
# Cargo.toml

[dependencies]
rsmonad = { version = "*", default-features = false }
```
