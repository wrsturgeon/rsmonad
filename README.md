# Monads in Rust

Haskell-style monads with Rust syntax.

## Syntax

Rust requires `>>=` to be self-modifying and not to return a value, so `>>=` becomes `>>` and `return` (keyword) becomes `consume`.
At the moment, Haskell's `>>` seems unnecessary in an eager language like Rust, but I could easily be convinced otherwise! Please let me know if you'd like it implemented.

## Examples

This typechecks, compiles, and runs without a hitch:
```rust
use rsmonad::prelude::*;
fn successor(x: u8) -> Maybe<u8> {
    x.checked_add(1).map_or(Nothing, Just)
}
assert_eq!(
    Just(3) >> successor,
    Just(4),
);
assert_eq!(
    Nothing >> successor,
    Nothing,
);
assert_eq!(
    Just(255) >> successor,
    Nothing,
);
```

This as well, if you choose to enable the non-`no-std` bits:
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

The logic of Haskell lists with the speed of Rust vectors:
```rust
// from the wonderful Haskell docs: https://en.wikibooks.org/wiki/Haskell/Understanding_monads/List
fn bunny(s: &str) -> List<&str> {
    List(vec![s, s, s])
}
assert_eq!(
    List::consume("bunny") >> bunny,
    List(vec!["bunny", "bunny", "bunny"]),
);
assert_eq!(
    List::consume("bunny") >> bunny >> bunny,
    List(vec!["bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny", "bunny"]),
);
```

And even the notoriously tricky `join`-in-terms-of-`bind` with no type annotations necessary:
```rust
let li = List::consume(List::consume(0_u8)); // List<List<u8>>
let joined = join(li);                       // -->  List<u8>!
assert_eq!(joined, List::consume(0_u8));
```

Plus, we automatically derive `QuickCheck::Arbitrary` and property-test the monad laws, inextricable from defining a Monad in the first place.

## Sharp edges

Right now, you can use `>>` as sugar for `bind` only when you have a _concrete instance_ of `Monad` like `Maybe` but not a general `<M: Monad<A>>`.
The latter still works but requires an explicit call to `m.bind(f)` (or, if you don't `use` the trait, `Monad::<A>::bind(m, f)`).
This should be fixed with the Rust's non-lifetime binder feature when it rolls out.
