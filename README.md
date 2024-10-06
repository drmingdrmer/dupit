# dupit

[![Crates.io](https://img.shields.io/crates/v/dupit.svg)](https://crates.io/crates/dupit)
[![docs.rs](https://docs.rs/dupit/badge.svg)](https://docs.rs/dupit)

Duplicate a value with `Copy` or fallback to `Clone`

## Usage:

Derive `dupit::Duplicate` for a type `Foo`
and use `Foo::dup()` to copy or clone.

## Example: duplicate a value via `Clone`

Value is duplicated via `Clone`, if `Copy` is not implemented.

```rust
use dupit::Duplicate;

#[derive(Clone, dupit::Duplicate)]
struct Cloneable;

let a = Cloneable.dup();
```

## Example: duplicate a value via `Copy`

Value is duplicated via `Copy`, if `Copy` is implemented for it.

```rust
use dupit::Duplicate;

#[derive(Clone, Copy, dupit::Duplicate)]
struct Copyable;

let b = Copyable.dup();
```
