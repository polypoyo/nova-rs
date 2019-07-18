# Rust Conventions

This document outlines conventions that we use for rust. It is closely tied with the API guidelines
laid out by the [Rust Language Nursery](https://rust-lang-nursery.github.io/api-guidelines/). If
this document does not specify something, then follow that document. That being said:

**THE CLARITY OF THE CODE IS MORE IMPORTANT THAN ANY OF THESE CONVENTIONS!** If breaking the conventions
results in better, cleaner, more intelligible code, do it. 

## Use Statements

Use statements bring other code into your current scope. To aid in comprehension of the code,
you should prefer importing modules over individual things. For example:

```rust
use std::fs;
use std::io;

// use std::fs::File
// use std::io::Error
```

The exception to this rule is if the module is part of your own crate/module. When importing your own
crate, use globs, as you are bound to include many things. However, please use your discretion and if
the modules get too large you should no longer use individual imports and instead import modules.

Do **not** use `super`. Use absolute paths starting from `crate`.

```rust
use crate::module_a::*;
use crate::module_b::*;
use crate::module_c::*;
```

A further exception is that you should always glob-use a crate's prelude. This includes your own.

```rust
use futures::prelude::*;
// std::io::*;
```

### Multi-use

Because there is a limited amount of statements you should always prefer using `use` statements with
no brackets. This aids readability and looks dope.

```rust
use std::fs;
use std::io;

//use std::{
//    fs,
//    io
//};
```

Like all nice rules, there is an exception. If there is a platform specific thing that requires
a specific pattern of imports, please combine those into as few imports as is reasonable, again
favoring modules if at all possible.

```rust
#[cfg(windows)]
use my_lib::windows;
//#[cfg(windows)]
//use my_lib::windows::MicrosoftWindows;

// This is okay because they are doing compile time 
// polymorphism based on the target.
#[cfg(osx)]
use my_lib::osx::GenericThing;
#[cfg(linux)]
use my_lib::linux::GenericThing;
```
