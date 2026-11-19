---
id: targets-shared-literal-da-peca
kind: doc
target_file: src/targets/shared.rs
line_hint: 324
author: 
created: 
title: "literal_da_peca"
tags: []
signature: "pub fn literal_da_peca (v : & crawrad_ir :: Value) -> Option < String >"
---

The Rust literal of a value that only the box knows how to represent.

**The Rust literal of a value that only the BOX knows how to represent.**

`None` for everything that the targets already knew how to write on their own (text, number,
flag) — the caller tries this first and falls back to its own afterward.

# The hole this closes

A third-party component could declare `kind = "edges"` or
`kind = "color"`, the Object Inspector edited it, the `.crawform` stored it — and the
generated application was born with the field at `Default`. The value died between the
click and `cargo build`, just like the reference died before
18/09/2026.

# Examples

```rust
use crawrad_codegen::targets::shared::literal_da_peca;
use crawrad_ir::Value;

let lit = literal_da_peca(&Value::Bool(true));
assert!(lit.is_none());
```
