# Duplicate a value via `Copy` or `Clone`

## Usage:

Derive `dupit::Duplicate` for a type `Foo`
and use `Foo::dup()` to copy or clone.

## Example: duplicate a value via `Clone`

Value is duplicated via `Clone`, if `Copy` is not implemented.

```rust
# fn foo() {
use dupit::Duplicate;

#[derive(Clone, dupit::Duplicate)]
struct Cloneable;

let a = Cloneable.dup();
}

```

## Example: duplicate a value via `Copy`

Value is duplicated via `Copy`, if `Copy` is implemented for it.

```rust
# fn foo() {
use dupit::Duplicate;

#[derive(Clone, Copy, dupit::Duplicate)]
struct Copyable;

let b = Copyable.dup();
# }
```
