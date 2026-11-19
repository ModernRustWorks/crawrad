---
id: generator-write
kind: doc
target_file: src/generator.rs
line_hint: 299
author: 
created: 
title: "write"
tags: []
signature: "pub fn write (destination : & Path , plan : Plan , pending : & mut dyn PendingEdits) -> String"
---

Write the plan, and hand back the sentence for the status bar.

The second half of [`prepare`], and separate from it for one reason: the
document change the pre-pass asked for goes through the shell in between.
Whoever writes without applying [`Plan::unlink`] still gets coherent files
— the plan never depended on the transaction — and a document one step
behind.

It reloads the editor on success and **only** on success: `Generate` has
just written UNDERNEATH the code tab — it rewrites the whole `.designer.rs`
and grows the unit at the mark — so with either of them on screen the text
became the version from before the generation, and nothing would say so.
What can come back without losing anything comes back on its own; what was
dirty stays, and is named. Reloading over what is being typed would trade a
silent loss for another one.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::generator::{write, Plan};

let plan = Plan {
    saved: None,
    expired: Vec::new(),
    files: Ok(Vec::new()),
};
let res = write(Path::new("."), plan, &mut ());
assert!(!res.is_empty());
```
