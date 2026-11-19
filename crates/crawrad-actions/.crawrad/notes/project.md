---
id: project
kind: doc
target_file: src/lib.rs
line_hint: 179
author: 
created: 
title: "Project"
tags: []
signature: "pub struct Project"
---

What the IDE knows about the open project **outside** the document.

Each field is a fact, not a verdict: *"does the package that `Run` would
run have an executable target?"*, *"does the project have a form a new one
can inherit from?"*, *"is a project open at all?"* and *"where is the
debugger?"*. The Explorer's commands read `open`, not the form: a console
project has a folder and no form. Whoever turns the fact
into a decision is the `enabled` of the action — so the day there is a
`Build` that is valid for a library, it reads the same fact and reaches
another conclusion.

Whoever answers `has_binary` is **`cargo`**, and not a manifest reader of our own: only
it sees the declared `[[bin]]`, the auto-discovered `src/main.rs`, and
`src/bin/*.rs` — and reimplementing that is exactly what ADR 0018 forbids.
The shell queries with `cargo metadata --format-version 1 --no-deps`.

# Examples

```rust
use crawrad_actions::Project;

let p = Project { has_binary: true, ..Project::default() };
assert!(p.has_binary && p.open);
```
