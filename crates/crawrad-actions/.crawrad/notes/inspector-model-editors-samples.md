---
id: inspector-model-editors-samples
kind: doc
target_file: src/inspector_model/editors.rs
line_hint: 171
author: 
created: 
title: "samples"
tags: []
signature: "pub fn samples () -> Vec < Editor >"
---

One editor of every kind, for the DSL gate.

Public, and not `#[cfg(test)]`: the shell's gates read it across the
crate (a `cfg(test)` item is only seen by this crate's own tests).

The children come flattened in, because a template that only a child names
is a template nobody checks.

# Examples

```rust
use crawrad_actions::inspector_model::editors::samples;

let editors = samples();
assert!(!editors.is_empty());
```
