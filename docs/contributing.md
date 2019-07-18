# Contributing to Nova Renderer

We are always welcome to new contributers. Please read the following before contributing.

### Contact Us

If you want to contribute, we recommend that you drop us a line over on Discord so we can help onboard you properly. 
However, we welcome pull requests from anyone, talking to us beforehand is not required

## Rust Setup

You must have a Rust 3.28+ nightly toolchain setup. You also must have rustfmt installed for that toolchain so the 
automatically installed pre-commit hook works. Cargo will use nightly automatically.

Cargo is used as expected.

```
cargo build --all
cargo test
cargo clippy
```

You do not have to manually call rustfmt, it will be applied when you commit your changes via a precommit hook.

## Project Guidelines

These documents outline the purpose of the project and the rules you must adhere to.

#### Principles

- [The Project Charter](docs/project_charter.md).
- [Project Pipeline](docs/project_pipeline.md). Don't know what to do? Look here!

#### Rules

**These must be followed for your PR to be accepted.**

- [Git Rules/Cheatsheet](docs/git.md).
- [Rust Conventions](docs/rust_conventions.md). 

## Primers

Primers on various new Rust technologies that we will be using in Nova Renderer.

- [Async Await Primer](docs/async_await.md)
