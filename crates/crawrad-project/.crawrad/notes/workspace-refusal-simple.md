---
id: workspace-refusal-simple
kind: doc
target_file: src/workspace.rs
line_hint: 1692
author: 
created: 
title: "Refusal::simple"
tags: []
signature: "pub fn simple (phrase : String) -> Self"
---

The refusal that offers nothing.

The refusal that offers nothing: a disk error, a `src/` that could not be created,
a form that could not be saved.

It is the `From<String>` of `?` inside `create_project`: the dozens of
`map_err(|e| format!(…))` already there continue as they were.

# Examples

```rust
use crawrad_project::workspace::Refusal;

let r = Refusal::simple("error".into());
assert_eq!(r.phrase, "error");
```
