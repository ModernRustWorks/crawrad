---
id: process-after-build
kind: doc
target_file: src/process.rs
line_hint: 193
author: 
created: 
title: "after_build"
tags: []
signature: "pub fn after_build (verdict : Result < Outcome , String >) -> AfterBuild"
---

Reads the build verdict the way both shells read it.

The three cases are three different things on screen, and collapsing any two
of them loses information the user needs:

- **compiled** — launch;
- **ran and failed** — cargo explained itself, and [`Outcome::report`] picks
  the line to show (issue #67);
- **never started** — cargo is not on the `PATH`, the directory is gone.
  That is not code that does not compile, and it takes cargo's own words.

# Examples

```rust
use crawrad_os::process::{after_build, AfterBuild, Outcome};

let outcome = Outcome { success: true, ..Default::default() };
assert_eq!(after_build(Ok(outcome)), AfterBuild::Launch);
```
