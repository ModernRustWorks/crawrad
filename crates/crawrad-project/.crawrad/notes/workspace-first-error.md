---
id: workspace-first-error
kind: doc
target_file: src/workspace.rs
line_hint: 1768
author: 
created: 
title: "first_error"
tags: []
signature: "pub fn first_error (cargo_output : & str) -> String"
---

The first error line from cargo, or a generic phrase when none exists.

Moved out of [`crate::creation::cargo_with_output`] to have its own test (issue #63):
it reads `error` at the start of the line, and colored cargo `error` arrives
as `\x1b[1m\x1b[91merror\x1b[0m:`. When that happened, every creation failure
became `cargo failed` — and [`refusal_new`], which looks for `destination` and
`already exists` in this phrase, lost the *"open it instead"* that makes the button appear.
A specific error became a silent error.

Color is stripped here too, not just in callers, for the reason explained in [`without_color`].

# Examples

```rust
use crawrad_project::workspace::first_error;

assert_eq!(first_error("error: something broke"), "error: something broke");
assert_eq!(first_error("warning: only a warning"), "cargo failed");
```
