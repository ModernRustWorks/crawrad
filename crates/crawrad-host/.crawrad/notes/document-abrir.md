---
id: document-abrir
kind: doc
target_file: src/document.rs
line_hint: 117
author: 
created: 
title: "abrir"
tags: []
signature: "pub fn abrir (last : Option < PathBuf >) -> Result < Option < Aberto > , String >"
---

[`carregar`] against the real disk, with the order of [`start_list`].

The first command-line argument enters if it does not look like an option
(`--algo`), which is the same snippet as in the `design_canvas` example.

`Ok(None)`: nobody named a form and there is no last project — the
Welcome page's turn (#555), not a failure.

# Examples

```rust,no_run
// Reads forms from disk and inspects process arguments.
use std::path::PathBuf;
use crawrad_host::document::abrir;

let res = abrir(Some(PathBuf::from("nonexistent.crawform")));
assert!(res.is_err());
```

# Errors

Returns `Err` if a named form file cannot be read or parsed.
