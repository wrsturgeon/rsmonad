# Monads in Rust

Haskell-style, but with `bind` as `>>` instead of `>>=`.
Rust requires `>>=` to be self-modifying and not to return a value.
This of course means that Haskell's `>>` must go by another name, if at all.
Currently `>>` and `return` are not yet implemented and in the design stage.

# Examples

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
