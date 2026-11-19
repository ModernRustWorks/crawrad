---
id: generator-failure-prefix
kind: doc
target_file: src/generator.rs
line_hint: 75
author: 
created: 
title: "FAILURE_PREFIX"
tags: []
signature: "pub const FAILURE_PREFIX : & str = \"generate:\""
---

The beginning of every generation sentence that **failed**.

A constant because it is read somewhere else: `Run` and the double click on
the canvas decide whether to abort by looking at the prefix of the status
bar ([`refused`]). Spelled twice, this is the same family as the shortcut
that disagreed with its own tooltip (ADR 0011) — and the price here is
`Run` launching cargo on top of a generation that never happened.

# Examples

```rust
use crawrad_project::generator::FAILURE_PREFIX;

assert_eq!(FAILURE_PREFIX, "generate:");
```
