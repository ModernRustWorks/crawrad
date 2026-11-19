---
id: stale-bindings-editing
kind: doc
target_file: src/lib.rs
line_hint: 215
author: 
created: 
title: "stale_bindings_editing"
tags: []
signature: "pub fn stale_bindings_editing (raiz : & Path , doc : & crawrad_ir :: Document , editing : & dyn Fn (& Path) -> Option < String > ,) -> Vec < Desligamento >"
---

Finds stale bindings using in-memory editor buffer contents.

[`vinculos_vencidos`], judging a file that is open with unsaved edits by
its EDITED text (#59).

# Examples

```rust
use std::path::Path;
use crawrad_codegen::stale_bindings_editing;
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
let vencidos = stale_bindings_editing(Path::new("."), &doc, &|_| None);
assert!(vencidos.is_empty());
```

`editing` returns the text on screen for a path, or `None` to read the
disk. `Generate` writes the editor before it asks (`crawrad-project`'s
`prepare`), so it can read the disk; the Explorer preview cannot write
(it is drawn every frame), and it passes the editor's buffers here so it
judges the same text `Generate` will judge.
