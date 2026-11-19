---
id: pull-pull
kind: doc
target_file: src/pull.rs
line_hint: 89
author: 
created: 
title: "Pull"
tags: []
signature: "pub struct Pull"
---

What asks and where the answers go.

# Examples

```rust
use crawrad_lsp::pull::Pull;

fn assert_sync<T: Sync>() {}
assert_sync::<Pull>();
```
