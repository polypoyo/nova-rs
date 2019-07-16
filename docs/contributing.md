# Contributing to Nova Renderer

We are always welcome to new contributers. Please read the following before contributing.

### Contact Us

If you want to contribute, it is recommended to drop us a line over on discord so we can help onboard you properly. It is not manditory however.

## Rust Setup

You must have a rust 3.28+ nightly toolchain setup. You also must have rustfmt installed for that toolchain so the automatically installed pre-commit hook works. Cargo will use nightly automatically.

Cargo is used as expected.

```
cargo build --all
cargo test
cargo clippy
```

You do not have to manually call rustfmt, it will be applied when you commit your changes via a precommit hook.

## Project Guidelines

These documents outline the purpose of the project and the rules you must adhear to.

#### Principles

- [The Project Charter](docs/project_charter.md).

#### Rules

**These must be followed for your PR to be accepted.**

- [Git Rules](docs/git.md). 

## Primers

Primers on various new Rust technologies that we will be using in Nova Renderer.

- [Async Await Primer](docs/async_await.md)
