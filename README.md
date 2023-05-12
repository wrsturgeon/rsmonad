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
fn successor(x: u8) -> Option<u8> {
    x.checked_add(1)
}
assert_eq!(
    Just(3) >> successor >> Option::unwrap,
    Just(4),
);
assert_eq!(
    Nothing >> successor >> Option::unwrap,
    Nothing,
);
```

This as well, if you choose to enable the non-`no-std` bits:
```rust
fn afraid_of_circles(x: u8) {
    if x == 0 { panic!("aaaaaa!"); }
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
