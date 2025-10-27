# Rustlox

This is an implementation of the Lox programming language from [Crafting Interpreters](https://craftinginterpreters.com). It's a toy project that I'm using to learn about programming languages & Rust 🤓

## Current progress

I'm currently on chapter ...
I accidentally skipped ahead and did some work on classes. This is on the `wip_classes` branch.

## Debugging

Rustlox uses the [log](https://docs.rs/log/latest/log/) crate for debugging.

Debug messages can be seen by using the `RUST_LOG` env var:

```
RUST_LOG=trace cargo run test.lox
```
