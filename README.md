# Monads in Rust

Haskell-style monads with Rust syntax.

## Syntax
Rust requires `>>=` to be self-modifying and not to return a value, so here's the following conversion table:
Haskell &rarr; Rust
`>>=` &rarr; `>>`
`>>` &rarr; `&`
`return` &rarr; `consume`

## Examples

This, remarkably, compiles and runs without a hitch:
```rust
use rsmonad::*;
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
