---
id: note-missing-docs
kind: doc
target_file: src/note.rs
line_hint: 1737
author: 
created: 
title: "missing_docs"
tags: []
signature: "pub fn missing_docs (root : & Path) -> Result < Vec < MissingDoc > , String >"
---

Lists undocumented public items in the crate at `root`.

Lists the public items of the crate at `root` that carry neither rustdoc
nor a doc note, as rustc's `missing_docs` lint does. The module tree is
walked from the crate root, so private and `#[cfg(test)]` modules are
never entered.

# Examples

```rust
use std::path::Path;
use crawrad_cli::note::missing_docs;

let res = missing_docs(Path::new("nonexistent_path"));
assert!(res.is_err());
```

# Errors

Returns an error if `Cargo.toml` or the crate entry point cannot be read.
