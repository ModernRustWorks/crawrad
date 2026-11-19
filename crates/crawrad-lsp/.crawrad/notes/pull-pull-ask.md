---
id: pull-pull-ask
kind: doc
target_file: src/pull.rs
line_hint: 104
author: 
created: 
title: "Pull::ask"
tags: []
signature: "pub fn ask (self : & Arc < Self > , file : & Path , version : i32)"
---

Asks for `file`'s diagnostics at `version` on the reader's thread.

Asks for `file`'s diagnostics at `version`; the answer is handled on
the reader's thread.

# Examples

```rust,no_run
// Pull cannot be constructed directly because its transport fields are crate-private.
use crawrad_lsp::pull::Pull;
use std::path::Path;
use std::sync::Arc;

fn demo(pull: &Arc<Pull>) {
    pull.ask(Path::new("src/lib.rs"), 1);
}
```
