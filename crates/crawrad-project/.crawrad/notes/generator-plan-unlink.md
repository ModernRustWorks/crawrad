---
id: generator-plan-unlink
kind: doc
target_file: src/generator.rs
line_hint: 262
author: 
created: 
title: "Plan::unlink"
tags: []
signature: "pub fn unlink (& self) -> Option < Transaction >"
---

The document change the pre-pass asks for, or `None` when nothing
expired.

**ONE transaction for all of them**: undo is a single gesture, like the
save that triggered it. One per link would make the user press `Ctrl+Z`
five times to undo one `Ctrl+S`.

It is handed back instead of applied because writing into the document
is the shell's single door, and this module does not have the key. What
[`write`](fn@write) puts on disk does not depend on this being applied — see
[`plan_with_prepass`].

# Examples

```rust
use crawrad_project::generator::Plan;

let plan = Plan {
    saved: None,
    expired: Vec::new(),
    files: Ok(Vec::new()),
};
assert!(plan.unlink().is_none());
```
