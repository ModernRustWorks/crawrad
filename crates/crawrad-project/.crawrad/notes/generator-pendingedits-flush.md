---
id: generator-pendingedits-flush
kind: doc
target_file: src/generator.rs
line_hint: 226
author: 
created: 
title: "PendingEdits::flush"
tags: []
signature: "fn flush (& mut self) -> Result < Option < PathBuf > , String >"
---

Flushes unsaved editor changes to disk.

Write what is on screen to disk. `Ok(Some(path))` when something went
out, `Ok(None)` when there was nothing to write, and `Err` with the
sentence for the status bar when the write was refused.

**Refusing must abort the generation** — see [`prepare`].

# Errors

Returns an error message for the status bar if the write is refused.

# Examples

```rust
use crawrad_project::generator::PendingEdits;

let mut edits: () = ();
assert_eq!(edits.flush(), Ok(None));
```
