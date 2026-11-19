---
id: edit-resolve-path
kind: doc
target_file: src/edit.rs
line_hint: 72
author: 
created: 
title: "resolve_path"
tags: []
signature: "pub fn resolve_path (args : & Value , active : Option < & std :: path :: Path > ,) -> Result < String , (& 'static str , & 'static str) >"
---

Resolves the target file path from arguments or the active editor path.

The path to operate on: `path` from `args`, else `active`. Refused when
neither was given.

# Examples

```rust
use crawrad_mcp::edit::resolve_path;
use serde_json::json;

let args = json!({ "path": "src/lib.rs" });
let path = resolve_path(&args, None).unwrap();
assert_eq!(path, "src/lib.rs");
```

# Errors

Returns `Err` with `("no-file", ...)` when neither an argument path nor active path is available.
