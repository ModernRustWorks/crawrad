---
id: pull-pull-forget
kind: doc
target_file: src/pull.rs
line_hint: 118
author: 
created: 
title: "Pull::forget"
tags: []
signature: "pub fn forget (& self , file : & Path)"
---

`file` closed.

# Examples

```rust,no_run
// Pull cannot be constructed directly because its transport fields are crate-private.
use crawrad_lsp::pull::Pull;
use std::path::Path;

fn demo(pull: &Pull) {
    pull.forget(Path::new("src/lib.rs"));
}
```
