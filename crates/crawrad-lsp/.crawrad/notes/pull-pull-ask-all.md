---
id: pull-pull-ask-all
kind: doc
target_file: src/pull.rs
line_hint: 110
author: 
created: 
title: "Pull::ask_all"
tags: []
signature: "pub fn ask_all (self : & Arc < Self >)"
---

Asks again for every open file, at the version last asked.

# Examples

```rust,no_run
// Pull cannot be constructed directly because its transport fields are crate-private.
use crawrad_lsp::pull::Pull;
use std::sync::Arc;

fn demo(pull: &Arc<Pull>) {
    pull.ask_all();
}
```
